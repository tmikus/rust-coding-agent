use anthropic_rust::ContentBlock;
use schemars::{schema_for, JsonSchema};
use serde::de::DeserializeOwned;

pub struct Tool {
    pub description: String,
    pub execute: fn(tool: &Tool, input: serde_json::Value) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>>,
    pub name: String,
    pub validator: ToolInputValidator,
}

pub struct ToolInputValidator {
    pub schema: serde_json::Value,
    validator: jsonschema::Validator,
}

impl ToolInputValidator {
    pub fn new<T: JsonSchema>() -> Self {
        let schema = serde_json::to_value(schema_for!(T)).unwrap();
        let validator = jsonschema::validator_for(&schema).unwrap();
        Self {
            schema,
            validator,
        }
    }

    pub fn get_value<T: DeserializeOwned>(&self, input: serde_json::Value) -> Result<T, Box<dyn std::error::Error>> {
        if let Err(e) = self.validator.validate(&input) {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))
        }
        match serde_json::from_value(input) {
            Ok(value) => Ok(value),
            Err(e) => Err(e.into())
        }
    }
}
