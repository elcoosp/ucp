use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use ucp_core::cam::PackageManifest;
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
    Contract {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 'o', default_value = "./ucp-contract.json")]
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
        } => cmd_bootstrap(&source_dir, &output_dir, ollama_url, &llm_model, watch).await,
        Commands::Validate { spec } => cmd_validate(&spec),
        Commands::Generate {
            spec,
            target,
            output,
        } => cmd_generate(&spec, &target, &output),
        Commands::Dashboard { spec, output } => cmd_dashboard(&spec, &output),
        Commands::Mcp { spec } => cmd_mcp(&spec).await,
        Commands::Contract { spec, output } => cmd_contract(&spec, &output),
        Commands::Export {
            target,
            spec,
            output,
            library,
            version,
        } => cmd_export(&spec, &target, &output, &library, &version),
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
        },
        Commands::Merge {
            input,
            output,
            html_dir,
        } => cmd_merge(&input, &output, &html_dir),
        Commands::Components {
            spec,
            format,
            filter,
            verbose,
        } => cmd_components(&spec, &format, verbose, &filter),
    }
}

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
    }
    println!("Exported to {}", output.display());
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
    let merged = ucp_synthesizer::merge::merge_specs(&specs).context("Merge failed")?;
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
