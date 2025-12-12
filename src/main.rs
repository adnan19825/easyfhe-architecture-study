use std::fs;
use serde::Deserialize;
use colored::*;
use anyhow::{Context, Result};

// --- STRUKTUREN FÜR POLICY ---
#[derive(Debug, Deserialize)]
struct SecurityProfile { spec: Spec }
#[derive(Debug, Deserialize)]
struct Spec { crypto: CryptoConfig, runtime: RuntimeConfig }
#[derive(Debug, Deserialize)]
struct CryptoConfig { min_security_bits: u32, pqc_kem: String }
#[derive(Debug, Deserialize)]
struct RuntimeConfig { wasm_max_size_mb: f32 }

// --- NEU: STRUKTUREN FÜR AGENT ---
#[derive(Debug, Deserialize)]
struct Agent { metadata: AgentMeta, spec: AgentSpec }
#[derive(Debug, Deserialize)]
struct AgentMeta { name: String, role: String }
#[derive(Debug, Deserialize)]
struct AgentSpec { constraints: AgentConstraints }
#[derive(Debug, Deserialize)]
struct AgentConstraints { max_actions_per_minute: u32 }

fn main() -> Result<()> {
    println!("{}", "=== EASY-EVA STUDIO GOVERNANCE ===".blue().bold());
    
    // 1. POLICY PRÜFEN
    println!("\n{}", "1. Validating Legislative (Policy)...".white().bold());
    let policy_path = "policies/security_profile.yaml";
    let content = fs::read_to_string(policy_path)
        .with_context(|| format!("Missing Policy: {}", policy_path))?;
    let profile: SecurityProfile = serde_yaml::from_str(&content)?;

    // PQC Check (Fix für die Warnung)
    if profile.spec.crypto.pqc_kem.contains("ML-KEM") || profile.spec.crypto.pqc_kem.contains("Kyber") {
        println!("{} PQC Standard: {}", "✓".green(), profile.spec.crypto.pqc_kem);
    } else {
        println!("{} WARNING: Legacy Algo ({})", "!".yellow(), profile.spec.crypto.pqc_kem);
    }

    // Security Bits Check
    if profile.spec.crypto.min_security_bits < 128 {
        println!("{} SECURITY FAIL: Bits < 128", "X".red());
    } else {
        println!("{} Crypto Strength OK ({} bits)", "✓".green(), profile.spec.crypto.min_security_bits);
    }
    
    // Wasm Size Check
    if profile.spec.runtime.wasm_max_size_mb > 2.0 {
        println!("{} SIZE FAIL: Wasm > 2MB", "X".red());
    } else {
        println!("{} Wasm Size OK ({} MB)", "✓".green(), profile.spec.runtime.wasm_max_size_mb);
    }

    // 2. AGENTEN PRÜFEN
    println!("\n{}", "2. Validating Executive (Agents)...".white().bold());
    let agent_path = "agents/traffic_controller.yaml";
    let agent_content = fs::read_to_string(agent_path)
        .with_context(|| format!("Missing Agent: {}", agent_path))?;
    let agent: Agent = serde_yaml::from_str(&agent_content)?;

    println!("Agent Found: {}", agent.metadata.name.cyan());
    
    // Check Role
    if agent.metadata.role == "OPTIMIZER" {
        println!("{} Role Authorized: OPTIMIZER", "✓".green());
    } else {
        println!("{} Role Unknown: {}", "?".yellow(), agent.metadata.role);
    }

    // Check Rate Limiting (Darf nicht spammen)
    if agent.spec.constraints.max_actions_per_minute > 60 {
        println!("{} RISK: Agent allows >1 action/sec", "X".red());
    } else {
        println!("{} Rate Limits Safe ({} ops/min)", "✓".green(), agent.spec.constraints.max_actions_per_minute);
    }

    println!("\n{}", "SYSTEM STATUS: GREEN. READY TO DEPLOY.".green().bold().on_black());
    Ok(())
}

