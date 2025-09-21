mod agent;
mod config;
mod tool;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::new()?;
    let mut agent = agent::Agent::new(config.api_key, vec![
        tools::bash::execute_bash_tool(),
        tools::edit_file::edit_file_tool(),
        tools::list_files::list_files_tool(),
        tools::read_file::read_file_tool(),
    ])?;
    agent.run().await
}
