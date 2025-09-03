use crate::tool::Tool;
use anthropic_rust::{ChatRequest, Client, ContentBlock, MessageParam, Model, Role};
use std::io::{Write, stdin};

pub struct Agent {
    client: Client,
    messages: Vec<MessageParam>,
    tools: Vec<Tool>,
}

impl Agent {
    pub fn new(api_key: String, tools: Vec<Tool>) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .api_key(api_key)
            .model(Model::Claude3Haiku20240307)
            .build()?;
        Ok(Self {
            client,
            messages: vec![],
            tools,
        })
    }

    fn get_user_message(&self) -> Option<String> {
        print!("You: ");
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => Some(buffer),
            Err(_) => None,
        }
    }

    fn get_tools(&self) -> Vec<anthropic_rust::Tool> {
        self.tools
            .iter()
            .map(|t| {
                anthropic_rust::Tool::builder(t.name.clone())
                    .description(t.description.clone())
                    .schema_value(t.validator.schema.clone())
                    .build()
            })
            .collect()
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let user_input = match self.get_user_message() {
                Some(input) => input,
                None => break,
            };
            if user_input.trim() == "" {
                continue;
            }
            self.messages.push(MessageParam {
                content: vec![ContentBlock::Text {
                    text: user_input,
                    citations: None,
                }],
                role: Role::User,
            });
            let mut message = self
                .client
                .execute_chat(ChatRequest {
                    messages: self.messages.clone(),
                    system: None,
                    tools: Some(self.get_tools()),
                    temperature: None,
                    top_p: None,
                    stop_sequences: None,
                })
                .await?;
            self.messages.push(message.clone().into());
            // Keep processing until Claude stops using tools
            loop {
                let mut tool_results = vec![];
                let mut has_tool_use = false;
                for content in message.content.iter() {
                    match content {
                        ContentBlock::Text { text, .. } => {
                            println!("Claude: {}", text);
                        }
                        ContentBlock::Image { .. } => {
                            println!("Claude: <IMAGE>");
                        }
                        ContentBlock::Document { .. } => {
                            println!("Claude: <DOCUMENT>");
                        }
                        ContentBlock::ToolUse {
                            id, name, input, ..
                        } => {
                            has_tool_use = true;
                            println!(
                                "Tool use detected: {} with input {}",
                                name,
                                input.to_string()
                            );
                            let tool = match self.tools.iter().filter(|t| &t.name == name).next() {
                                Some(tool) => tool,
                                None => {
                                    println!("Tool not found");
                                    continue;
                                }
                            };
                            match (tool.execute)(&tool, input.to_owned()) {
                                Ok(result) => tool_results.push(ContentBlock::ToolResult {
                                    tool_use_id: id.to_owned(),
                                    content: result,
                                    is_error: Some(false),
                                }),
                                Err(e) => tool_results.push(ContentBlock::ToolResult {
                                    tool_use_id: id.to_owned(),
                                    content: vec![ContentBlock::Text {
                                        text: e.to_string(),
                                        citations: None,
                                    }],
                                    is_error: Some(true),
                                }),
                            }
                        }
                        ContentBlock::ToolResult { .. } => {
                            println!("Claude: <TOOL RESULT>");
                        }
                    }
                }
                // If Claude didn't use any tools, break out of the loop
                if !has_tool_use {
                    break;
                }
                // Add the tool results to the message and send it back to Claude
                self.messages.push(MessageParam {
                    content: tool_results,
                    role: Role::User,
                });
                // Send the message back to Claude
                message = self
                    .client
                    .execute_chat(ChatRequest {
                        messages: self.messages.clone(),
                        system: None,
                        tools: None,
                        temperature: None,
                        top_p: None,
                        stop_sequences: None,
                    })
                    .await?;
                self.messages.push(message.clone().into());
            }
        }
        println!("Bye!");
        Ok(())
    }
}
