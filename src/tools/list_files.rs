use crate::tool::Tool;
use anthropic_rust::ContentBlock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ListFilesInput {
    #[schemars(description = "Optional relative path to list files from. Defaults to current directory if not provided.")]
    path: Option<String>,
}

pub fn list_files_tool() -> Tool {
    Tool::new(
        "list_files".into(),
        "List files and directories at a given path. If no path is provided, lists files in the current directory.".into(),
        execute_list_files,
    )
}

fn execute_list_files(input: ListFilesInput) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>> {
    let path = input.path.unwrap_or_else(|| ".".to_string());
    let files = fs::read_dir(path)?;
    let mut content = vec![];
    for file in files {
        let file = file?;
        let file_name = file.file_name().to_string_lossy().to_string();
        if file.file_type()?.is_dir() {
            if file_name == ".direnv" || file_name.starts_with(".direnv/") {
                continue;
            }
            content.push(file_name + "/");
        } else {
            content.push(file_name);
        }
    }
    serde_json::to_string(&content)
        .map(|s| vec![ContentBlock::Text { citations: None, text: s }])
        .map_err(|e| e.into())
}
