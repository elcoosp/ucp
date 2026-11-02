#!/bin/bash

# Script complémentaire de commit automatique et conventionnel - Un commit par fichier
# Compatible Bash 3.2 (macOS) - Version nettoyée sans cassure Markdown

files=$(git status --porcelain | awk '{print $2}')

if [ -z "$files" ]; then
    echo "Aucun fichier restant modifie detecte."
    exit 0
fi

for file in $files; do
    if [ ! -f "$file" ]; then
        continue
    fi

    echo "Analyse et commit pour : $file"

    case "$file" in
        "ucp-cli/tests/cli.rs")
            subject="test(cli): deploy honest command line and validations integration tests"
            body="Verify process exit codes behaviors and ensure help messages match real offline tool capabilities."
            ;;
        "ucp-cli/tests/fixtures/sample-spec.json")
            subject="test(cli): create component mock spec fixture"
            body="Provide realistic component mock data for structured validate command verification pipelines."
            ;;
        "ucp-synthesizer/benches/extraction_bench.rs")
            subject="perf(synthesizer): bench tokenizer extraction and pipeline cold run latencies"
            body="Establish performance baseline tracking cold vs cached file scanner processing execution logs."
            ;;
        "ucp-synthesizer/benches/merge_bench.rs")
            subject="perf(synthesizer): bench cross framework specimen merge capabilities"
            body="Measure exact fnv1a clustering scaling costs when handling large components counts in memory."
            ;;
        "ucp-synthesizer/src/contract/ai_contract.rs" | "ucp-synthesizer/src/contract/mcp_server.rs" | "ucp-synthesizer/src/contract/mod.rs")
            subject="refactor(synthesizer): remove obsolete ai contract and mcp server submodules"
            body="Purge temporary speculative agent tools and hooks in line with zero llm architecture goals."
            ;;
        "ucp-synthesizer/src/curation.rs" | "ucp-synthesizer/src/dashboard/generator.rs" | "ucp-synthesizer/src/dashboard/mod.rs" | "ucp-synthesizer/src/dashboard/template.html")
            subject="refactor(synthesizer): prune deprecated html dashboard and curation elements"
            body="Drop unused browser layout visualizers to focus exclusively on fast offline static analysis."
            ;;
        "ucp-synthesizer/src/discovery.rs")
            subject="refactor(synthesizer): delete legacy network github discovery agent code"
            body="Remove unused octocrab repository web scrapers to enforce strict offline boundaries."
            ;;
        "ucp-synthesizer/src/export/a2ui.rs" | "ucp-synthesizer/src/export/ag_ui.rs")
            subject="refactor(synthesizer): remove deprecated catalog and event scheme exporters"
            body="Clean up output generation logic formats to retain only standard w3c design and tokens definitions."
            ;;
        "ucp-synthesizer/src/llm.rs" | "ucp-synthesizer/src/pipeline/enrichment.rs")
            subject="refactor(pipeline): strip old local ollama interaction modules"
            body="Purge inference prompts and request definitions ensuring identical input produces byte-identical specs."
            ;;
        "ucp-synthesizer/tests/curation.rs" | "ucp-synthesizer/tests/dashboard_test.rs" | "ucp-synthesizer/tests/llm_mock.rs")
            subject="test(synthesizer): delete deprecated validation tests for deleted layout tools"
            body="Prune obsolete integration test files tracking removed client surfaces."
            ;;
        *)
            subject="chore: update $(basename "$file")"
            body="Refactor or clean up internal logic structures within $file."
            ;;
    esac

    # Indexation temporaire, commit et nettoyage progressif de la file d'attente
    git add "$file"
    git commit -m "$subject" -m "$body"

    echo "✅ Commite : $file"
    echo "--------------------------------------------"
done

echo "Félicitations, tous les fichiers restants du dépôt ont été enregistrés individuellement."
