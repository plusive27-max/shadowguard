use shadowguard::{Args, scan_directory, analyze_file, get_llm_summary, generate_report};
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("🚀 ShadowGuard starting secret scan on: {}", args.path.bold());

    let files = scan_directory(&args.path)?;
    println!("   Found {} files to analyze", files.len());

    let progress = ProgressBar::new(files.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut all_findings = Vec::new();
    for file in files {
        let findings = analyze_file(&file);
        all_findings.extend(findings);
        progress.inc(1);
    }
    progress.finish_with_message("✅ File analysis complete");

    println!("🤖 Asking local LLM for intelligent risk analysis...");
    let llm_response = get_llm_summary(&all_findings, &args.model)
        .await
        .unwrap_or_else(|e| format!("LLM unavailable: {}", e));

    println!("\n{}", llm_response.cyan());

    generate_report(&all_findings, &args.output, args.high_risk_only)?;

    println!("✅ ShadowGuard finished!");
    Ok(())
}
