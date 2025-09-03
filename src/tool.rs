use anthropic_rust::ContentBlock;
use schemars::{JsonSchema, schema_for};
use serde::de::DeserializeOwned;

pub struct Tool {
    pub description: String,
    execute_fn: Box<dyn Fn(&Tool, serde_json::Value) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>>>,
    pub name: String,
    pub validator: ToolInputValidator,
}

impl Tool {
    pub fn new<T: DeserializeOwned + 'static>(
        name: String,
        description: String,
        execute_fn: impl Fn(T) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>> + 'static,
    ) -> Self
    where
        T: JsonSchema,
    {
        Self {
            name,
            description,
            execute_fn: Box::new(move |tool, input| {
                execute_fn(tool.validator.get_value(input)?)
            }),
            validator: ToolInputValidator::new::<T>(),
        }
    }

    pub fn execute(&self, input: serde_json::Value) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>> {
        (self.execute_fn)(self, input)
    }
}

pub struct ToolInputValidator {
    pub schema: serde_json::Value,
    validator: jsonschema::Validator,
}

impl ToolInputValidator {
    pub fn new<T: JsonSchema>() -> Self {
        let schema = serde_json::to_value(schema_for!(T)).unwrap();
        let validator = jsonschema::validator_for(&schema).unwrap();
        Self { schema, validator }
    }

    pub fn get_value<T: DeserializeOwned>(
        &self,
        input: serde_json::Value,
    ) -> Result<T, Box<dyn std::error::Error>> {
        if let Err(e) = self.validator.validate(&input) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            )));
        }
        match serde_json::from_value(input) {
            Ok(value) => Ok(value),
            Err(e) => Err(e.into()),
        }
    }
}
