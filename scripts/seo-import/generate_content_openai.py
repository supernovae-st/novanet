#!/usr/bin/env python3
"""
Generate LLM `content` field for SEOKeyword nodes using OpenAI GPT-4o-mini.

Reads content_batch.json and generates semantic context descriptions
in parallel batches (20 keywords per API call, 10 concurrent calls).

Usage:
  python3 scripts/seo-import/generate_content_openai.py
"""

import asyncio
import json
import os
import sys
from pathlib import Path

try:
    from openai import AsyncOpenAI
except ImportError:
    print("ERROR: pip install openai")
    sys.exit(1)

INPUT_PATH = Path(__file__).parent / 'output' / 'content_batch.json'
OUTPUT_PATH = Path(__file__).parent / 'output' / 'content_generated.json'

BATCH_SIZE = 25
MAX_CONCURRENT = 10

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

Output format: Return ONLY a JSON array of objects with "key" and "content" fields. No markdown, no explanation."""


def build_batch_prompt(keywords):
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
    async with semaphore:
        prompt = build_batch_prompt(keywords)

        try:
            response = await client.chat.completions.create(
                model="gpt-4o-mini",
                temperature=0.3,
                max_tokens=4096,
                messages=[
                    {"role": "system", "content": SYSTEM_PROMPT},
                    {"role": "user", "content": prompt}
                ]
            )

            response_text = response.choices[0].message.content

            # Extract JSON from response
            if '```json' in response_text:
                response_text = response_text.split('```json')[1].split('```')[0]
            elif '```' in response_text:
                response_text = response_text.split('```')[1].split('```')[0]

            results = json.loads(response_text.strip())
            tokens_used = response.usage.total_tokens if response.usage else 0

            print(f'  Batch {batch_num}/{total_batches}: {len(results)} contents generated '
                  f'(tokens: {tokens_used})')

            return results

        except json.JSONDecodeError as e:
            print(f'  Batch {batch_num}/{total_batches}: JSON parse error: {e}')
            print(f'  Response preview: {response_text[:200]}...')
            return []
        except Exception as e:
            print(f'  Batch {batch_num}/{total_batches}: Error: {e}')
            return []


async def main():
    api_key = os.environ.get('OPENAI_API_KEY')
    if not api_key:
        print("ERROR: OPENAI_API_KEY environment variable not set")
        sys.exit(1)

    with open(INPUT_PATH) as f:
        all_keywords = json.load(f)

    # Resume support
    existing = {}
    if OUTPUT_PATH.exists():
        with open(OUTPUT_PATH) as f:
            existing_data = json.load(f)
        existing = {item['key']: item['content'] for item in existing_data}
        print(f'Found {len(existing)} existing content entries, resuming...')

    remaining = [kw for kw in all_keywords if kw['key'] not in existing]
    print(f'Total keywords: {len(all_keywords)}')
    print(f'Already processed: {len(existing)}')
    print(f'Remaining: {len(remaining)}')

    if not remaining:
        print('All keywords already processed!')
        return

    batches = [remaining[i:i+BATCH_SIZE] for i in range(0, len(remaining), BATCH_SIZE)]
    total_batches = len(batches)
    print(f'Processing {len(remaining)} keywords in {total_batches} batches '
          f'(batch_size={BATCH_SIZE}, concurrency={MAX_CONCURRENT})')

    client = AsyncOpenAI(api_key=api_key)
    semaphore = asyncio.Semaphore(MAX_CONCURRENT)

    # Process in waves to save progress periodically
    all_results = dict(existing)
    wave_size = 20  # Save every 20 batches

    for wave_start in range(0, len(batches), wave_size):
        wave_batches = batches[wave_start:wave_start + wave_size]
        wave_num = wave_start // wave_size + 1
        total_waves = (len(batches) + wave_size - 1) // wave_size

        print(f'\n--- Wave {wave_num}/{total_waves} ({len(wave_batches)} batches) ---')

        tasks = [
            generate_batch(client, batch, semaphore, wave_start + i + 1, total_batches)
            for i, batch in enumerate(wave_batches)
        ]

        batch_results = await asyncio.gather(*tasks)

        for results in batch_results:
            for item in results:
                if 'key' in item and 'content' in item:
                    all_results[item['key']] = item['content']

        # Save progress after each wave
        output_data = [{'key': k, 'content': v} for k, v in all_results.items()]
        OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
        OUTPUT_PATH.write_text(json.dumps(output_data, indent=2, ensure_ascii=False), encoding='utf-8')
        print(f'  Progress saved: {len(all_results)}/{len(all_keywords)} '
              f'({len(all_results)/len(all_keywords)*100:.1f}%)')

    print(f'\nDone! Wrote {len(all_results)} content entries to {OUTPUT_PATH}')
    print(f'Coverage: {len(all_results)}/{len(all_keywords)} ({len(all_results)/len(all_keywords)*100:.1f}%)')


if __name__ == '__main__':
    asyncio.run(main())
