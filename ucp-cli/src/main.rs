//! # UCP CLI
//!
//! Command-line interface for the Universal Component Protocol: extract
//! component APIs from source code, unify them across framework ports,
//! detect drift, and export machine-readable specs.
//!
//! Design rules:
//! - Every command is offline and deterministic.
//! - `--json` gives machine-readable summaries for CI.
//! - `validate` exits non-zero on structurally broken specs.

use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use ucp_core::cam::PackageManifest;
use ucp_core::validation::{has_blocking_issues, validate_component_set, Severity};
use ucp_maintainer::{
    diff::diff_specs_text,
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
#[command(
    name = "ucp",
    about = "Extract, unify, and diff UI component APIs across framework ports",
    version
)]
/// UCP CLI argument parser.
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract a spec from a source directory (offline, deterministic).
    Bootstrap {
        #[arg(long)]
        source_dir: String,
        #[arg(long, default_value = "./ucp-output")]
        output_dir: String,
        /// Print a machine-readable JSON summary instead of the human log.
        #[arg(long)]
        json: bool,
    },
    /// Validate spec structure; exits non-zero on blocking issues.
    Validate {
        spec: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Generate framework code from a spec.
    Generate {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 't')]
        target: GeneratorTarget,
        #[arg(long, short = 'o', default_value = "./generated")]
        output: PathBuf,
    },
    /// Import an external format into a UCP spec.
    Import {
        #[arg(long, short = 't')]
        target: ImportTarget,
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 'o', default_value = "./ucp-output")]
        output: PathBuf,
    },
    /// Export a spec to a partner format.
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
    /// Build a shadcn-compatible registry from a spec.
    Registry {
        #[command(subcommand)]
        action: RegistryAction,
    },
    /// Merge multiple specs into one unified spec.
    Merge {
        #[arg(long, num_args = 1..)]
        input: Vec<PathBuf>,
        #[arg(long, short = 'o', default_value = "./ucp-output/merged.json")]
        output: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// List or inspect components in a spec.
    Components {
        spec: PathBuf,
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
        #[arg(long, short = 'f')]
        filter: Option<String>,
        #[arg(long, short = 'v')]
        verbose: bool,
    },
    /// Interactive TUI for resolving merge conflicts.
    Curate {
        #[arg(long)]
        merged: PathBuf,
        #[arg(long, short = 'o', default_value = "./curated.json")]
        output: PathBuf,
    },
    /// Structural diff between two specs.
    Diff {
        #[arg(long)]
        spec_a: PathBuf,
        #[arg(long)]
        spec_b: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Merge DTCG design-token files.
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
    /// Drift detection: re-extract source and compare against the spec.
    VerifySpec {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long)]
        source_dir: PathBuf,
    },
    /// Local spec registry backed by SQLite.
    RegistryStore {
        #[arg(long)]
        db: Option<PathBuf>,
        #[command(subcommand)]
        action: RegistryStoreAction,
    },
    /// Watch source files and rebuild the spec on change.
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
    /// Export a spec to every supported format.
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
    let exit_code = dispatch(cli.command).await?;
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
    Ok(())
}

/// Dispatch commands; the return value is the process exit code.
async fn dispatch(command: Commands) -> anyhow::Result<i32> {
    match command {
        Commands::Bootstrap {
            source_dir,
            output_dir,
            json,
        } => cmd_bootstrap(&source_dir, &output_dir, json).await?,
        Commands::Validate { spec, json } => return cmd_validate(&spec, json),
        Commands::Generate {
            spec,
            target,
            output,
        } => cmd_generate(&spec, &target, &output)?,
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
            )?,
        },
        Commands::Merge {
            input,
            output,
            json,
        } => cmd_merge(&input, &output, json)?,
        Commands::Components {
            spec,
            format,
            filter,
            verbose,
        } => cmd_components(&spec, &format, verbose, filter.as_deref())?,
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
        Commands::Watch {
            source_dir,
            output,
            base_spec,
            library,
            version,
            debounce_ms,
        } => {
            ucp_maintainer::watch::run_watch(
                &source_dir,
                base_spec.as_deref(),
                &output,
                &library,
                &version,
                debounce_ms,
            )
            .await?;
        }
        Commands::ExportAll {
            spec,
            output,
            library,
            version,
        } => cmd_export_all(&spec, &output, &library, &version)?,
    }
    Ok(0)
}

// ---------------------------------------------------------------------------
// Command implementations
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

async fn cmd_bootstrap(source_dir: &str, output_dir: &str, json: bool) -> anyhow::Result<()> {
    if !json {
        println!("Scanning {source_dir}...");
    }
    let cache_path = PathBuf::from(output_dir).join(".ucp-cache.json");
    let opts = ucp_synthesizer::pipeline::PipelineOptions {
        cache_path: Some(cache_path),
    };
    let output = ucp_synthesizer::pipeline::run_pipeline_with_options(source_dir, &opts)
        .await
        .context("Pipeline failed")?;
    std::fs::create_dir_all(output_dir)?;
    let spec_path = PathBuf::from(output_dir).join("ucp-spec.json");
    output.save_to_file(&spec_path)?;
    if json {
        let summary = serde_json::json!({
            "ok": true,
            "output": spec_path.display().to_string(),
            "stats": {
                "files_scanned": output.stats.files_scanned,
                "files_parsed": output.stats.files_parsed,
                "components_found": output.stats.components_found,
                "conflicts_detected": output.stats.conflicts_detected,
                "cache_hits": output.stats.cache_hits,
            },
            "components": output.components.len(),
        });
        println!("{summary}");
    } else {
        print_stats(&output.stats);
        println!("\nSpec written to {}", spec_path.display());
        if output.components.is_empty() {
            println!("\nNo components found.");
        } else {
            println!("\nBootstrap complete.");
        }
    }
    Ok(())
}

/// Validate a spec structurally; returns the process exit code.
fn cmd_validate(spec: &Path, json: bool) -> anyhow::Result<i32> {
    let output = SynthesisOutput::load_from_file(spec).context("Failed to load spec")?;
    let issues = validate_component_set(&output.components);
    let blocking = has_blocking_issues(&issues);
    let errors = issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .count();
    let warnings = issues
        .iter()
        .filter(|i| i.severity == Severity::Warning)
        .count();

    if json {
        let summary = serde_json::json!({
            "valid": !blocking,
            "errors": errors,
            "warnings": warnings,
            "components": output.components.len(),
            "conflicts": output
                .components
                .iter()
                .map(|c| c.props.iter().filter(|p| !p.conflicts.is_empty()).count())
                .sum::<usize>(),
            "issues": issues
                .iter()
                .map(|i| serde_json::json!({
                    "code": i.code,
                    "severity": if i.severity == Severity::Error { "error" } else { "warning" },
                    "component": i.component_id,
                    "message": i.message,
                }))
                .collect::<Vec<_>>(),
        });
        println!("{summary}");
    } else {
        print_stats(&output.stats);
        for issue in &issues {
            let tag = if issue.severity == Severity::Error {
                "ERROR"
            } else {
                "WARN"
            };
            println!("  [{tag}] {}: {}", issue.component_id, issue.message);
        }
        if blocking {
            println!("\nSpec is INVALID: {errors} error(s), {warnings} warning(s).");
        } else if errors + warnings == 0 {
            println!("\nSpec is valid (no structural issues).");
        } else {
            println!("\nSpec is valid with {errors} error(s), {warnings} warning(s).");
        }
    }
    Ok(i32::from(blocking))
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
            ucp_synthesizer::generate::react::generate_react(&manifest, &output.to_string_lossy())?;
        }
        GeneratorTarget::Gpui => {
            ucp_synthesizer::generate::gpui::generate_gpui(&manifest, &output.to_string_lossy())?;
        }
        GeneratorTarget::WebComponents => {
            ucp_synthesizer::generate::web_components::generate_web_components(
                &manifest,
                &output.to_string_lossy(),
            )?;
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
        format!("{target:?}").to_lowercase(),
        output.display()
    );
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
            ucp_synthesizer::export::llms_txt::export_llms_txt(&synth, &output.to_string_lossy())?;
        }
    }
    println!("Exported to {}", output.display());
    Ok(())
}

fn cmd_merge(inputs: &[PathBuf], output: &Path, json: bool) -> anyhow::Result<()> {
    let mut specs = Vec::new();
    for path in inputs {
        specs.push(SynthesisOutput::load_from_file(path)?);
    }
    let merged = ucp_synthesizer::merge::merge_specs(&specs, MergeOptions::default())
        .context("Merge failed")?;
    merged.save_to_file(output)?;
    if json {
        let summary = serde_json::json!({
            "ok": true,
            "output": output.display().to_string(),
            "components": merged.components.len(),
            "conflicts_detected": merged.stats.conflicts_detected,
        });
        println!("{summary}");
    } else {
        print_stats(&merged.stats);
        println!("\nMerge complete: {}", output.display());
    }
    Ok(())
}

fn cmd_components(
    spec: &Path,
    format: &str,
    verbose: bool,
    filter: Option<&str>,
) -> anyhow::Result<()> {
    let output = SynthesisOutput::load_from_file(spec)?;
    let filter_re: Option<regex::Regex> = filter.map(regex::Regex::new).transpose()?;
    let components: Vec<&ucp_core::cam::CanonicalAbstractComponent> = output
        .components
        .iter()
        .filter(|c| filter_re.as_ref().is_none_or(|re| re.is_match(&c.id)))
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
        println!("  [] {name}");
        if verbose {
            for p in &comp.props {
                let conflicts = p.conflicts.len();
                let marker = if conflicts > 0 {
                    format!(" [{conflicts} conflict(s)]")
                } else {
                    String::new()
                };
                println!(
                    "     - {}: {}{}",
                    p.canonical_name,
                    p.abstract_type.kind_name(),
                    marker
                );
            }
        }
    }
    Ok(())
}

fn print_stats(stats: &ucp_synthesizer::pipeline::PipelineStats) {
    println!("   Files scanned: {}", stats.files_scanned);
    println!("   Files parsed:  {}", stats.files_parsed);
    println!("   Components:    {}", stats.components_found);
    if stats.cache_hits > 0 {
        println!("   Cache hits:    {}", stats.cache_hits);
    }
    if stats.conflicts_detected > 0 {
        println!("   Conflicts:     {}", stats.conflicts_detected);
    }
    let _ = std::io::stdout().flush();
}

fn cmd_curate(merged: &Path, output: &Path) -> anyhow::Result<()> {
    let merged_spec =
        SynthesisOutput::load_from_file(merged).context("Failed to load merged spec")?;
    let curated =
        ucp_maintainer::curate_tui::run_curation_tui(&merged_spec).context("Curation failed")?;
    curated
        .save_to_file(output)
        .context("Failed to save curated spec")?;
    println!("Curated spec saved to {}", output.display());
    Ok(())
}

fn cmd_diff(spec_a: &Path, spec_b: &Path, json: bool) -> anyhow::Result<()> {
    let a = SynthesisOutput::load_from_file(spec_a).context("Failed to load spec_a")?;
    let b = SynthesisOutput::load_from_file(spec_b).context("Failed to load spec_b")?;

    if json {
        let report = ucp_maintainer::diff::diff_specs(&a, &b)?;
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        let text = diff_specs_text(&a, &b)?;
        println!("{text}");
    }
    Ok(())
}

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
        println!("{} conflict(s) detected:", result.conflicts.len());
        for c in &result.conflicts {
            println!("  - {}:", c.key);
            for (src, val) in &c.values {
                println!("      {src}: {val}");
            }
        }
    }

    let json = serde_json::to_string_pretty(&result.merged)?;
    std::fs::write(output, json)?;
    println!("Merged tokens saved to {}", output.display());
    Ok(())
}

async fn cmd_verify_spec(spec_path: &Path, source_dir: &Path) -> anyhow::Result<()> {
    let spec = SynthesisOutput::load_from_file(spec_path)?;
    let report = verify_spec_against_source(&spec, &source_dir.to_string_lossy())
        .await
        .context("Drift detection failed")?;

    if report.drifted_components.is_empty()
        && report.missing_in_source.is_empty()
        && report.new_in_source.is_empty()
    {
        println!("No drift detected - spec is up-to-date.");
    } else {
        println!("Drift detected:");
        for comp in &report.drifted_components {
            println!(
                "  ~ {} (confidence: {:.2})",
                comp.component_id, comp.confidence
            );
            for drift in &comp.prop_drifts {
                println!(
                    "      {}: {} -> {}",
                    drift.prop_name, drift.spec_type, drift.source_type
                );
            }
        }
        for comp in &report.missing_in_source {
            println!("  - {comp} (in spec, missing from source)");
        }
        for comp in &report.new_in_source {
            println!("  + {comp} (in source, not in spec)");
        }
    }

    Ok(())
}

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
            println!("Stored as ID {id} (name: {name})");
        }
        RegistryStoreAction::List => {
            let items = store.list().context("Failed to list registry")?;
            if items.is_empty() {
                println!("Registry is empty.");
            } else {
                for (id, name) in &items {
                    println!("  [{id}] {name}");
                }
            }
        }
        RegistryStoreAction::Show { id } => {
            match store.get(id).context("Failed to retrieve spec")? {
                Some(spec) => {
                    println!("{}", serde_json::to_string_pretty(&spec)?);
                }
                None => println!("No spec with ID {id}"),
            }
        }
        RegistryStoreAction::Delete { id } => {
            if store.delete(id).context("Failed to delete spec")? {
                println!("Deleted spec ID {id}");
            } else {
                println!("No spec with ID {id}");
            }
        }
    }
    Ok(())
}

fn cmd_export_all(
    spec_path: &Path,
    output_dir: &Path,
    library: &str,
    version: &str,
) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(spec_path).context("Failed to read spec")?;
    let synth: SynthesisOutput = serde_json::from_str(&content).context("Invalid spec format")?;

    std::fs::create_dir_all(output_dir)?;

    ucp_synthesizer::export::w3c::export_w3c(&synth, &output_dir.join("w3c").to_string_lossy())?;

    ucp_synthesizer::export::design_md::export_design_md(
        &synth,
        None,
        library,
        version,
        &output_dir.join("design-md").to_string_lossy(),
    )?;

    ucp_synthesizer::export::llms_txt::export_llms_txt(
        &synth,
        &output_dir.join("llms-txt").to_string_lossy(),
    )?;

    let manifest = synth.to_package_manifest(library, version, vec!["dioxus".into()]);
    ucp_synthesizer::generate::registry::generate_registry(
        &manifest,
        &output_dir.join("registry").to_string_lossy(),
        None,
        None,
        None,
    )?;

    println!("All exports written to {}", output_dir.display());
    Ok(())
}
