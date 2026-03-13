#!/usr/bin/env python3
"""
Generate LLM `content` field for SEOKeyword nodes using Claude Sonnet.

Reads content_batch.json and generates semantic context descriptions
in parallel batches (10 keywords per API call, 5 concurrent calls).

Usage:
  python3 scripts/seo-import/generate_content.py
"""

import asyncio
import json
import os
import sys
from pathlib import Path

try:
    import anthropic
except ImportError:
    print("ERROR: pip install anthropic")
    sys.exit(1)

INPUT_PATH = Path(__file__).parent / 'output' / 'content_batch.json'
OUTPUT_PATH = Path(__file__).parent / 'output' / 'content_generated.json'

# Batch size: how many keywords per API call
BATCH_SIZE = 20
# Concurrency: how many API calls in parallel
MAX_CONCURRENT = 5

SYSTEM_PROMPT = """You are a SEO strategist writing concise semantic context descriptions for keyword nodes in a knowledge graph.

For each keyword, write a 1-2 sentence `content` field that describes:
- The search intent and user motivation behind this keyword
- The SEO strategic opportunity (based on volume, difficulty, trend)
- What type of content would best serve this query

Rules:
- Write in English regardless of the keyword language
- Be factual and analytical, not promotional
- Reference the specific keyword naturally
- Include intent signals (informational, transactional, navigational)
- Never invent metrics — only reference data provided
- Keep each description between 40-80 words
- The content field is for LLM context, NOT for end users

Output format: Return a JSON array of objects with "key" and "content" fields only."""


def build_batch_prompt(keywords):
    """Build prompt for a batch of keywords."""
    kw_list = []
    for kw in keywords:
        parts = [f'keyword: "{kw["value"]}"']
        parts.append(f'locale: {kw["locale_key"]}')
        if kw.get('volume'):
            parts.append(f'volume: {kw["volume"]:,}/mo')
        if kw.get('difficulty') is not None:
            parts.append(f'difficulty: {kw["difficulty"]}/100')
        if kw.get('intent'):
            parts.append(f'intent: {kw["intent"]}')
        if kw.get('traffic_potential'):
            parts.append(f'traffic_potential: {kw["traffic_potential"]:,}')
        if kw.get('trend'):
            parts.append(f'trend: {kw["trend"]}')
        kw_list.append(f'- key: "{kw["key"]}" | {" | ".join(parts)}')

    return f"""Generate the `content` field for these {len(keywords)} SEO keywords:

{chr(10).join(kw_list)}

Return a JSON array with {len(keywords)} objects, each having "key" and "content" fields."""


async def generate_batch(client, keywords, semaphore, batch_num, total_batches):
    """Generate content for a batch of keywords."""
    async with semaphore:
        prompt = build_batch_prompt(keywords)

        try:
            message = await asyncio.to_thread(
                client.messages.create,
                model="claude-sonnet-4-20250514",
                max_tokens=4096,
                system=SYSTEM_PROMPT,
                messages=[{"role": "user", "content": prompt}]
            )

            response_text = message.content[0].text

            # Extract JSON from response (handle markdown code blocks)
            if '```json' in response_text:
                response_text = response_text.split('```json')[1].split('```')[0]
            elif '```' in response_text:
                response_text = response_text.split('```')[1].split('```')[0]

            results = json.loads(response_text.strip())

            print(f'  Batch {batch_num}/{total_batches}: {len(results)} contents generated '
                  f'(tokens: {message.usage.input_tokens}+{message.usage.output_tokens})')

            return results

        except json.JSONDecodeError as e:
            print(f'  Batch {batch_num}/{total_batches}: JSON parse error: {e}')
            print(f'  Response: {response_text[:200]}...')
            return []
        except Exception as e:
            print(f'  Batch {batch_num}/{total_batches}: Error: {e}')
            return []


async def main():
    # Check API key
    api_key = os.environ.get('ANTHROPIC_API_KEY')
    if not api_key:
        print("ERROR: ANTHROPIC_API_KEY environment variable not set")
        sys.exit(1)

    # Load keywords
    with open(INPUT_PATH) as f:
        all_keywords = json.load(f)

    # Check for existing progress
    existing = {}
    if OUTPUT_PATH.exists():
        with open(OUTPUT_PATH) as f:
            existing_data = json.load(f)
        existing = {item['key']: item['content'] for item in existing_data}
        print(f'Found {len(existing)} existing content entries, resuming...')

    # Filter out already-processed keywords
    remaining = [kw for kw in all_keywords if kw['key'] not in existing]
    print(f'Total keywords: {len(all_keywords)}')
    print(f'Already processed: {len(existing)}')
    print(f'Remaining: {len(remaining)}')

    if not remaining:
        print('All keywords already processed!')
        return

    # Create batches
    batches = [remaining[i:i+BATCH_SIZE] for i in range(0, len(remaining), BATCH_SIZE)]
    total_batches = len(batches)
    print(f'Processing {len(remaining)} keywords in {total_batches} batches '
          f'(batch_size={BATCH_SIZE}, concurrency={MAX_CONCURRENT})')

    client = anthropic.Anthropic(api_key=api_key)
    semaphore = asyncio.Semaphore(MAX_CONCURRENT)

    # Process all batches
    all_results = list(existing.items())  # Start with existing
    tasks = [
        generate_batch(client, batch, semaphore, i+1, total_batches)
        for i, batch in enumerate(batches)
    ]

    batch_results = await asyncio.gather(*tasks)

    for results in batch_results:
        for item in results:
            if 'key' in item and 'content' in item:
                all_results.append((item['key'], item['content']))

    # Write output
    output_data = [{'key': k, 'content': v} for k, v in all_results]
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(output_data, indent=2, ensure_ascii=False), encoding='utf-8')

    print(f'\nWrote {len(output_data)} content entries to {OUTPUT_PATH}')

    # Stats
    total_input = sum(1 for kw in all_keywords if any(r[0] == kw['key'] for r in all_results))
    print(f'Coverage: {total_input}/{len(all_keywords)} ({total_input/len(all_keywords)*100:.1f}%)')


if __name__ == '__main__':
    asyncio.run(main())
