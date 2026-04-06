use crate::error::ShadowGuardError;
use crate::analyzer::SecretFinding;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;

pub async fn get_llm_summary(findings: &[SecretFinding], model: &str) -> Result<String, ShadowGuardError> {
    let ollama = Ollama::default();

    let mut context = String::new();
    context.push_str(&format!("Found {} potential secrets across {} files.\n\n", findings.len(), findings.iter().map(|f| &f.path).collect::<std::collections::HashSet<_>>().len()));

    for finding in findings {
        context.push_str(&format!(
            "File: {}\nType: {}\nSnippet: {}\n\n",
            finding.path, finding.secret_type, finding.line_snippet
        ));
    }

    let prompt = format!(
        "You are a senior security engineer. Provide a clear, professional summary of the discovered secrets. \
        Explain risk level and give immediate remediation steps.\n\n{}",
        context
    );

    let request = GenerationRequest::new(model.to_string(), prompt);
    let response = ollama.generate(request).await
        .map_err(|e| ShadowGuardError::Ollama(e.to_string()))?;

    Ok(response.response)
}
