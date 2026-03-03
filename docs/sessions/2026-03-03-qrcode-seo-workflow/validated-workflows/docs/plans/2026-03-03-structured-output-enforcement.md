# Structured Output Enforcement Implementation Plan

**Goal**: Implement 3-layer structured output validation ensuring LLM outputs match expected schemas, with retry mechanism on validation failure.

**Architecture**: Option C Hybrid
- Layer 1: LLM-side enforcement via DynamicSubmitTool (JSON Schema in tool definition)
- Layer 2: Code-side validation via jsonschema crate (belt + suspenders)
- Layer 3: Retry loop with error feedback (up to max_retries attempts)

**Tech Stack**:
- jsonschema = "0.26" (already in Cargo.toml)
- serde-saphyr = "0.0.20" (YAML parsing)
- rig-core patterns (ToolDyn, SubmitTool concept)

**Success Criteria**: 16/16 workflow tasks pass + 4 new complex workflows

---

## Phase 1: Schema Infrastructure (AST Layer)

### Task 1.1: Add SchemaRef enum to ast/output.rs

**File**: `tools/nika/src/ast/output.rs`

**Test first** (add to existing test module):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_schema_ref_inline_deserialize() {
        let yaml = r#"
output:
  format: json
  schema:
    type: object
    properties:
      name:
        type: string
    required:
      - name
"#;
        let policy: OutputPolicy = serde_saphyr::from_str(yaml).unwrap();
        assert!(matches!(policy.schema, Some(SchemaRef::Inline(_))));
    }

    #[test]
    fn test_schema_ref_file_deserialize() {
        let yaml = r#"
output:
  format: json
  schema: "schemas/keyword.schema.json"
"#;
        let policy: OutputPolicy = serde_saphyr::from_str(yaml).unwrap();
        assert!(matches!(policy.schema, Some(SchemaRef::File(_))));
    }

    #[test]
    fn test_output_policy_max_retries() {
        let yaml = r#"
output:
  format: json
  max_retries: 3
"#;
        let policy: OutputPolicy = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(policy.max_retries, Some(3));
    }
}
```

**Run test** (expect FAIL):
```bash
cargo test test_schema_ref --package nika
```

**Implementation**:
```rust
use serde::de::{self, Deserializer, Visitor};
use serde_json::Value as JsonValue;

/// Reference to a JSON Schema - either inline or file path
#[derive(Debug, Clone)]
pub enum SchemaRef {
    /// Inline JSON Schema object
    Inline(JsonValue),
    /// Path to JSON Schema file
    File(String),
}

impl<'de> Deserialize<'de> for SchemaRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SchemaRefVisitor;

        impl<'de> Visitor<'de> for SchemaRefVisitor {
            type Value = SchemaRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a JSON Schema object or a file path string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SchemaRef::File(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SchemaRef::File(v))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let value = JsonValue::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(SchemaRef::Inline(value))
            }
        }

        deserializer.deserialize_any(SchemaRefVisitor)
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct OutputPolicy {
    #[serde(default)]
    pub format: OutputFormat,

    /// JSON Schema for output validation (inline object or file path)
    #[serde(default)]
    pub schema: Option<SchemaRef>,

    /// Maximum retry attempts on validation failure (default: 2)
    #[serde(default)]
    pub max_retries: Option<u8>,

    /// Save output to file path
    #[serde(default)]
    pub save: Option<String>,
}
```

**Verify test passes**:
```bash
cargo test test_schema_ref --package nika
```

**Commit**:
```bash
git add tools/nika/src/ast/output.rs
git commit -m "feat(nika): add SchemaRef enum for inline/file schema support"
```

---

### Task 1.2: Update runtime/output.rs to handle SchemaRef

**File**: `tools/nika/src/runtime/output.rs`

**Test first**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_validate_inline_schema_success() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "required": ["name"]
        });
        let value = json!({"name": "test"});
        let result = validate_schema_ref(&value, &SchemaRef::Inline(schema)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_inline_schema_failure() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "required": ["name"]
        });
        let value = json!({"other": "field"});
        let result = validate_schema_ref(&value, &SchemaRef::Inline(schema)).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("required"));
    }
}
```

**Run test** (expect FAIL):
```bash
cargo test test_validate_inline_schema --package nika
```

**Implementation** (add to runtime/output.rs):
```rust
use crate::ast::output::SchemaRef;

/// Validate a JSON value against a SchemaRef (inline or file)
pub async fn validate_schema_ref(value: &Value, schema_ref: &SchemaRef) -> Result<(), NikaError> {
    match schema_ref {
        SchemaRef::File(path) => validate_schema(value, path).await,
        SchemaRef::Inline(schema) => validate_inline_schema(value, schema),
    }
}

/// Validate against an inline JSON Schema
fn validate_inline_schema(value: &Value, schema: &Value) -> Result<(), NikaError> {
    let compiled = jsonschema::validator_for(schema).map_err(|e| {
        NikaError::SchemaValidationFailed(format!("Invalid schema: {e}"))
    })?;

    let errors: Vec<_> = compiled.iter_errors(value).collect();
    if errors.is_empty() {
        Ok(())
    } else {
        let error_messages: Vec<String> = errors
            .iter()
            .map(|e| format!("- {}: {}", e.instance_path, e))
            .collect();
        Err(NikaError::SchemaValidationFailed(format!(
            "Output validation failed:\n{}",
            error_messages.join("\n")
        )))
    }
}

/// Extract validation error details for retry feedback
pub fn format_validation_errors(value: &Value, schema: &Value) -> String {
    let compiled = match jsonschema::validator_for(schema) {
        Ok(c) => c,
        Err(e) => return format!("Invalid schema: {e}"),
    };

    let errors: Vec<_> = compiled.iter_errors(value).collect();
    if errors.is_empty() {
        return "No validation errors".to_string();
    }

    errors
        .iter()
        .map(|e| {
            format!(
                "- Path '{}': {} (got: {})",
                e.instance_path,
                e,
                serde_json::to_string(e.instance).unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
```

**Verify test passes**:
```bash
cargo test test_validate_inline_schema --package nika
```

**Commit**:
```bash
git add tools/nika/src/runtime/output.rs
git commit -m "feat(nika): add validate_schema_ref for inline schema validation"
```

---

## Phase 2: DynamicSubmitTool (LLM-side Enforcement)

### Task 2.1: Create DynamicSubmitTool struct

**File**: `tools/nika/src/tools/submit_tool.rs` (new file)

**Test first**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_submit_tool_definition() {
        let schema = json!({
            "type": "object",
            "properties": {
                "keywords": {
                    "type": "array",
                    "items": { "type": "string" }
                }
            },
            "required": ["keywords"]
        });

        let tool = DynamicSubmitTool::new(schema.clone());
        let def = tool.definition();

        assert_eq!(def.name, "submit");
        assert_eq!(def.parameters, schema);
    }

    #[tokio::test]
    async fn test_submit_tool_call_valid() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "required": ["name"]
        });

        let tool = DynamicSubmitTool::new(schema);
        let input = json!({"name": "test"});
        let result = tool.call(input.clone()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }

    #[tokio::test]
    async fn test_submit_tool_call_invalid() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "required": ["name"]
        });

        let tool = DynamicSubmitTool::new(schema);
        let input = json!({"wrong": "field"});
        let result = tool.call(input).await;

        assert!(result.is_err());
    }
}
```

**Implementation**:
```rust
//! Dynamic submit tool for structured output enforcement
//!
//! Based on rig's Extractor pattern - injects JSON Schema as tool definition
//! so LLM understands expected output format.

use serde_json::Value;
use crate::error::NikaError;
use crate::runtime::output::validate_inline_schema;

/// Tool definition for provider-agnostic tool calling
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

/// Dynamic submit tool that enforces structured output via tool calling
///
/// When added to a chat completion request, the LLM is instructed to "submit"
/// its response using this tool, which includes the expected JSON Schema.
#[derive(Debug, Clone)]
pub struct DynamicSubmitTool {
    schema: Value,
    description: Option<String>,
}

impl DynamicSubmitTool {
    /// Create a new submit tool with the given JSON Schema
    pub fn new(schema: Value) -> Self {
        Self {
            schema,
            description: None,
        }
    }

    /// Set a custom description for the tool
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Get the tool definition for the LLM
    pub fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "submit".to_string(),
            description: self.description.clone().unwrap_or_else(|| {
                "Submit your response in the required structured format. \
                 Use this tool to provide your final answer.".to_string()
            }),
            parameters: self.schema.clone(),
        }
    }

    /// Validate and return the submitted data
    pub async fn call(&self, input: Value) -> Result<Value, NikaError> {
        // Validate against schema
        validate_inline_schema(&input, &self.schema)?;
        Ok(input)
    }

    /// Get the schema for error feedback
    pub fn schema(&self) -> &Value {
        &self.schema
    }
}

/// Convert to provider-specific tool format (Claude)
impl DynamicSubmitTool {
    pub fn to_claude_tool(&self) -> Value {
        serde_json::json!({
            "name": "submit",
            "description": self.definition().description,
            "input_schema": self.schema
        })
    }

    pub fn to_openai_tool(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "submit",
                "description": self.definition().description,
                "parameters": self.schema
            }
        })
    }
}
```

**Add to mod.rs**:
```rust
// In tools/nika/src/tools/mod.rs
pub mod submit_tool;
pub use submit_tool::DynamicSubmitTool;
```

**Verify tests pass**:
```bash
cargo test submit_tool --package nika
```

**Commit**:
```bash
git add tools/nika/src/tools/submit_tool.rs tools/nika/src/tools/mod.rs
git commit -m "feat(nika): add DynamicSubmitTool for LLM-side schema enforcement"
```

---

## Phase 3: Retry Loop in Executor

### Task 3.1: Add retry logic to run_infer

**File**: `tools/nika/src/runtime/executor.rs`

**Test first** (integration test):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_infer_with_schema_validation_success() {
        // Test that valid output passes schema validation
        let output_policy = OutputPolicy {
            format: OutputFormat::Json,
            schema: Some(SchemaRef::Inline(json!({
                "type": "object",
                "properties": {
                    "result": { "type": "string" }
                },
                "required": ["result"]
            }))),
            max_retries: Some(2),
            save: None,
        };

        // Mock: LLM returns valid JSON
        let llm_output = r#"{"result": "success"}"#;
        let validated = validate_infer_output(llm_output, &output_policy).await;
        assert!(validated.is_ok());
    }

    #[tokio::test]
    async fn test_infer_with_schema_validation_failure() {
        let output_policy = OutputPolicy {
            format: OutputFormat::Json,
            schema: Some(SchemaRef::Inline(json!({
                "type": "object",
                "properties": {
                    "result": { "type": "string" }
                },
                "required": ["result"]
            }))),
            max_retries: Some(0),
            save: None,
        };

        // Mock: LLM returns invalid JSON (missing required field)
        let llm_output = r#"{"other": "value"}"#;
        let validated = validate_infer_output(llm_output, &output_policy).await;
        assert!(validated.is_err());
    }
}
```

**Implementation** (modify run_infer in executor.rs):
```rust
/// Validate infer output against OutputPolicy schema
async fn validate_infer_output(
    output: &str,
    policy: &OutputPolicy,
) -> Result<Value, NikaError> {
    // Extract JSON from output
    let value = extract_json_from_output(output).map_err(|e| {
        NikaError::OutputValidation(format!("Failed to extract JSON: {e}"))
    })?;

    // Validate against schema if present
    if let Some(schema_ref) = &policy.schema {
        validate_schema_ref(&value, schema_ref).await?;
    }

    Ok(value)
}

/// Build retry feedback message for LLM
fn build_retry_feedback(
    attempt: u8,
    max_retries: u8,
    error: &NikaError,
    schema: &Value,
) -> String {
    format!(
        "Your previous response did not match the required schema. \
         Attempt {}/{}.

Error details:
{}

Required schema:
{}

Please try again, ensuring your response matches the schema exactly.",
        attempt,
        max_retries + 1,
        error,
        serde_json::to_string_pretty(schema).unwrap_or_default()
    )
}

/// Run infer with schema validation and retry loop
async fn run_infer(
    &self,
    task_id: &Arc<str>,
    infer: &InferParams,
    bindings: &ResolvedBindings,
    datastore: &DataStore,
) -> Result<String, NikaError> {
    let max_retries = infer.output.as_ref()
        .and_then(|o| o.max_retries)
        .unwrap_or(2);

    let schema = infer.output.as_ref()
        .and_then(|o| o.schema.as_ref())
        .and_then(|s| match s {
            SchemaRef::Inline(v) => Some(v.clone()),
            SchemaRef::File(_) => None, // File schemas loaded separately
        });

    let mut messages = self.build_infer_messages(infer, bindings, datastore).await?;
    let mut last_error = None;

    for attempt in 0..=max_retries {
        // Add submit tool if schema present
        let tools = if let Some(ref s) = schema {
            let submit = DynamicSubmitTool::new(s.clone());
            Some(vec![submit.to_claude_tool()])
        } else {
            None
        };

        // Call provider
        let output = self.call_provider(&messages, tools.as_ref()).await?;

        // Validate if schema present
        if let Some(ref policy) = infer.output {
            if policy.schema.is_some() {
                match validate_infer_output(&output, policy).await {
                    Ok(value) => {
                        // Success - return validated JSON
                        return Ok(serde_json::to_string(&value)?);
                    }
                    Err(e) => {
                        if attempt < max_retries {
                            // Add retry feedback and continue
                            let feedback = build_retry_feedback(
                                attempt + 1,
                                max_retries,
                                &e,
                                schema.as_ref().unwrap(),
                            );
                            messages.push(Message::assistant(&output));
                            messages.push(Message::user(&feedback));
                            last_error = Some(e);
                            continue;
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
        }

        // No schema validation needed
        return Ok(output);
    }

    Err(last_error.unwrap_or_else(|| NikaError::InferFailed("Unknown error".into())))
}
```

**Verify tests pass**:
```bash
cargo test test_infer_with_schema --package nika
```

**Commit**:
```bash
git add tools/nika/src/runtime/executor.rs
git commit -m "feat(nika): add retry loop with schema validation in run_infer"
```

---

## Phase 4: Update JSON Schema

### Task 4.1: Update nika-workflow.schema.json

**File**: `tools/nika/schemas/nika-workflow.schema.json`

**Changes to OutputPolicy**:
```json
"OutputPolicy": {
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "format": {
      "type": "string",
      "enum": ["json", "yaml", "text", "markdown"],
      "default": "text"
    },
    "schema": {
      "oneOf": [
        {
          "type": "string",
          "description": "Path to JSON Schema file"
        },
        {
          "type": "object",
          "description": "Inline JSON Schema object"
        }
      ],
      "description": "JSON Schema for output validation (file path or inline object)"
    },
    "max_retries": {
      "type": "integer",
      "minimum": 0,
      "maximum": 5,
      "default": 2,
      "description": "Maximum retry attempts on validation failure"
    },
    "save": {
      "type": "string",
      "description": "File path to save output"
    }
  }
}
```

**Verify schema validity**:
```bash
cargo run -- schema validate
```

**Commit**:
```bash
git add tools/nika/schemas/nika-workflow.schema.json
git commit -m "feat(nika): update OutputPolicy schema for inline schemas and max_retries"
```

---

## Phase 5: Integration Testing

### Task 5.1: Update workflow 06 with inline schema

**File**: `docs/sessions/2026-03-03-qrcode-seo-workflow/validated-workflows/workflows/06-seo-keyword-ingestion.yaml`

**Add inline schema to extract_keyword_data task**:
```yaml
- id: extract_keyword_data
  infer: |
    Extract structured SEO keyword data from the following research:
    {{use.research}}

    Return the data in the specified JSON format.
  output:
    format: json
    max_retries: 3
    schema:
      type: object
      properties:
        keywords:
          type: array
          items:
            type: object
            properties:
              value:
                type: string
                description: The keyword phrase
              slug_form:
                type: string
                pattern: "^[a-z0-9-]+$"
                description: URL-safe slug form
              volume:
                type: integer
                minimum: 0
              difficulty:
                type: integer
                minimum: 0
                maximum: 100
              intent:
                type: string
                enum: [informational, navigational, transactional, commercial]
            required: [value, slug_form, volume, difficulty, intent]
      required: [keywords]
```

### Task 5.2: Run workflow 06 and verify 16/16

```bash
cd tools/nika
cargo run -- run ../../docs/sessions/2026-03-03-qrcode-seo-workflow/validated-workflows/workflows/06-seo-keyword-ingestion.yaml
```

**Expected**: All 16 tasks pass with validated output.

**Verify traces**:
```bash
cat .nika/traces/gen-*.ndjson | jq -s 'map(select(.kind.type == "task_completed")) | length'
# Should output: 16
```

---

## Phase 6: Complex Test Workflows

### Task 6.1: Create workflow 07 - Multi-locale content generation

**File**: `workflows/07-multi-locale-generation.yaml`

Features tested:
- for_each with locales
- Nested schema validation
- MCP tool chaining
- Template interpolation

### Task 6.2: Create workflow 08 - Entity relationship mapping

Features tested:
- Graph traversal with novanet_traverse
- Complex JSON Schema with $ref
- Conditional task execution
- Output artifacts

### Task 6.3: Create workflow 09 - Batch SEO analysis

Features tested:
- Parallel task execution
- Array output schemas
- Error recovery with retry
- File-based schema reference

### Task 6.4: Create workflow 10 - Full page generation pipeline

Features tested:
- All 5 Nika verbs (infer, exec, fetch, invoke, agent)
- NovaNet MCP integration (all 11 tools)
- Complex nested schemas
- Multi-step validation chain

---

## Verification Checklist

- [ ] `cargo test` - all tests pass
- [ ] `cargo clippy` - zero warnings
- [ ] Workflow 06: 16/16 tasks pass
- [ ] Workflow 07: all tasks pass
- [ ] Workflow 08: all tasks pass
- [ ] Workflow 09: all tasks pass
- [ ] Workflow 10: all tasks pass
- [ ] Traces show validation events
- [ ] Retry attempts logged correctly
- [ ] Output artifacts match schemas

---

## Appendix: Error Codes

| Code | Description |
|------|-------------|
| NIKA-020 | Schema validation failed |
| NIKA-021 | Max retries exceeded |
| NIKA-022 | Invalid schema reference |
| NIKA-023 | JSON extraction failed |

---

*Plan created: 2026-03-03*
*Target: 16/16 + 4 complex workflows*
