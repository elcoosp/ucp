use anyhow::Context;
use clap::{Parser, Subcommand};
use ucp_synthesizer::curation::generate_curation_html;
use ucp_synthesizer::pipeline::run_pipeline;

#[derive(Parser)]
#[command(name = "ucp", about = "UCP v4.0 AI Unification Engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the full AI synthesis pipeline
    Bootstrap {
        /// Local directory containing source files (.rs, .tsx)
        #[arg(long)]
        source_dir: String,

        /// Output directory for generated spec and HTML
        #[arg(long, default_value = "./ucp-output")]
        output_dir: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bootstrap { source_dir, output_dir } => {
            println!("🔍 Scanning {}...", source_dir);

            let output = run_pipeline(&source_dir)
                .context("Pipeline failed")?;

            println!("   📁 Files scanned: {}", output.stats.files_scanned);
            println!("   📄 Files parsed:   {}", output.stats.files_parsed);
            println!("   🧩 Components:     {}", output.stats.components_found);
            println!("   ⚠️  Conflicts:      {}", output.stats.conflicts_detected);

            std::fs::create_dir_all(&output_dir)
                .context("Failed to create output directory")?;

            // Write UCP spec JSON
            let spec_json = serde_json::to_string_pretty(&output)?;
            let spec_path = format!("{}/ucp-spec.json", output_dir);
            std::fs::write(&spec_path, &spec_json)?;
            println!("\n   ✓ Spec written to {}", spec_path);

            // Write curation HTML if there are components with conflicts
            let all_conflicts: Vec<_> = output
                .components
                .iter()
                .flat_map(|c| c.props.iter().flat_map(|p| p.conflicts.iter().cloned()))
                .collect();

            let html_path = format!("{}/review.html", output_dir);
            let html = if !all_conflicts.is_empty() {
                generate_curation_html(
                    &all_conflicts,
                    &format!("Source: {}", source_dir),
                    "",
                    &spec_json,
                )?
            } else {
                format!(
                    "<!DOCTYPE html><html><head><style>body{{font-family:sans-serif;max-width:600px;margin:40px auto;padding:0 20px}}</style></head>\
                    <body><h1>UCP v4.0 Curation UI</h1>\
                    <p>✅ No conflicts detected across {} components.</p>\
                    <p><a href=\"ucp-spec.json\">View full spec</a></p></body></html>",
                    output.components.len()
                )
            };
            std::fs::write(&html_path, &html)?;
            println!("   ✓ Review UI written to {}", html_path);

            if output.components.is_empty() {
                println!("\n⚠️  No components found. Make sure --source-dir contains .rs or .tsx files with component definitions.");
            } else {
                println!("\n✅ Synthesis complete!");
            }
        }
    }

    Ok(())
}
