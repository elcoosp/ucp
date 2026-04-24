use anyhow::Context;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ucp", about = "UCP v4.0 AI Unification Engine")]
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
    },
    /// Validate a UCP spec file
    Validate {
        spec: PathBuf,
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bootstrap { source_dir, output_dir, ollama_url, llm_model } => {
            cmd_bootstrap(&source_dir, &output_dir, ollama_url, &llm_model).await
        }
        Commands::Validate { spec } => cmd_validate(&spec),
        Commands::Merge { input, output, html_dir } => cmd_merge(&input, &output, &html_dir),
    }
}

async fn cmd_bootstrap(
    source_dir: &str,
    output_dir: &str,
    ollama_url: Option<String>,
    llm_model: &str,
) -> anyhow::Result<()> {
    println!("🔍 Scanning {}...", source_dir);

    let dry_run = ollama_url.is_none();
    let opts = ucp_synthesizer::pipeline::PipelineOptions {
        ollama_url,
        llm_model: llm_model.to_string(),
        dry_run,
    };

    let output = ucp_synthesizer::pipeline::run_pipeline_with_options(&source_dir, &opts)
        .await
        .context("Pipeline failed")?;

    print_stats(&output.stats);

    std::fs::create_dir_all(output_dir).context("Failed to create output directory")?;

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

fn cmd_validate(spec: &PathBuf) -> anyhow::Result<()> {
    println!("📄 Validating {}...", spec.display());
    let output = ucp_synthesizer::pipeline::SynthesisOutput::load_from_file(spec)
        .context("Failed to load spec")?;
    print_stats(&output.stats);

    let conflict_count: usize = output
        .components
        .iter()
        .map(|c| c.props.iter().filter(|p| !p.conflicts.is_empty()).count())
        .sum();

    if conflict_count > 0 {
        println!("\n⚠️  {} unresolved conflict(s).", conflict_count);
    } else {
        println!("\n✅ Spec is valid with no conflicts.");
    }
    Ok(())
}

fn cmd_merge(inputs: &[PathBuf], output: &PathBuf, html_dir: &str) -> anyhow::Result<()> {
    println!("🔗 Merging {} spec(s)...", inputs.len());

    let mut specs = Vec::new();
    for path in inputs {
        println!("   Loading {}...", path.display());
        let spec = ucp_synthesizer::pipeline::SynthesisOutput::load_from_file(path)
            .with_context(|| format!("Failed to load {}", path.display()))?;
        println!("      → {} components", spec.components.len());
        specs.push(spec);
    }

    let merged = ucp_synthesizer::merge::merge_specs(&specs).context("Merge failed")?;
    print_stats(&merged.stats);

    merged.save_to_file(output)?;
    println!("\n   ✓ Merged spec written to {}", output.display());

    if merged.stats.conflicts_detected > 0 {
        let all_conflicts: Vec<_> = merged
            .components
            .iter()
            .flat_map(|c| c.props.iter().flat_map(|p| p.conflicts.iter().cloned()))
            .collect();
        let spec_json = serde_json::to_string_pretty(&merged)?;
        std::fs::create_dir_all(html_dir)?;
        let html = ucp_synthesizer::curation::generate_curation_html(
            &all_conflicts,
            &format!("Merged from {} specs", inputs.len()),
            "",
            &spec_json,
        )?;
        let html_path = PathBuf::from(html_dir).join("review.html");
        std::fs::write(&html_path, &html)?;
        println!("   ✓ Review UI written to {}", html_path.display());
    }

    println!("\n✅ Merge complete!");
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
    output: &ucp_synthesizer::pipeline::SynthesisOutput,
    output_dir: &str,
    source_label: &str,
) -> anyhow::Result<()> {
    let all_conflicts: Vec<_> = output
        .components
        .iter()
        .flat_map(|c| c.props.iter().flat_map(|p| p.conflicts.iter().cloned()))
        .collect();

    let html_path = PathBuf::from(output_dir).join("review.html");
    let html = if !all_conflicts.is_empty() {
        let spec_json = serde_json::to_string_pretty(output)?;
        ucp_synthesizer::curation::generate_curation_html(
            &all_conflicts,
            &format!("Source: {}", source_label),
            "",
            &spec_json,
        )?
    } else {
        format!(
            "<!DOCTYPE html><html><head><style>body{{font-family:sans-serif;max-width:600px;margin:40px auto;padding:0 20px;}}</style></head>\
            <body><h1>UCP v4.0 Curation UI</h1>\
            <p>✅ No conflicts detected across {} components.</p>\
            <p><a href=\"ucp-spec.json\">View full spec</a></p></body></html>",
            output.components.len()
        )
    };
    std::fs::write(&html_path, &html)?;
    println!("   ✓ Review UI written to {}", html_path.display());
    Ok(())
}
