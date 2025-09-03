# Rust Coding Agent

A Rust-based interactive CLI tool that enables conversational AI interactions with Claude 3 Haiku, featuring extensible tool support for enhanced functionality.

## Features

- **Interactive CLI Interface**: Engage in natural language conversations with Claude 3 Haiku
- **Tool Integration**: Extensible tool system allowing Claude to perform actions like file operations
- **Conversation State Management**: Maintains context throughout the interaction session
- **JSON Schema Validation**: Type-safe tool inputs with automatic validation
- **Error Handling**: Robust error management for API calls and tool execution

## Prerequisites

- Rust 1.90+ (2025 edition)
- Anthropic API key

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-coding-agent
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env and add your Anthropic API key
```

3. Build the project:
```bash
cargo build --release
```

## Configuration

Create a `.env` file in the project root with the following:

```env
ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

## Usage

Run the interactive agent:

```bash
cargo run
```

Once started, you can:
- Type messages to converse with Claude
- Claude can use available tools (currently file reading)
- Press Ctrl+C or send an empty message to exit

### Example Session

```
You: Can you read the contents of Cargo.toml?
Tool use detected: read_file with input {"path":"Cargo.toml"}
Claude: Here are the contents of your Cargo.toml file...

You: What dependencies does this project use?
Claude: Based on the Cargo.toml file, this project uses several dependencies including...
```

## Architecture

### Core Components

- **Agent**: Manages conversation flow and tool execution loop
- **Config**: Handles environment configuration and API key management
- **Tool System**: Extensible framework for adding new capabilities
- **Tools**: Individual tool implementations (currently includes file reading)

### Available Tools

#### read_file
Allows Claude to read the contents of files in the current directory and subdirectories.

**Usage**: Claude can request to read files by specifying relative file paths.

## Development

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Adding New Tools

To add a new tool:

1. Create a new module in `src/tools/`
2. Implement the tool following the existing pattern:
   - Define input struct with `JsonSchema` derive
   - Create tool constructor function
   - Implement execution function
3. Register the tool in `main.rs`

Example tool structure:
```rust
use crate::tool::{Tool, ToolInputValidator};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct YourToolInput {
    // Define input parameters
}

pub fn your_tool() -> Tool {
    Tool {
        name: "your_tool".into(),
        description: "Description of what your tool does".into(),
        execute: execute_your_tool,
        validator: ToolInputValidator::new::<YourToolInput>(),
    }
}

fn execute_your_tool(tool: &Tool, input: serde_json::Value) -> Result<Vec<ContentBlock>, Box<dyn std::error::Error>> {
    // Implementation here
}
```

## Dependencies

- **anthropic_rust**: Anthropic API client (local development version)
- **tokio**: Async runtime for handling API calls
- **serde**: Serialization framework for JSON handling
- **jsonschema**: JSON schema validation for tool inputs
- **anyhow**: Error handling utilities
- **dotenv**: Environment variable loading

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run `cargo fmt` and `cargo clippy`
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Tomasz Mikus (hi@tomaszmik.us)
