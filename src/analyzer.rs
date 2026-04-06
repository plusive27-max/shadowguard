use regex::Regex;
use std::path::Path;
use std::fs;
use lazy_static::lazy_static;

lazy_static! {
    static ref SECRET_PATTERNS: Vec<(&'static str, &'static str)> = vec![
        (r#"sk-[a-zA-Z0-9]{48}"#, "OpenAI / Anthropic API key"),
        (r#"ghp_[a-zA-Z0-9]{36}"#, "GitHub Personal Access Token"),
        (r#"AKIA[0-9A-Z]{16}"#, "AWS Access Key ID"),
        (r#"amzn\.mws\.[0-9a-f]{20}"#, "Amazon MWS Key"),
        (r#"ssh-rsa [A-Za-z0-9+/=]+"#, "SSH Public Key"),
        (r#"password\s*[:=]\s*["']?[^"'\s]+"#, "Hardcoded password"),
        (r#"discord\.com/api/webhooks/[0-9]+/[A-Za-z0-9-]+"#, "Discord Webhook"),
        (r#"sk_live_[A-Za-z0-9]{24}"#, "Stripe Secret Key"),
        (r#"xoxb-[0-9]+-[A-Za-z0-9]+"#, "Slack Bot Token"),
        (r#"eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+"#, "JWT Token"),
        (r#"AIza[0-9A-Za-z-_]{35}"#, "Google API Key"),
    ];
}

#[derive(Debug, Clone)]
pub struct SecretFinding {
    pub path: String,
    pub risk_score: u8,
    pub secret_type: String,
    pub line_snippet: String,
}

pub fn analyze_file(path: &Path) -> Vec<SecretFinding> {
    let mut findings = Vec::new();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return findings,
    };

    // Regex-based secret detection
    for (pattern, secret_type) in SECRET_PATTERNS.iter() {
        let re = Regex::new(pattern).unwrap();
        for cap in re.find_iter(&content) {
            findings.push(SecretFinding {
                path: path.to_string_lossy().into_owned(),
                risk_score: 90,
                secret_type: secret_type.to_string(),
                line_snippet: cap.as_str().chars().take(70).collect(),
            });
        }
    }

    // Entropy check (high entropy = likely cryptographic material)
    if !content.is_empty() {
        let entropy = calculate_entropy(content.as_bytes());
        if entropy > 7.0 {
            findings.push(SecretFinding {
                path: path.to_string_lossy().into_owned(),
                risk_score: 75,
                secret_type: "High-entropy string (possible key/token)".to_string(),
                line_snippet: format!("Entropy: {:.2}", entropy),
            });
        }
    }

    findings
}

fn calculate_entropy(data: &[u8]) -> f64 {
    let mut freq = [0u32; 256];
    for &byte in data {
        freq[byte as usize] += 1;
    }
    let len = data.len() as f64;
    let mut entropy = 0.0;
    for count in freq {
        if count == 0 { continue; }
        let p = count as f64 / len;
        entropy -= p * p.log2();
    }
    entropy
}
