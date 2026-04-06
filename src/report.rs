use crate::analyzer::SecretFinding;
use std::fs;
use chrono::Local;

pub fn generate_report(findings: &[SecretFinding], output_path: &str, high_risk_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut md = format!("# 🔑 ShadowGuard Secrets Report\n\n**Generated:** {}\n\n", Local::now().format("%Y-%m-%d %H:%M:%S"));
    md.push_str(&format!("**Total secrets found:** {}\n\n", findings.len()));

    for finding in findings {
        if high_risk_only && finding.risk_score < 70 { continue; }
        md.push_str(&format!("### {}\n", finding.path));
        md.push_str(&format!("**Type:** {}\n**Risk:** {} / 100\n**Snippet:** `{}`\n\n", 
            finding.secret_type, finding.risk_score, finding.line_snippet));
    }

    fs::write(output_path, md)?;
    println!("✅ Report saved → {}", output_path);
    Ok(())
}
