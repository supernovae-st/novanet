# MVP 8 Phase 1: Reasoning Capture

**Created:** 2026-02-19
**Status:** Ready for Implementation
**Effort:** 3-4 hours
**Prerequisites:** MVP 7 ✅ (rig-core migration complete)

---

## Overview

Add `thinking` field capture to AgentTurn events, enabling Claude's extended thinking to be recorded in NDJSON traces and displayed in TUI.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  REASONING CAPTURE ARCHITECTURE                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  YAML Workflow                                                                  │
│  ┌────────────────────┐                                                         │
│  │ agent:             │                                                         │
│  │   extended_thinking│───┐                                                     │
│  │   prompt: "..."    │   │                                                     │
│  └────────────────────┘   │                                                     │
│                           ▼                                                     │
│  ┌────────────────────────────────────────────────────────────┐                │
│  │  RigAgentLoop                                               │                │
│  │  ├── Use streaming API (not prompt())                      │                │
│  │  ├── Extract thinking blocks from stream                   │                │
│  │  └── Emit AgentTurn with metadata.thinking                 │                │
│  └────────────────────────────────────────────────────────────┘                │
│                           │                                                     │
│                           ▼                                                     │
│  ┌────────────────────────────────────────────────────────────┐                │
│  │  EventLog                                                   │                │
│  │  └── AgentTurn { metadata: { thinking: "..." } }           │                │
│  └────────────────────────────────────────────────────────────┘                │
│                           │                                                     │
│              ┌────────────┴────────────┐                                        │
│              ▼                         ▼                                        │
│  ┌──────────────────┐     ┌──────────────────┐                                 │
│  │  NDJSON Trace    │     │  TUI Reasoning   │                                 │
│  │  .nika/traces/   │     │  Panel           │                                 │
│  └──────────────────┘     └──────────────────┘                                 │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Infrastructure Status (80% Complete)

| Component | Status | Location |
|-----------|--------|----------|
| `AgentTurnMetadata.thinking` | ✅ Done | `event/log.rs:45-65` |
| `AgentTurn` event emission | ✅ Done | `event/log.rs:274-289` |
| Serialization tests | ✅ Done | `event/log.rs:989-1034` |
| TUI reasoning panel | ✅ Partial | `tui/panels/reasoning.rs` |
| `extended_thinking` in AgentParams | ❌ Missing | `ast/agent.rs` |
| Streaming API usage | ❌ Missing | `runtime/rig_agent_loop.rs` |

---

## Implementation Batches

### Batch 1: AgentParams Update (No Dependencies)

**File:** `nika-dev/tools/nika/src/ast/agent.rs`

#### Task 1.1: Add extended_thinking field

```rust
// After line 68, add:
/// Enable extended thinking for Claude models
/// Allows model to reason through complex problems before responding
#[serde(default)]
pub extended_thinking: Option<bool>,
```

#### Task 1.2: Add validation

```rust
// In validate() method, add:
if self.extended_thinking == Some(true) {
    if let Some(ref provider) = self.provider {
        if provider != "claude" {
            return Err("extended_thinking only supported for claude provider".to_string());
        }
    }
}
```

#### Task 1.3: Update Default impl

```rust
// In Default::default(), add:
extended_thinking: None,
```

#### Tests (TDD)

```rust
#[test]
fn test_extended_thinking_parses_from_yaml() {
    let yaml = r#"
        prompt: "test"
        extended_thinking: true
    "#;
    let params: AgentParams = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(params.extended_thinking, Some(true));
}

#[test]
fn test_extended_thinking_validates_provider() {
    let params = AgentParams {
        prompt: "test".to_string(),
        extended_thinking: Some(true),
        provider: Some("openai".to_string()),
        ..Default::default()
    };
    assert!(params.validate().is_err());
}

#[test]
fn test_extended_thinking_ok_for_claude() {
    let params = AgentParams {
        prompt: "test".to_string(),
        extended_thinking: Some(true),
        provider: Some("claude".to_string()),
        ..Default::default()
    };
    assert!(params.validate().is_ok());
}
```

---

### Batch 2: RigAgentLoop Streaming (Depends on Batch 1)

**File:** `nika-dev/tools/nika/src/runtime/rig_agent_loop.rs`

#### Task 2.1: Check rig-core streaming support

First verify rig-core v0.31 supports thinking in streaming:

```rust
// Check if rig-core exposes:
// 1. Streaming with thinking parameter
// 2. ContentBlockDelta types for thinking
// 3. Message usage in stream
```

#### Task 2.2: Add thinking extraction helper

```rust
/// Extract thinking blocks from streamed content
fn extract_thinking(content_blocks: &[ContentBlock]) -> Option<String> {
    let thinking: Vec<&str> = content_blocks
        .iter()
        .filter_map(|block| match block {
            ContentBlock::Thinking { thinking } => Some(thinking.as_str()),
            _ => None,
        })
        .collect();

    if thinking.is_empty() {
        None
    } else {
        Some(thinking.join("\n"))
    }
}
```

#### Task 2.3: Update run_claude() for streaming

```rust
pub async fn run_claude(&mut self) -> Result<RigAgentLoopResult, NikaError> {
    let extended_thinking = self.params.extended_thinking.unwrap_or(false);

    // If extended_thinking enabled, use streaming API
    if extended_thinking {
        return self.run_claude_with_thinking().await;
    }

    // Otherwise use existing prompt() method
    // ... existing code ...
}

async fn run_claude_with_thinking(&mut self) -> Result<RigAgentLoopResult, NikaError> {
    // Implementation using streaming API
    // 1. Build streaming request with thinking enabled
    // 2. Collect thinking blocks and response
    // 3. Extract token usage
    // 4. Create metadata WITH thinking
    // 5. Emit AgentTurn event
}
```

#### Task 2.4: Update metadata creation

```rust
// Change from:
let metadata = AgentTurnMetadata::text_only(&response, stop_reason);

// To:
let metadata = AgentTurnMetadata {
    thinking,  // From streaming extraction
    response_text: response,
    input_tokens,
    output_tokens,
    cache_read_tokens: 0,
    stop_reason: stop_reason.to_string(),
};
```

#### Tests (TDD)

```rust
#[tokio::test]
async fn test_run_claude_without_thinking_works() {
    // Existing behavior preserved
}

#[tokio::test]
async fn test_run_claude_with_thinking_extracts_blocks() {
    // Mock streaming response with thinking blocks
    // Verify thinking is captured in metadata
}

#[tokio::test]
async fn test_agent_turn_event_contains_thinking() {
    // Run agent with thinking enabled
    // Verify AgentTurn event has metadata.thinking
}
```

---

### Batch 3: TUI Display (Depends on Batch 2)

**File:** `nika-dev/tools/nika/src/tui/state.rs`

#### Task 3.1: Add thinking to AgentTurnState

```rust
pub struct AgentTurnState {
    pub index: u32,
    pub status: String,
    pub tokens: Option<u32>,
    pub tool_calls: Vec<String>,
    pub thinking: Option<String>,  // NEW
}
```

**File:** `nika-dev/tools/nika/src/tui/panels/reasoning.rs`

#### Task 3.2: Add thinking display section

```rust
fn render_thinking_section(&self, area: Rect, buf: &mut Buffer, thinking: &str) {
    // Header
    let header = Line::from(vec![
        Span::styled("─── ", Style::default().fg(Color::DarkGray)),
        Span::styled("Thinking", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" ───", Style::default().fg(Color::DarkGray)),
    ]);
    buf.set_line(area.x, area.y, &header, area.width);

    // Content (scrollable)
    let content_area = Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: area.height.saturating_sub(1),
    };

    let paragraph = Paragraph::new(thinking)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::ITALIC))
        .wrap(Wrap { trim: true });

    paragraph.render(content_area, buf);
}
```

#### Task 3.3: Update panel layout

Split reasoning panel into:
- Top: Thinking content (collapsible)
- Bottom: Response output

```rust
fn render(&self, area: Rect, buf: &mut Buffer) {
    let thinking = self.get_latest_thinking();

    if let Some(ref thinking) = thinking {
        // Split layout: thinking (40%) + response (60%)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(area);

        self.render_thinking_section(chunks[0], buf, thinking);
        self.render_response_section(chunks[1], buf);
    } else {
        // Full response (no thinking)
        self.render_response_section(area, buf);
    }
}
```

#### Tests

```rust
#[test]
fn test_reasoning_panel_shows_thinking() {
    let mut state = TuiState::default();
    state.agent_turns.push(AgentTurnState {
        thinking: Some("Step 1: Analyze...".to_string()),
        ..Default::default()
    });

    // Verify render includes thinking section
}
```

---

### Batch 4: Integration & Error Handling (Depends on Batch 2, 3)

**File:** `nika-dev/tools/nika/src/error.rs`

#### Task 4.1: Add error variants

```rust
#[error("[NIKA-113] Extended thinking capture failed: {reason}")]
ThinkingCaptureFailed { reason: String },

#[error("[NIKA-114] Extended thinking not supported for provider '{provider}'")]
ThinkingNotSupported { provider: String },
```

**File:** `nika-dev/tools/nika/tests/reasoning_capture_test.rs`

#### Task 4.2: Integration test

```rust
#[tokio::test]
async fn test_full_reasoning_capture_workflow() {
    // 1. Parse workflow with extended_thinking: true
    // 2. Execute with mock Claude response containing thinking
    // 3. Verify NDJSON trace contains thinking
    // 4. Verify TUI state has thinking
}
```

**File:** `nika-dev/tools/nika/examples/v04-reasoning-capture.yaml`

#### Task 4.3: Example workflow

```yaml
# v0.4 Feature Showcase: Reasoning Capture
#
# Demonstrates extended thinking capture from Claude.
# The model's reasoning process is recorded in traces.
#
# Run with: cargo run -- run examples/v04-reasoning-capture.yaml
#
schema: "nika/workflow@0.4"
provider: claude

tasks:
  - id: analyze_with_reasoning
    agent:
      prompt: |
        Analyze why QR codes are effective for marketing.
        Think through this step by step before answering.
      extended_thinking: true
      model: claude-sonnet-4-20250514
      max_turns: 1
    output:
      format: text

  - id: summarize
    use:
      analysis: analyze_with_reasoning
    infer:
      prompt: |
        Summarize this analysis in 2 sentences:
        {{use.analysis}}
      model: claude-haiku-4-20250514
    output:
      format: text

flows:
  - source: analyze_with_reasoning
    target: summarize
```

---

## Dependency Graph

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  BATCH DEPENDENCIES                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Batch 1 ─────────────────┐                                                     │
│  [AgentParams]            │                                                     │
│  (no deps)                │                                                     │
│                           ▼                                                     │
│                    Batch 2 ─────────────────┐                                   │
│                    [RigAgentLoop]           │                                   │
│                    (depends: 1)             │                                   │
│                           │                 │                                   │
│                           ▼                 ▼                                   │
│                    Batch 3          Batch 4                                     │
│                    [TUI]            [Integration]                               │
│                    (depends: 2)     (depends: 2, 3)                             │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Success Criteria

| Criterion | Verification |
|-----------|--------------|
| `extended_thinking: true` parses in YAML | Unit test |
| Validation rejects non-Claude providers | Unit test |
| Streaming API extracts thinking blocks | Unit test |
| `AgentTurn` event contains `metadata.thinking` | Integration test |
| NDJSON trace includes thinking | `cargo run -- trace show` |
| TUI displays thinking section | Manual verification |
| Example workflow runs successfully | `cargo run -- run examples/v04-reasoning-capture.yaml` |

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| rig-core v0.31 doesn't expose thinking in streaming | Medium | High | Use Anthropic SDK directly as fallback |
| Thinking blocks format changes | Low | Medium | Abstract extraction into helper function |
| Large thinking content impacts TUI performance | Low | Low | Add scrolling/truncation |

---

## Fallback Plan

If rig-core v0.31 doesn't support thinking extraction:

1. Add direct Anthropic SDK dependency:
   ```toml
   anthropic = "0.2"
   ```

2. Create dedicated thinking-enabled completion function:
   ```rust
   async fn complete_with_thinking(
       prompt: &str,
       system: Option<&str>,
   ) -> Result<(String, Option<String>, TokenUsage), NikaError>
   ```

3. Use raw HTTP client if SDK doesn't support:
   ```rust
   let response = reqwest::Client::new()
       .post("https://api.anthropic.com/v1/messages")
       .header("x-api-key", api_key)
       .header("anthropic-beta", "extended-thinking-2025-01-24")
       .json(&request)
       .send()
       .await?;
   ```

---

## Estimated Timeline

| Batch | Tasks | Effort | Cumulative |
|-------|-------|--------|------------|
| 1 | AgentParams + tests | 30 min | 30 min |
| 2 | RigAgentLoop streaming | 1.5 hours | 2 hours |
| 3 | TUI display | 45 min | 2.75 hours |
| 4 | Integration + example | 45 min | 3.5 hours |

**Total:** ~3.5 hours
