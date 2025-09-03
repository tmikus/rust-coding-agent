use std::fs;
use anthropic_rust::ContentBlock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::tool::{ToolInputValidator, Tool};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct ReadFileInput {
    path: String,
}

pub fn read_file_tool() -> Tool {
    Tool {
        description: "Read the contents of a given relative file path. Use this when you want to see what's inside a file. Do not use this with directory names.".into(),
        name: "read_file".into(),
        execute: execute_read_file,
        validator: ToolInputValidator::new::<ReadFileInput>(),
    }
}

fn execute_read_file(tool: &Tool, input: serde_json::Value) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>> {
    let input = tool.validator.get_value::<ReadFileInput>(input)?;
    println!("Executing read_file tool with path: {}", input.path);
    let content = fs::read_to_string(input.path)?;
    Ok(vec![ContentBlock::Text {
        citations: None,
        text: content,
    }])
}
