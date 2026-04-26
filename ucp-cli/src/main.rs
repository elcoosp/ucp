use anyhow::Context;
use clap::{Parser, Subcommand};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "ucp",
    about = "UCP v4.0 AI Unification Engine",
    version,
    after_help = "EXAMPLES:\n    ucp bootstrap --source-dir ./src\n    ucp bootstrap --source-dir ./src --ollama-url http://localhost:11434 --llm-model llama3\n    ucp validate ucp-spec.json\n    ucp merge --input a.json --input b.json -o merged.json\n    ucp components ucp-spec.json\n    ucp generate --spec ucp-spec.json --target dioxus --output ./generated"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the full AI synthesis pipeline on a local source directory
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

    /// Validate a UCP spec file
    Validate { spec: PathBuf },

    /// Generate code from a UCP spec file
    Generate {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long, short = 't', default_value = "dioxus")]
        target: String,
        #[arg(long, short = 'o', default_value = "./generated")]
        output: PathBuf,
    },

    /// Merge multiple UCP spec files into a unified spec
    Merge {
        #[arg(long, num_args = 1..)]
        input: Vec<PathBuf>,
        #[arg(long, short = 'o', default_value = "./ucp-output/merged.json")]
        output: PathBuf,
        #[arg(long, default_value = "./ucp-output")]
        html_dir: String,
    },

    /// List components in a UCP spec file
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bootstrap { source_dir, output_dir, ollama_url, llm_model, watch } => {
            cmd_bootstrap(&source_dir, &output_dir, ollama_url, &llm_model, watch).await
        }
        Commands::Validate { spec } => cmd_validate(&spec),
        Commands::Generate { spec, target, output } => cmd_generate(&spec, &target, &output),
        Commands::Merge { input, output, html_dir } => cmd_merge(&input, &output, &html_dir),
        Commands::Components { spec, format, filter, verbose } => cmd_components(&spec, &format, verbose, &filter),
    }
}

fn cmd_generate(spec: &Path, target: &str, output: &Path) -> anyhow::Result<()> {
    use ucp_core::cam::PackageManifest;
    use ucp_synthesizer::pipeline::SynthesisOutput;

    let content = std::fs::read_to_string(spec).context("Failed to read spec")?;

    // Try PackageManifest first, then SynthesisOutput
    let manifest: PackageManifest = if let Ok(m) = serde_json::from_str(&content) {
        m
    } else if let Ok(syn) = serde_json::from_str::<SynthesisOutput>(&content) {
        syn.to_package_manifest("ucp-extracted", "0.1.0", vec!["dioxus".to_string()])
    } else {
        anyhow::bail!("Invalid spec format: expected PackageManifest or SynthesisOutput");
    };

    match target {
        "dioxus" => {
            ucp_synthesizer::generate::dioxus::generate_dioxus(&manifest, &output.to_string_lossy())
                .context("Failed to generate Dioxus code")?;
            println!("Generated Dioxus code in {}", output.display());
        }
        _ => anyhow::bail!("Unsupported target: {}. Supported: dioxus", target),
    }
    Ok(())
}
async fn cmd_bootstrap(
    source_dir: &str, output_dir: &str, ollama_url: Option<String>, llm_model: &str, watch: bool,
) -> anyhow::Result<()> {
    if watch {
        let err = std::process::Command::new("watchexec")
            .args(["-w", source_dir, "--clear", "-r", "ucp", "bootstrap", "--source-dir", source_dir])
            .exec();
        eprintln!("Failed to launch watchexec: {err}");
        std::process::exit(1);
    }
    println!("🔍 Scanning {}...", source_dir);
    let dry_run = ollama_url.is_none();
    let opts = ucp_synthesizer::pipeline::PipelineOptions { ollama_url, llm_model: llm_model.to_string(), dry_run };
    let output = ucp_synthesizer::pipeline::run_pipeline_with_options(source_dir, &opts).await.context("Pipeline failed")?;
    print_stats(&output.stats);
    std::fs::create_dir_all(output_dir).context("Failed to create output directory")?;
    let spec_path = PathBuf::from(output_dir).join("ucp-spec.json");
    output.save_to_file(&spec_path)?;
    println!("\n   ✓ Spec written to {}", spec_path.display());
    write_review_html(&output, output_dir, source_dir)?;
    if output.components.is_empty() { println!("\n⚠️  No components found."); } else { println!("\n✅ Synthesis complete!"); }
    Ok(())
}

fn cmd_validate(spec: &Path) -> anyhow::Result<()> {
    println!("📄 Validating {}...", spec.display());
    let output = ucp_synthesizer::pipeline::SynthesisOutput::load_from_file(spec).context("Failed to load spec")?;
    print_stats(&output.stats);
    let conflict_count: usize = output.components.iter().map(|c| c.props.iter().filter(|p| !p.conflicts.is_empty()).count()).sum();
    if conflict_count > 0 { println!("\n⚠️  {} unresolved conflict(s).", conflict_count); } else { println!("\n✅ Spec is valid with no conflicts."); }
    Ok(())
}

fn cmd_merge(inputs: &[PathBuf], output: &Path, html_dir: &str) -> anyhow::Result<()> {
    println!("🔗 Merging {} spec(s)...", inputs.len());
    let mut specs = Vec::new();
    for path in inputs {
        println!("   Loading {}...", path.display());
        let spec = ucp_synthesizer::pipeline::SynthesisOutput::load_from_file(path).with_context(|| format!("Failed to load {}", path.display()))?;
        specs.push(spec);
    }
    let merged = ucp_synthesizer::merge::merge_specs(&specs).context("Merge failed")?;
    print_stats(&merged.stats);
    merged.save_to_file(output)?;
    println!("\n   ✓ Merged spec written to {}", output.display());
    if merged.stats.conflicts_detected > 0 {
        let all_conflicts: Vec<_> = merged.components.iter().flat_map(|c| c.props.iter().flat_map(|p| p.conflicts.iter().cloned())).collect();
        let spec_json = serde_json::to_string_pretty(&merged)?;
        std::fs::create_dir_all(html_dir)?;
        let html = ucp_synthesizer::curation::generate_curation_html(&all_conflicts, &format!("Merged from {} specs", inputs.len()), "", &spec_json)?;
        let html_path = PathBuf::from(html_dir).join("review.html");
        std::fs::write(&html_path, &html)?;
        println!("   ✓ Review UI written to {}", html_path.display());
    }
    println!("\n✅ Merge complete!");
    Ok(())
}

fn cmd_components(spec: &Path, format: &str, verbose: bool, filter: &Option<String>) -> anyhow::Result<()> {
    let output = ucp_synthesizer::pipeline::SynthesisOutput::load_from_file(spec).context("Failed to load spec")?;
    let filter_re: Option<regex::Regex> = filter.as_ref().map(|f| regex::Regex::new(f).context(format!("Invalid filter regex: {f}"))).transpose()?;
    let mut components: Vec<&ucp_core::cam::CanonicalAbstractComponent> = output.components.iter().filter(|c| filter_re.as_ref().is_none_or(|re| re.is_match(&c.id))).collect();
    if format == "json" { let json = serde_json::to_string_pretty(&components)?; println!("{json}"); return Ok(()); }
    if components.is_empty() { println!("No components found."); return Ok(()); }
    components.sort_by(|a, b| a.id.cmp(&b.id));
    println!("{} component(s) in {}\n", components.len(), spec.display());
    for comp in &components {
        let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let source = comp.source_repos.first().map(|s| s.file_path.as_str()).unwrap_or("unknown");
        let events_len = comp.events.len();
        println!("  🧩 {} (from {})", name, source);
        if events_len > 0 { let event_names: Vec<_> = comp.events.iter().map(|e| e.canonical_name.as_str()).collect(); println!("     Events: {}", event_names.join(", ")); }
        if verbose {
            for prop in &comp.props {
                let conflict_marker = if prop.conflicts.is_empty() { String::new() } else { format!(" [{} conflict(s)]", prop.conflicts.len()) };
                println!("     - {}: {:?} (conf: {:.2}){}", prop.canonical_name, prop.abstract_type, prop.confidence, conflict_marker);
            }
            if let Some(ref sm) = comp.extracted_state_machine { println!("     State machine: {} (initial: {}, {} states)", sm.id, sm.initial, sm.states.len()); }
        } else { println!("     {} prop(s)", comp.props.len()); }
    }
    Ok(())
}

fn print_stats(stats: &ucp_synthesizer::pipeline::PipelineStats) {
    println!("   📁 Files scanned:   {}", stats.files_scanned);
    println!("   📄 Files parsed:    {}", stats.files_parsed);
    println!("   🧩 Components:     {}", stats.components_found);
    if stats.conflicts_detected > 0 { println!("   ⚠️  Conflicts:       {}", stats.conflicts_detected); }
    if stats.llm_enriched { println!("   🧠 LLM enriched:     yes"); }
}

fn write_review_html(output: &ucp_synthesizer::pipeline::SynthesisOutput, output_dir: &str, source_label: &str) -> anyhow::Result<()> {
    let all_conflicts: Vec<_> = output.components.iter().flat_map(|c| c.props.iter().flat_map(|p| p.conflicts.iter().cloned())).collect();
    let html_path = PathBuf::from(output_dir).join("review.html");
    let html = if !all_conflicts.is_empty() {
        let spec_json = serde_json::to_string_pretty(output)?;
        ucp_synthesizer::curation::generate_curation_html(&all_conflicts, &format!("Source: {}", source_label), "", &spec_json)?
    } else {
        format!("<!DOCTYPE html><html><head><style>body{{font-family:sans-serif;max-width:600px;margin:40px auto;padding:0 20px;}}</style></head><body><h1>UCP v4.0 Curation UI</h1><p>✅ No conflicts detected across {} components.</p><p><a href=\"ucp-spec.json\">View full spec</a></p></body></html>", output.components.len())
    };
    std::fs::write(&html_path, &html)?;
    println!("   ✓ Review UI written to {}", html_path.display());
    Ok(())
}
