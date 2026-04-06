# 🔑 ShadowGuard

**Local AI-Powered Credential & Secrets Guardian** written in Rust.

Scans your projects for hardcoded secrets, API keys, tokens, passwords, SSH keys, etc. and uses a local LLM (Ollama) to give professional risk analysis and remediation steps.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)

---

## ✨ Features
- ⚡ Fast recursive scanning with live progress bar
- 🧠 Advanced detection:
  - Regex patterns for 10+ common secret types
  - High-entropy string detection (keys/tokens)
- 🤖 Local LLM analysis (Ollama)
- 📄 Professional Markdown secrets report
- Zero cloud dependencies

## 🚀 Quick Start

1. Start Ollama:

    `` bash
   ollama run llama3.2

3. Build & run:

   ``bash
cargo build --release
.\target\release\shadowguard.exe --path "C:\Users\YourName\Projects" --output secrets-report.md

Usage Examples
   
    PowerShell
# Scan a project
shadowguard --path ".\my-project"

# High-risk only

shadowguard --path "D:\code" --high-risk-only
Security & Legal
Only scan code/repositories you own or have explicit permission to audit.

Built by plusive27-max
Part of the Rusty Cybersecurity Suite (portscanner → rustyforensics → shadowguard)
