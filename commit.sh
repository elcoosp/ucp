#!/bin/bash

# Script de commit automatique et conventionnel - Un commit par fichier
# Compatible Bash 3.2 (macOS) - Version nettoyée sans cassure Markdown

files=$(git status --porcelain | awk '{print $2}')

if [ -z "$files" ]; then
    echo "Aucun fichier modifie detecte."
    exit 0
fi

for file in $files; do
    if [ ! -f "$file" ]; then
        continue
    fi

    echo "Analyse et commit pour : $file"

    case "$file" in
        ".gitignore")
            subject="chore: ignore ucp output directory"
            body="Add ucp-output pattern to prevent tracking of local spec generation artifacts."
            ;;
        ".just-e2e/test_e2e.sh")
            subject="test(e2e): update core validation matchers"
            body="Align end-to-end assertions with the updated component spacing and spec is valid output strings."
            ;;
        ".just-e2e/test_e2e_conflict.sh")
            subject="test(e2e): refine conflict regex output checks"
            body="Update validation step to support new conflict format printouts."
            ;;
        ".just-e2e/test_e2e_curate.sh")
            subject="test(e2e): match validation format inside curate checks"
            body="Support unresolved conflicts and structural metrics updates in curation tests."
            ;;
        ".just-e2e/test_perf.sh")
            subject="test(perf): adjust log matchers inside performance run script"
            body="Sync spacing format for files parsed outputs inside execution logs."
            ;;
        "CHANGELOG.md")
            subject="docs(changelog): draft release notes for version 0.13.0"
            body="Document tokenization overhauls type ontology extensions and vaporware removal."
            ;;
        "Cargo.lock")
            subject="chore: update lockfile dependencies for criterion benches"
            body="Synchronize workspace packages adding alloca anes cast ciborium and itertools."
            ;;
        "Cargo.toml")
            subject="chore: bump workspace version to 0.13.0"
            body="Elevate repository crates baseline version specification to 0.13.0."
            ;;
        "README.md")
            subject="docs(readme): rewrite introduction for deterministic static analysis"
            body="Overhaul content removing AI references to focus entirely on framework API diff capabilities."
            ;;
        "ROADMAP.md")
            subject="docs(roadmap): simplify future steps around static analysis parity"
            body="Prune dynamic agent visions replacing them with tangible vue and svelte extractor goals."
            ;;
        "SECURITY.md")
            subject="docs(security): update policies enforcing total offline boundaries"
            body="Clarify that zero network connections happen during parsing since the removal of ollama."
            ;;
        "TRANSFORMATION.md")
            subject="docs(maintainer): document the 0.13 architectural pivot"
            body="Add record tracking why vaporware was deleted and how correctness was achieved."
            ;;
        "docs/maintainer/architecture.md")
            subject="docs(architecture): strip layout references to ollama and custom exporters"
            body="Align design schemas with a clean static unifier flow."
            ;;
        "docs/maintainer/pipeline.md")
            subject="docs(pipeline): document incremental mtime cache layer"
            body="Add specification text tracking how files are served from the statistics cache."
            ;;
        "docs/plans/v0.13.0/spec.md")
            subject="docs(plans): retract legacy agent server blueprints"
            body="Mark vaporware plans as explicitly rejected in favor of static correctness."
            ;;
        "docs/user/commands/bootstrap.md")
            subject="docs(commands): document json output flag inside bootstrap specs"
            body="Remove old llm model options and watch mode parameters from the reference table."
            ;;
        "docs/user/commands/index.md")
            subject="docs(commands): prune deleted commands from index listing"
            body="Strip contract mcp and dashboard references to clean up the user index."
            ;;
        "docs/user/tutorial-bootstrap.md")
            subject="docs(tutorial): convert bootstrap steps to focus on json flags"
            body="Replace old local ollama instance requirements with machine summary patterns."
            ;;
        "docs/user/tutorial-export.md")
            subject="docs(tutorial): remove deprecated a2ui and agui export outputs"
            body="Sync export folder visualization lists with modern unifier formats."
            ;;
        "ucp-cli/Cargo.toml")
            subject="chore(cli): inject tempfile into dev dependencies scope"
            body="Provide file creation support inside structural cli test cases."
            ;;
        "ucp-cli/src/main.rs")
            subject="refactor(cli): clean up main entry subcommands map definitions"
            body="Strip old mcp dashboard and contract routes implementing raw json validate returns."
            ;;
        "ucp-cli/tests/cli.rs")
            subject="test(cli): implement structural validate and bootstrap integration tests"
            body="Verify exit codes behaviors and ensure about documentation matches real tools."
            ;;
        "ucp-cli/tests/fixtures/sample-spec.json")
            subject="test(cli): write static sample fixture manifest"
            body="Provide realistic component mock data for structural validate command verification."
            ;;
        "ucp-core/Cargo.toml")
            subject="chore(core): add proptest dependency requirement"
            body="Provide property testing framework support inside names processing layers."
            ;;
        "ucp-core/src/lib.rs")
            subject="refactor(core): expose naming normalization and validation modules"
            body="Stabilize library facade export tracks for the synthesizer layer."
            ;;
        "ucp-core/src/naming.rs")
            subject="feat(core): implement cross convention identifier normalization"
            body="Add letter folding and event stem stripping to bridge ports convention gaps."
            ;;
        "ucp-core/src/validation.rs")
            subject="feat(core): design structural component validation rules"
            body="Provide duplicate id checks and confidence out of bounds tracking logic."
            ;;
        "ucp-core/tests/naming.proptest-regressions")
            subject="test(core): seed property test naming regressions ledger"
            body="Track previously shrunk failure tokens to prevent normalization edge regressions."
            ;;
        "ucp-core/tests/naming.rs")
            subject="test(core): deploy idempotent property tests for naming normalization"
            body="Assert convergence across snake camel and kebab convention prop variations."
            ;;
        "ucp-core/tests/validation.rs")
            subject="test(core): test spec soundness error checking routes"
            body="Verify that duplicate ids fail validation while empty component lists emit warnings."
            ;;
        "ucp-maintainer/src/curate.rs" | "ucp-maintainer/src/diff.rs" | "ucp-maintainer/src/registry.rs" | "ucp-maintainer/src/verify.rs")
            subject="refactor(maintainer): replace llm enriched flag with cache hits counter"
            body="Sync database and inspection schemas with the modern pipeline output formats."
            ;;
        "ucp-maintainer/src/watch.rs")
            subject="refactor(maintainer): bind incremental spec cache inside file change watch loop"
            body="Prune deprecated a2ui and contract calls from the automatic re export chain."
            ;;
        "ucp-maintainer/tests/curation.rs" | "ucp-maintainer/tests/diff.rs" | "ucp-maintainer/tests/registry.rs" | "ucp-maintainer/tests/verify.rs")
            subject="test(maintainer): patch test specification instantiations for cache stats"
            body="Align test fixtures with the newly defaulted metrics entries."
            ;;
        "ucp-synthesizer/Cargo.toml")
            subject="chore(synthesizer): configure extraction and merge bench targets"
            body="Activate criterion harness configurations for performance profiling."
            ;;
        "ucp-synthesizer/benches/extraction_bench.rs")
            subject="perf(synthesizer): bench tokenizer extraction and pipeline speeds"
            body="Establish performance baseline tracking cold vs cached execution latencies."
            ;;
        "ucp-synthesizer/benches/merge_bench.rs")
            subject="perf(synthesizer): bench cross framework specimen merge scaling"
            body="Measure exact fnv1a clustering costs when handling large component counts."
            ;;
        "ucp-synthesizer/src/extract/rust_ast.rs")
            subject="refactor(extract): derive serialization traits for raw rust definitions"
            body="Enable disk caching for intermediate structure extractions."
            ;;
        "ucp-synthesizer/src/extract/tsx_ast.rs")
            subject="feat(extract): implement tokenizer based tsx interface scanner"
            body="Lex text code into explicit streams avoiding prop swallow bugs on inline arrow boundaries."
            ;;
        "ucp-synthesizer/src/lib.rs")
            subject="refactor(synthesizer): strip old curation contract and dashboard submodules"
            body="Align crate structure with the zero llm offline processing philosophy."
            ;;
        "ucp-synthesizer/src/merge.rs")
            subject="feat(merge): attach presence drift conflicts inside specs unifier"
            body="Flag props that exist in only some source framework ports to identify silent drift."
            ;;
        "ucp-synthesizer/src/pipeline/cache.rs")
        subject="feat(pipeline): create mtime and size file extraction cache layers"
        body="Provide best effort disk persistence tracking unchanged file stats to skip re parsing."
        ;;
        "ucp-synthesizer/src/pipeline/extraction.rs")
        subject="refactor(pipeline): route file parsing passes through the parse cache"
        body="Optimize scheduler throughput to skip unchanged items entirely in watch and CI modes."
        ;;
        "ucp-synthesizer/src/pipeline/output.rs")
        subject="refactor(pipeline): default cache hits inside statistics structures"
        body="Expose backing fields supporting back-compatibility during old spec loads."
        ;;
        "ucp-synthesizer/src/pipeline/unification.rs")
        subject="refactor(pipeline): switch clustering hashes to fold component names"
        body="Let framework ports with distinct prop sets cluster together to reveal API drifts."
        ;;
        "ucp-synthesizer/src/unify.rs")
        subject="feat(unify): extend typescript type ontology mappings"
        body="Map nullable type unions suffix arrays and react namespaces into canonical types."
        ;;
        "ucp-synthesizer/tests/cache.rs")
        subject="test(synthesizer): verify incremental parse cache lookups"
        body="Confirm that modified files get re parsed while unchanged file entries hit cache."
        ;;
        "ucp-synthesizer/tests/common/mod.rs" | "ucp-synthesizer/tests/dioxus_merge_context.rs" | "ucp-synthesizer/tests/incremental_merge.rs" | "ucp-synthesizer/tests/proptest_conflicts.rs")
        subject="test(synthesizer): update common mock macros for pipeline statistics"
        body="Ensure old tests inject defaulted cache fields correctly."
        ;;
        "ucp-synthesizer/tests/cross_framework_merge.rs")
        subject="test(synthesizer): verify cross framework button unification properties"
        body="Assert that react tsx and leptos ports combine perfectly flagging presence drifts."
        ;;
        "ucp-synthesizer/tests/export_snapshots.rs")
        subject="test(synthesizer): trim snapshot tests to match remaining formats"
        body="Prune deleted contract and a2ui targets from macro lookups."
        ;;
        "ucp-synthesizer/tests/pipeline_integration.rs")
        subject="test(synthesizer): patch line counters metrics in integration tests"
        body="Confirm real line starts are evaluated inside raw extractions."
        ;;
        "ucp-synthesizer/tests/tsx_extraction.rs")
        subject="test(synthesizer): adapt old tests type spacing definitions"
        body="Sync types string matching with the modern whitespace-stripped format."
        ;;
        "ucp-synthesizer/tests/tsx_extraction_v2.rs")
        subject="test(synthesizer): write comprehensive tests for token based tsx scanners"
        body="Verify arrow type survival comment exclusions and newline member processing."
        ;;
        "ucp-synthesizer/tests/v030_integration.rs" | "ucp-synthesizer/tests/v060_integration.rs" | "ucp-synthesizer/tests/v090_integration.rs")
        subject="test(synthesizer): strip out of date format exports from integration tests"
        body="Clean up legacy validation checks matching the simplified format registry."
        ;;
        *)
        subject="chore: update $(basename "$file")"
        body="Refactor or clean up internal logic structures within $file."
        ;;
        esac
        git add "$file"
        git commit -m "$subject" -m "$body"
        echo "✅ Commite : $file"
        echo "--------------------------------------------"
        done
        echo "Traitement du depot termine avec succes !"
