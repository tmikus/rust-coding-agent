use crate::tool::Tool;
use anthropic_rust::ContentBlock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ReadFileInput {
    path: String,
}

pub fn read_file_tool() -> Tool {
    Tool::new(
        "read_file".into(),
        "Read the contents of a given relative file path. Use this when you want to see what's inside a file. Do not use this with directory names.".into(),
        execute_read_file,
    )
}

fn execute_read_file(input: ReadFileInput) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>> {
    println!("Executing read_file tool with path: {}", input.path);
    let content = fs::read_to_string(input.path)?;
    Ok(vec![ContentBlock::Text {
        citations: None,
        text: content,
    }])
}
