use crate::tool::Tool;
use anthropic_rust::ContentBlock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct BashInput {
    #[schemars(description = "The bash command to execute.")]
    command: String,
}

pub fn execute_bash_tool() -> Tool {
    Tool::new(
        "bash".into(),
        "Execute a bash command and return its output. Use this to run shell commands.".into(),
        execute_bash,
    )
}

fn execute_bash(input: BashInput) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>> {
    let output = Command::new("bash")
    .arg("-c")
    .arg(input.command)
    .output()?;
    Ok(vec![ContentBlock::Text {
        citations: None,
        text: String::from_utf8_lossy(&output.stdout).trim().to_string(),
    }])
}
