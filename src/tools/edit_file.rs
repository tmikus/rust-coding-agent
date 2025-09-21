use std::fs;
use anthropic_rust::ContentBlock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::tool::Tool;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct EditFileInput {
    #[schemars(description = "Text to replace the old_str with")]
    new_str: String,
    #[schemars(description = "Text to search for - must match exactly and must only have one match exactly")]
    old_str: String,
    #[schemars(description = "The path to the file")]
    path: String,
}

pub fn edit_file_tool() -> Tool {
    Tool::new(
        "edit_file".into(),
        "Make edits to a text file.

Replaces 'old_str' with 'new_str' in the given file. 'old_str' and 'new_str' MUST be different from each other.

If the file specified with path doesn't exist, it will be created.
".into(),
        execute_edit_file,
    )
}

fn create_new_file(path: &str, content: &str) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>> {
    fs::write(path, content)?;
    Ok(vec![ContentBlock::Text {
        citations: None,
        text: "OK".to_string(),
    }])
}

fn execute_edit_file(input: EditFileInput) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>> {
    if input.path.is_empty() {
        return Err("path must not be empty".into());
    }
    if input.old_str == input.new_str {
        return Err("old_str and new_str must be different".into());
    }

    if !std::path::Path::new(&input.path).exists() {
        return create_new_file(&input.path, &input.new_str);
    }
    let file_content = fs::read_to_string(&input.path)?;
    // Special case: if old_str is empty, we're appending to the file
    let new_content: String;
    if input.old_str.is_empty() {
        new_content = file_content + &input.new_str;
    } else {
        // Count occurrences first to ensure we have exactly one match
        let count = file_content.matches(&input.old_str).count();
        if count == 0 {
            return Err("old_str not found in file".into());
        }
        if count > 1 {
            return Err(format!("old_str found {} times in file {}, must be unique", count, input.path).into())
        }
        new_content = file_content.replace(&input.old_str, &input.new_str);
    }
    fs::write(&input.path, new_content)?;
    Ok(vec![ContentBlock::Text {
        citations: None,
        text: "OK".to_string(),
    }])
}
