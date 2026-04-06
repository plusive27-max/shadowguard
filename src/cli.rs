use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "shadowguard", version = "0.1.0", author = "plusive27-max")]
pub struct Args {
    /// Path to scan (folder or drive)
    #[arg(short, long, required = true)]
    pub path: String,

    /// Output report file
    #[arg(short, long, default_value = "secrets-report.md")]
    pub output: String,

    /// Ollama model
    #[arg(short, long, default_value = "llama3.2")]
    pub model: String,

    /// Show only high-risk findings
    #[arg(long, default_value_t = false)]
    pub high_risk_only: bool,
}
