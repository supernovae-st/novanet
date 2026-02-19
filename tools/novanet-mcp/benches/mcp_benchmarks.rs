//! Performance benchmarks for NovaNet MCP Server
//!
//! Run with: cargo bench
//!
//! Benchmark categories:
//! - Token counting (estimate vs exact)
//! - Query execution (simple vs complex)
//! - Cache operations
//!
//! Metrics targets:
//! - Token estimate: < 100μs
//! - Token exact: < 1ms
//! - Simple query: < 50ms
//! - Schema query: < 100ms
//! - Generate (block): < 500ms
//! - Generate (page): < 2s

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use novanet_mcp::tokens::TokenCounter;
use std::hint::black_box;
use std::time::Duration;

// ═══════════════════════════════════════════════════════════════════════════════
// Token Counting Benchmarks
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_token_estimate(c: &mut Criterion) {
    let counter = TokenCounter::new();

    let texts = [
        ("short", "Hello, world!"),
        (
            "medium",
            "The quick brown fox jumps over the lazy dog. NovaNet is a knowledge graph for content generation across 200+ locales. This is a typical evidence packet.",
        ),
        (
            "long",
            &"Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(100),
        ),
        (
            "cjk",
            "你好世界。これは日本語のテキストです。한국어 텍스트입니다。",
        ),
    ];

    let mut group = c.benchmark_group("token_estimate");
    group.measurement_time(Duration::from_secs(5));

    for (name, text) in texts {
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(BenchmarkId::new("estimate", name), text, |b, text| {
            b.iter(|| counter.estimate(black_box(text)))
        });
    }

    group.finish();
}

fn bench_token_count_exact(c: &mut Criterion) {
    let counter = TokenCounter::new();

    let texts = [
        ("short", "Hello, world!"),
        (
            "medium",
            "The quick brown fox jumps over the lazy dog. NovaNet is a knowledge graph for content generation across 200+ locales.",
        ),
        (
            "long",
            &"Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(50)[..],
        ),
    ];

    let mut group = c.benchmark_group("token_count_exact");
    group.measurement_time(Duration::from_secs(5));

    for (name, text) in texts {
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(BenchmarkId::new("exact", name), text, |b, text| {
            b.iter(|| counter.count(black_box(text)))
        });
    }

    group.finish();
}

fn bench_token_within_budget(c: &mut Criterion) {
    let counter = TokenCounter::new();
    let text = "The quick brown fox jumps over the lazy dog. ".repeat(100);

    let budgets = [100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("token_within_budget");
    group.measurement_time(Duration::from_secs(5));

    for budget in budgets {
        group.bench_with_input(
            BenchmarkId::new("budget", budget),
            &(text.as_str(), budget),
            |b, (text, budget)| {
                b.iter(|| counter.within_budget(black_box(text), black_box(*budget)))
            },
        );
    }

    group.finish();
}

fn bench_token_truncate(c: &mut Criterion) {
    let counter = TokenCounter::new();
    let text = "The quick brown fox jumps over the lazy dog. ".repeat(200);

    let budgets = [50, 100, 500, 1000];

    let mut group = c.benchmark_group("token_truncate");
    group.measurement_time(Duration::from_secs(5));

    for budget in budgets {
        group.bench_with_input(
            BenchmarkId::new("truncate_to", budget),
            &(text.as_str(), budget),
            |b, (text, budget)| {
                b.iter(|| counter.truncate_to_budget(black_box(text), black_box(*budget)))
            },
        );
    }

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
// Cache Benchmarks
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_cache_key_generation(c: &mut Criterion) {
    use novanet_mcp::cache::QueryCache;

    let queries = [
        ("simple", "MATCH (n) RETURN n LIMIT 10", None),
        (
            "with_params",
            "MATCH (n {key: $key}) RETURN n",
            Some({
                let mut map = serde_json::Map::new();
                map.insert("key".to_string(), serde_json::json!("test-entity"));
                map
            }),
        ),
        (
            "complex",
            "MATCH (p:Page)-[:HAS_BLOCK]->(b:Block)-[:USES_ENTITY]->(e:Entity) WHERE p.key = $key RETURN e",
            Some({
                let mut map = serde_json::Map::new();
                map.insert("key".to_string(), serde_json::json!("homepage"));
                map
            }),
        ),
    ];

    let mut group = c.benchmark_group("cache_key_generation");
    group.measurement_time(Duration::from_secs(3));

    for (name, cypher, params) in queries {
        group.bench_with_input(
            BenchmarkId::new("cache_key", name),
            &(cypher, params),
            |b, (cypher, params)| {
                b.iter(|| QueryCache::cache_key(black_box(cypher), black_box(params)))
            },
        );
    }

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
// Prompt Rendering Benchmarks
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_prompt_rendering(c: &mut Criterion) {
    use novanet_mcp::prompts;

    let prompts_with_args: Vec<(&str, serde_json::Map<String, serde_json::Value>)> = vec![
        ("cypher_query", {
            let mut args = serde_json::Map::new();
            args.insert(
                "intent".to_string(),
                serde_json::json!("Find all Entity nodes in org realm"),
            );
            args
        }),
        ("block_generation", {
            let mut args = serde_json::Map::new();
            args.insert("block_key".to_string(), serde_json::json!("hero-section"));
            args.insert("locale".to_string(), serde_json::json!("fr-FR"));
            args
        }),
        ("page_generation", {
            let mut args = serde_json::Map::new();
            args.insert("page_key".to_string(), serde_json::json!("homepage"));
            args.insert("locale".to_string(), serde_json::json!("en-US"));
            args
        }),
        ("locale_briefing", {
            let mut args = serde_json::Map::new();
            args.insert("locale_key".to_string(), serde_json::json!("ja-JP"));
            args
        }),
    ];

    let mut group = c.benchmark_group("prompt_rendering");
    group.measurement_time(Duration::from_secs(3));

    for (name, args) in &prompts_with_args {
        group.bench_with_input(
            BenchmarkId::new("render", *name),
            &(name, args),
            |b, (name, args)| b.iter(|| prompts::render_prompt(black_box(name), black_box(args))),
        );
    }

    group.finish();
}

fn bench_prompt_list(c: &mut Criterion) {
    use novanet_mcp::prompts;

    c.bench_function("list_prompts", |b| b.iter(|| prompts::list_prompts()));
}

// ═══════════════════════════════════════════════════════════════════════════════
// JSON Serialization Benchmarks
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_json_serialization(c: &mut Criterion) {
    // Simulate typical evidence packet
    let evidence = serde_json::json!({
        "source_key": "qr-code-generator",
        "source_kind": "Entity",
        "evidence_type": "definition",
        "distance": 1,
        "relevance": 0.95,
        "content": "QR Code Generator: A powerful tool for creating custom QR codes with various styles and formats.",
        "tokens": 45
    });

    // Simulate evidence array (typical response)
    let evidence_array: Vec<_> = (0..20).map(|i| {
        serde_json::json!({
            "source_key": format!("entity-{}", i),
            "source_kind": "Entity",
            "evidence_type": "definition",
            "distance": i % 3 + 1,
            "relevance": 0.95 - (i as f64 * 0.02),
            "content": format!("Evidence packet {} with typical content for benchmarking purposes.", i),
            "tokens": 30 + i
        })
    }).collect();

    let mut group = c.benchmark_group("json_serialization");
    group.measurement_time(Duration::from_secs(3));

    group.bench_function("single_evidence", |b| {
        b.iter(|| serde_json::to_string(black_box(&evidence)))
    });

    group.bench_function("evidence_array_20", |b| {
        b.iter(|| serde_json::to_string(black_box(&evidence_array)))
    });

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
// Criterion Groups
// ═══════════════════════════════════════════════════════════════════════════════

criterion_group!(
    token_benches,
    bench_token_estimate,
    bench_token_count_exact,
    bench_token_within_budget,
    bench_token_truncate
);

criterion_group!(cache_benches, bench_cache_key_generation);

criterion_group!(prompt_benches, bench_prompt_rendering, bench_prompt_list);

criterion_group!(json_benches, bench_json_serialization);

criterion_main!(token_benches, cache_benches, prompt_benches, json_benches);
