//! # UCP CLI
//!
//! The command‑line interface for the Universal Component Protocol.
//!
//! Provides subcommands for bootstrapping, validation, merging, curation,
//! diff, token merging, drift detection, registry management, exporting,
//! and watch mode.

use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use ucp_core::cam::PackageManifest;
use ucp_maintainer::{
    diff::diff_specs_text,
    watch,
    registry::SpecStore,
    tokens::{merge_token_files, TokenMergeOptions},
    verify::verify_spec_against_source,
};
use ucp_synthesizer::merge::MergeOptions;
use ucp_synthesizer::pipeline::SynthesisOutput;

#[derive(ValueEnum, Clone, Debug)]
enum GeneratorTarget {
    Dioxus,
    Leptos,
    React,
    Gpui,
    #[clap(name = "web-components")]
    WebComponents,
    #[clap(name = "shadcn-registry")]
    ShadcnRegistry,
}

#[derive(ValueEnum, Clone, Debug)]
enum ExportTarget {
    A2ui,
    #[clap(name = "ag-ui")]
    AgUi,
    Dtcg,
    #[clap(name = "design-md")]
    DesignMd,
    #[clap(name = "llms-txt")]
    LlmsTxt,
}

#[derive(ValueEnum, Clone, Debug)]
enum ImportTarget {
    #[clap(name = "design-md")]
    DesignMd,
}

#[derive(Parser)]
#[command(name = "ucp", about = "UCP v4.0 AI Unification Engine", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Bootstrap {
        #[arg(long)]
        source_dir: String,
        #[arg(long, default_value = "./ucp-output")]
        output_dir: String,
        #[arg(long)]
        ollama_url: Option<String>,
        #[arg(long, default_value = "glm-5:cloud")]
        llm_model: String,
        #[arg(long)]
        watch: bool,
    },
    Validate {
        spec: PathBuf,
    },
    Generate {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 't')]
        target: GeneratorTarget,
        #[arg(long, short = 'o', default_value = "./generated")]
        output: PathBuf,
    },
    Dashboard {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 'o', default_value = "./dashboard")]
        output: PathBuf,
    },
    Mcp {
        #[arg(long)]
        spec: PathBuf,
    },
    McpServerJson {
        #[arg(long, short = 'n', default_value = "ucp-server")]
        name: String,
        #[arg(long, short = 'd', default_value = "UCP Component Intelligence Server")]
        description: String,
        #[arg(long, short = 'o', default_value = "./mcp")]
        output: PathBuf,
    },
    Contract {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 'o', default_value = "./ucp-contract.json")]
        output: PathBuf,
    },
    Import {
        #[arg(long, short = 't')]
        target: ImportTarget,
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 'o', default_value = "./ucp-output")]
        output: PathBuf,
    },
    Export {
        #[arg(long, short = 't')]
        target: ExportTarget,
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 'o', default_value = "./export")]
        output: PathBuf,
        #[arg(long, default_value = "ucp-library")]
        library: String,
        #[arg(long, default_value = "0.1.0")]
        version: String,
    },
    Registry {
        #[command(subcommand)]
        action: RegistryAction,
    },
    Merge {
        #[arg(long, num_args = 1..)]
        input: Vec<PathBuf>,
        #[arg(long, short = 'o', default_value = "./ucp-output/merged.json")]
        output: PathBuf,
        #[arg(long, default_value = "./ucp-output")]
        html_dir: String,
    },
    Components {
        spec: PathBuf,
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
        #[arg(long, short = 'f')]
        filter: Option<String>,
        #[arg(long, short = 'v')]
        verbose: bool,
    },
    /// v0.11: Interactive merge curation
    Curate {
        #[arg(long)]
        merged: PathBuf,
        #[arg(long, short = 'o', default_value = "./curated.json")]
        output: PathBuf,
    },
    /// v0.11: Structural spec diff
    Diff {
        #[arg(long)]
        spec_a: PathBuf,
        #[arg(long)]
        spec_b: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// v0.11: Merge DTCG tokens
    MergeTokens {
        #[arg(long, num_args = 1..)]
        input: Vec<PathBuf>,
        #[arg(long, short = 'o', default_value = "./merged-tokens.json")]
        output: PathBuf,
        #[arg(long, default_value = "error")]
        strategy: String,
        #[arg(long)]
        force: bool,
    },
    /// v0.11: Drift detection
    VerifySpec {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long)]
        source_dir: PathBuf,
    },
    /// v0.11: Local spec registry management
    RegistryStore {
        #[arg(long)]
        db: Option<PathBuf>,
        #[command(subcommand)]
        action: RegistryStoreAction,
    },
    /// v0.11: Watch source directory and auto-rebuild
    Watch {
        #[arg(long)]
        source_dir: String,
        #[arg(long, short = 'o', default_value = "./watch-output")]
        output: PathBuf,
        #[arg(long)]
        base_spec: Option<PathBuf>,
        #[arg(long, default_value = "ucp-library")]
        library: String,
        #[arg(long, default_value = "0.1.0")]
        version: String,
        #[arg(long, default_value = "500")]
        debounce_ms: u64,
    },
    /// v0.11: Export all formats
    ExportAll {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 'o', default_value = "./export-all")]
        output: PathBuf,
        #[arg(long, default_value = "ucp-library")]
        library: String,
        #[arg(long, default_value = "0.1.0")]
        version: String,
    },
}

#[derive(Subcommand)]
enum RegistryAction {
    Build {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 'o', default_value = "./registry")]
        output: PathBuf,
        #[arg(long)]
        namespace: Option<String>,
        #[arg(long)]
        author: Option<String>,
        #[arg(long)]
        homepage: Option<String>,
        #[arg(long)]
        base: bool,
        #[arg(long, short = 't')]
        tokens: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum RegistryStoreAction {
    /// Store a spec in the local registry
    Store {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 'n')]
        name: String,
    },
    /// List stored specs
    List,
    /// Show a specific spec
    Show { id: i64 },
    /// Delete a spec
    Delete { id: i64 },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Bootstrap {
            source_dir,
            output_dir,
            ollama_url,
            llm_model,
            watch,
        } => cmd_bootstrap(&source_dir, &output_dir, ollama_url, &llm_model, watch)
            .await
            .context("Bootstrap failed")?,
        Commands::Validate { spec } => cmd_validate(&spec)?,
        Commands::Generate {
            spec,
            target,
            output,
        } => cmd_generate(&spec, &target, &output)?,
        Commands::Dashboard { spec, output } => cmd_dashboard(&spec, &output)?,
        Commands::Mcp { spec } => cmd_mcp(&spec).await.context("MCP server failed")?,
        Commands::McpServerJson {
            name,
            description,
            output,
        } => cmd_mcp_server_json(&name, &description, &output)?,
        Commands::Contract { spec, output } => cmd_contract(&spec, &output)?,
        Commands::Import {
            target,
            spec,
            output,
        } => cmd_import(&spec, &target, &output)?,
        Commands::Export {
            target,
            spec,
            output,
            library,
            version,
        } => cmd_export(&spec, &target, &output, &library, &version)?,
        Commands::Registry { action } => match action {
            RegistryAction::Build {
                spec,
                output,
                namespace,
                author,
                homepage,
                base,
                tokens: _,
            } => cmd_registry_build(
                &spec,
                &output,
                namespace.as_deref(),
                author.as_deref(),
                homepage.as_deref(),
                base,
            ),
        }?,
        Commands::Merge {
            input,
            output,
            html_dir,
        } => cmd_merge(&input, &output, &html_dir)?,
        Commands::Components {
            spec,
            format,
            filter,
            verbose,
        } => cmd_components(&spec, &format, verbose, &filter)?,
        // v0.11 commands
        Commands::Curate { merged, output } => cmd_curate(&merged, &output)?,
        Commands::Diff {
            spec_a,
            spec_b,
            json,
        } => cmd_diff(&spec_a, &spec_b, json)?,
        Commands::MergeTokens {
            input,
            output,
            strategy,
            force,
        } => cmd_merge_tokens(&input, &output, &strategy, force)?,
        Commands::VerifySpec { spec, source_dir } => cmd_verify_spec(&spec, &source_dir).await?,
        Commands::RegistryStore { db, action } => cmd_registry_store(db, action)?,
        Commands::Watch { source_dir, output, base_spec, library, version, debounce_ms } => {
            ucp_maintainer::watch::run_watch(
                &source_dir,
                base_spec.as_deref(),
                &output,
                &library,
                &version,
                debounce_ms,
            ).await?;
        },
        Commands::ExportAll {
            spec,
            output,
            library,
            version,
        } => cmd_export_all(&spec, &output, &library, &version)?,
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Existing command implementations (unchanged)
// ---------------------------------------------------------------------------

fn load_manifest(spec: &Path) -> anyhow::Result<PackageManifest> {
    let content = std::fs::read_to_string(spec).context("Failed to read spec")?;
    if let Ok(m) = serde_json::from_str(&content) {
        return Ok(m);
    }
    if let Ok(syn) = serde_json::from_str::<SynthesisOutput>(&content) {
        return Ok(syn.to_package_manifest("ucp-extracted", "0.1.0", vec!["dioxus".into()]));
    }
    anyhow::bail!("Invalid spec format")
}

fn cmd_generate(spec: &Path, target: &GeneratorTarget, output: &Path) -> anyhow::Result<()> {
    let manifest = load_manifest(spec)?;
    match target {
        GeneratorTarget::Dioxus => ucp_synthesizer::generate::dioxus::generate_dioxus(
            &manifest,
            &output.to_string_lossy(),
        )?,
        GeneratorTarget::Leptos => ucp_synthesizer::generate::leptos::generate_leptos(
            &manifest,
            &output.to_string_lossy(),
        )?,
        GeneratorTarget::React => {
            ucp_synthesizer::generate::react::generate_react(&manifest, &output.to_string_lossy())?
        }
        GeneratorTarget::Gpui => {
            ucp_synthesizer::generate::gpui::generate_gpui(&manifest, &output.to_string_lossy())?
        }
        GeneratorTarget::WebComponents => {
            ucp_synthesizer::generate::web_components::generate_web_components(
                &manifest,
                &output.to_string_lossy(),
            )?
        }
        GeneratorTarget::ShadcnRegistry => ucp_synthesizer::generate::registry::generate_registry(
            &manifest,
            &output.to_string_lossy(),
            None,
            None,
            None,
        )?,
    }
    println!(
        "Generated {} code in {}",
        format!("{:?}", target).to_lowercase(),
        output.display()
    );
    Ok(())
}

fn cmd_export(
    spec: &Path,
    target: &ExportTarget,
    output: &Path,
    library: &str,
    version: &str,
) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(spec).context("Failed to read spec")?;
    let synth: SynthesisOutput = serde_json::from_str(&content).context("Invalid spec format")?;
    match target {
        ExportTarget::A2ui => ucp_synthesizer::export::a2ui::export_a2ui(
            &synth,
            library,
            version,
            &output.to_string_lossy(),
        )?,
        ExportTarget::AgUi => {
            ucp_synthesizer::export::ag_ui::export_ag_ui(&synth, &output.to_string_lossy())?
        }
        ExportTarget::Dtcg => {
            let tokens = ucp_synthesizer::extract::tokens::extract_tokens_from_source(&content)?;
            ucp_synthesizer::extract::tokens::export_tokens_to_dtcg(
                &tokens,
                &output.to_string_lossy(),
            )?;
        }
        ExportTarget::DesignMd => ucp_synthesizer::export::design_md::export_design_md(
            &synth,
            None,
            library,
            version,
            &output.to_string_lossy(),
        )?,
        ExportTarget::LlmsTxt => {
            ucp_synthesizer::export::llms_txt::export_llms_txt(&synth, &output.to_string_lossy())?
        }
    }
    println!("Exported to {}", output.display());
    Ok(())
}

fn cmd_import(spec: &Path, target: &ImportTarget, output: &Path) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(spec).context("Failed to read spec")?;
    match target {
        ImportTarget::DesignMd => {
            let manifest = ucp_synthesizer::import::design_md::parse_design_md(&content)
                .context("Failed to parse DESIGN.md")?;
            let output_path = output.join("ucp-spec.json");
            let json = serde_json::to_string_pretty(&manifest)?;
            std::fs::write(&output_path, json)?;
            println!("Imported DESIGN.md to {}", output_path.display());
        }
    }
    Ok(())
}

fn cmd_registry_build(
    spec: &Path,
    output: &Path,
    namespace: Option<&str>,
    author: Option<&str>,
    homepage: Option<&str>,
    base: bool,
) -> anyhow::Result<()> {
    let manifest = load_manifest(spec)?;
    if base {
        ucp_synthesizer::generate::registry::generate_registry_base(
            &manifest,
            None,
            &output.to_string_lossy(),
            namespace,
            author,
            homepage,
        )?;
    } else {
        ucp_synthesizer::generate::registry::generate_registry(
            &manifest,
            &output.to_string_lossy(),
            namespace,
            author,
            homepage,
        )?;
    }
    println!("Registry built in {}", output.display());
    Ok(())
}

fn cmd_dashboard(spec: &Path, output: &Path) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(spec).context("Failed to read spec")?;
    let synth: SynthesisOutput = serde_json::from_str(&content).context("Invalid spec format")?;
    ucp_synthesizer::dashboard::generator::generate_dashboard(&synth, &output.to_string_lossy())?;
    println!("Dashboard written to {}", output.display());
    Ok(())
}

async fn cmd_mcp(spec: &Path) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(spec).context("Failed to read spec")?;
    let synth: SynthesisOutput = serde_json::from_str(&content).context("Invalid spec format")?;
    ucp_synthesizer::contract::mcp_server::run_mcp_server(&synth).await?;
    Ok(())
}

fn cmd_mcp_server_json(name: &str, description: &str, output: &Path) -> anyhow::Result<()> {
    ucp_synthesizer::contract::mcp_server::generate_server_json(
        name,
        description,
        &output.to_string_lossy(),
    )?;
    println!(
        "MCP server.json written to {}",
        output.join("server.json").display()
    );
    Ok(())
}

fn cmd_contract(spec: &Path, output: &Path) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(spec).context("Failed to read spec")?;
    let synth: SynthesisOutput = serde_json::from_str(&content).context("Invalid spec format")?;
    ucp_synthesizer::contract::ai_contract::export_ai_contract(&synth, &output.to_string_lossy())?;
    println!("AI contract written to {}", output.display());
    Ok(())
}

async fn cmd_bootstrap(
    source_dir: &str,
    output_dir: &str,
    ollama_url: Option<String>,
    llm_model: &str,
    watch: bool,
) -> anyhow::Result<()> {
    if watch {
        let err = std::process::Command::new("watchexec")
            .args([
                "-w",
                source_dir,
                "--clear",
                "-r",
                "ucp",
                "bootstrap",
                "--source-dir",
                source_dir,
            ])
            .exec();
        eprintln!("Failed to launch watchexec: {err}");
        std::process::exit(1);
    }
    println!("🔍 Scanning {}...", source_dir);
    let dry_run = ollama_url.is_none();
    let opts = ucp_synthesizer::pipeline::PipelineOptions {
        ollama_url,
        llm_model: llm_model.to_string(),
        dry_run,
    };
    let output = ucp_synthesizer::pipeline::run_pipeline_with_options(source_dir, &opts)
        .await
        .context("Pipeline failed")?;
    print_stats(&output.stats);
    std::fs::create_dir_all(output_dir)?;
    let spec_path = PathBuf::from(output_dir).join("ucp-spec.json");
    output.save_to_file(&spec_path)?;
    println!("\n   ✓ Spec written to {}", spec_path.display());
    write_review_html(&output, output_dir, source_dir)?;
    if output.components.is_empty() {
        println!("\n⚠️  No components found.");
    } else {
        println!("\n✅ Synthesis complete!");
    }
    Ok(())
}

fn cmd_validate(spec: &Path) -> anyhow::Result<()> {
    let output = SynthesisOutput::load_from_file(spec).context("Failed to load spec")?;
    print_stats(&output.stats);
    let conflicts: usize = output
        .components
        .iter()
        .map(|c| c.props.iter().filter(|p| !p.conflicts.is_empty()).count())
        .sum();
    if conflicts > 0 {
        println!("\n⚠️  {} unresolved conflict(s).", conflicts);
    } else {
        println!("\n✅ Spec is valid with no conflicts.");
    }
    Ok(())
}

fn cmd_merge(inputs: &[PathBuf], output: &Path, html_dir: &str) -> anyhow::Result<()> {
    let mut specs = Vec::new();
    for path in inputs {
        specs.push(SynthesisOutput::load_from_file(path)?);
    }
    let merged = ucp_synthesizer::merge::merge_specs(&specs, MergeOptions::default())
        .context("Merge failed")?;
    print_stats(&merged.stats);
    merged.save_to_file(output)?;
    if merged.stats.conflicts_detected > 0 {
        std::fs::create_dir_all(html_dir)?;
        let html = ucp_synthesizer::curation::generate_curation_html(
            &[],
            "",
            "",
            &serde_json::to_string_pretty(&merged)?,
        )?;
        std::fs::write(PathBuf::from(html_dir).join("review.html"), &html)?;
    }
    println!("\n✅ Merge complete!");
    Ok(())
}

fn cmd_components(
    spec: &Path,
    format: &str,
    verbose: bool,
    filter: &Option<String>,
) -> anyhow::Result<()> {
    let output = SynthesisOutput::load_from_file(spec)?;
    let filter_re: Option<regex::Regex> =
        filter.as_ref().map(|f| regex::Regex::new(f)).transpose()?;
    let components: Vec<&ucp_core::cam::CanonicalAbstractComponent> = output
        .components
        .iter()
        .filter(|c| filter_re.as_ref().map_or(true, |re| re.is_match(&c.id)))
        .collect();
    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&components)?);
        return Ok(());
    }
    if components.is_empty() {
        println!("No components found.");
        return Ok(());
    }
    for comp in &components {
        let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        println!("  🧩 {}", name);
        if verbose {
            for p in &comp.props {
                println!("     - {}: {:?}", p.canonical_name, p.abstract_type);
            }
        }
    }
    Ok(())
}

fn print_stats(stats: &ucp_synthesizer::pipeline::PipelineStats) {
    println!("   📁 Files scanned:   {}", stats.files_scanned);
    println!("   📄 Files parsed:    {}", stats.files_parsed);
    println!("   🧩 Components:     {}", stats.components_found);
    if stats.conflicts_detected > 0 {
        println!("   ⚠️  Conflicts:       {}", stats.conflicts_detected);
    }
    if stats.llm_enriched {
        println!("   🧠 LLM enriched:     yes");
    }
}

fn write_review_html(
    output: &SynthesisOutput,
    output_dir: &str,
    source_label: &str,
) -> anyhow::Result<()> {
    let html = format!("<!DOCTYPE html><html><head><style>body{{font-family:sans-serif;}}</style></head><body><h1>UCP Review</h1><p>{} components from {}</p></body></html>", output.components.len(), source_label);
    std::fs::write(PathBuf::from(output_dir).join("review.html"), html)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// v0.11 command implementations
// ---------------------------------------------------------------------------

/// Run interactive curation on a merged spec.
fn cmd_curate(merged: &Path, output: &Path) -> anyhow::Result<()> {
    let merged_spec =
        SynthesisOutput::load_from_file(merged).context("Failed to load merged spec")?;
    let curated =
        ucp_maintainer::curate_tui::run_curation_tui(&merged_spec).context("Curation failed")?;
    curated
        .save_to_file(output)
        .context("Failed to save curated spec")?;
    println!("✅ Curated spec saved to {}", output.display());
    Ok(())
}

/// Diff two specs.
fn cmd_diff(spec_a: &Path, spec_b: &Path, json: bool) -> anyhow::Result<()> {
    let a = SynthesisOutput::load_from_file(spec_a).context("Failed to load spec_a")?;
    let b = SynthesisOutput::load_from_file(spec_b).context("Failed to load spec_b")?;

    if json {
        let report = ucp_maintainer::diff::diff_specs(&a, &b)?;
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        let text = diff_specs_text(&a, &b)?;
        println!("{}", text);
    }
    Ok(())
}

/// Merge DTCG token files.
fn cmd_merge_tokens(
    inputs: &[PathBuf],
    output: &Path,
    strategy: &str,
    force: bool,
) -> anyhow::Result<()> {
    let mut files = Vec::new();
    for path in inputs {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let tokens: ucp_synthesizer::extract::tokens::DtcgTokens =
            serde_json::from_str(&content)
                .with_context(|| format!("Invalid tokens file: {}", path.display()))?;
        files.push((path.to_string_lossy().to_string(), tokens));
    }

    let opts = TokenMergeOptions {
        strategy: strategy.to_string(),
        force,
    };
    let result = merge_token_files(
        &files
            .iter()
            .map(|(p, t)| (p.clone(), t.clone()))
            .collect::<Vec<_>>(),
        &opts,
    )
    .context("Token merge failed")?;

    if !result.conflicts.is_empty() {
        println!("⚠️  {} conflict(s) detected:", result.conflicts.len());
        for c in &result.conflicts {
            println!("  - {}:", c.key);
            for (src, val) in &c.values {
                println!("      {}: {}", src, val);
            }
        }
    }

    let json = serde_json::to_string_pretty(&result.merged)?;
    std::fs::write(output, json)?;
    println!("✅ Merged tokens saved to {}", output.display());
    Ok(())
}

/// Run drift detection against a source directory.
async fn cmd_verify_spec(spec_path: &Path, source_dir: &Path) -> anyhow::Result<()> {
    let spec = SynthesisOutput::load_from_file(spec_path)?;
    let report = verify_spec_against_source(&spec, &source_dir.to_string_lossy())
        .await
        .context("Drift detection failed")?;

    if report.drifted_components.is_empty()
        && report.missing_in_source.is_empty()
        && report.new_in_source.is_empty()
    {
        println!("✅ No drift detected – spec is up-to-date.");
    } else {
        println!("⚠️  Drift detected:");
        for comp in &report.drifted_components {
            println!(
                "  ~ {} (confidence: {:.2})",
                comp.component_id, comp.confidence
            );
            for drift in &comp.prop_drifts {
                println!(
                    "      {}: {} → {}",
                    drift.prop_name, drift.spec_type, drift.source_type
                );
            }
        }
        for comp in &report.missing_in_source {
            println!("  - {} (in spec, missing from source)", comp);
        }
        for comp in &report.new_in_source {
            println!("  + {} (in source, not in spec)", comp);
        }
    }

    Ok(())
}

/// Manage the local spec registry.
fn cmd_registry_store(db: Option<PathBuf>, action: RegistryStoreAction) -> anyhow::Result<()> {
    let db_path = db
        .unwrap_or_else(|| PathBuf::from("ucp-registry.db"))
        .to_string_lossy()
        .to_string();

    let store = SpecStore::open(&db_path).context("Failed to open registry store")?;

    match action {
        RegistryStoreAction::Store { spec, name } => {
            let spec = SynthesisOutput::load_from_file(&spec)?;
            let id = store.store(&name, &spec).context("Failed to store spec")?;
            println!("✅ Stored as ID {} (name: {})", id, name);
        }
        RegistryStoreAction::List => {
            let items = store.list().context("Failed to list registry")?;
            if items.is_empty() {
                println!("Registry is empty.");
            } else {
                for (id, name) in &items {
                    println!("  [{}] {}", id, name);
                }
            }
        }
        RegistryStoreAction::Show { id } => {
            match store.get(id).context("Failed to retrieve spec")? {
                Some(spec) => {
                    println!("{}", serde_json::to_string_pretty(&spec)?);
                }
                None => println!("No spec with ID {}", id),
            }
        }
        RegistryStoreAction::Delete { id } => {
            if store.delete(id).context("Failed to delete spec")? {
                println!("✅ Deleted spec ID {}", id);
            } else {
                println!("No spec with ID {}", id);
            }
        }
    }
    Ok(())
}

/// Export a spec to all supported formats.
fn cmd_export_all(
    spec_path: &Path,
    output_dir: &Path,
    library: &str,
    version: &str,
) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(spec_path).context("Failed to read spec")?;
    let synth: SynthesisOutput = serde_json::from_str(&content).context("Invalid spec format")?;

    std::fs::create_dir_all(output_dir)?;

    // A2UI
    ucp_synthesizer::export::a2ui::export_a2ui(
        &synth,
        library,
        version,
        &output_dir.join("a2ui").to_string_lossy(),
    )?;

    // AG-UI
    ucp_synthesizer::export::ag_ui::export_ag_ui(
        &synth,
        &output_dir.join("ag-ui").to_string_lossy(),
    )?;

    // W3C
    ucp_synthesizer::export::w3c::export_w3c(&synth, &output_dir.join("w3c").to_string_lossy())?;

    // DESIGN.md
    ucp_synthesizer::export::design_md::export_design_md(
        &synth,
        None,
        library,
        version,
        &output_dir.join("design-md").to_string_lossy(),
    )?;

    // LLMs.txt
    ucp_synthesizer::export::llms_txt::export_llms_txt(
        &synth,
        &output_dir.join("llms-txt").to_string_lossy(),
    )?;

    // AI Contract
    ucp_synthesizer::contract::ai_contract::export_ai_contract(
        &synth,
        &output_dir.join("ai-contract.json").to_string_lossy(),
    )?;

    // Registry (shadcn-compatible)
    let manifest = synth.to_package_manifest(library, version, vec!["dioxus".into()]);
    ucp_synthesizer::generate::registry::generate_registry(
        &manifest,
        &output_dir.join("registry").to_string_lossy(),
        None,
        None,
        None,
    )?;

    println!("✅ All exports written to {}", output_dir.display());
    println!("   📁 a2ui/a2ui-catalog.json");
    println!("   📁 ag-ui/ag-ui-events.json");
    println!("   📁 w3c/ucp-spec.w3c.json");
    println!("   📁 design-md/DESIGN.md");
    println!("   📁 llms-txt/llms.txt");
    println!("   📁 ai-contract.json");
    println!("   📁 registry/registry.json");

    Ok(())
}
