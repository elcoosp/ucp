# `ucp bootstrap`

Extract UI component definitions from a source directory.

## Usage

``` bash
ucp bootstrap --source-dir <PATH> [--output-dir <PATH>] [--ollama-url <URL>] [--llm-model <MODEL>] [--watch]
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--source-dir` | (required) | Directory containing component source files |
| `--output-dir` | `./ucp-output` | Where to write the spec |
| `--ollama-url` | `None` | Ollama instance for LLM enrichment |
| `--llm-model` | `glm-5:cloud` | Model name for LLM enrichment |
| `--watch` | `false` | Use watchexec for continuous bootstrap |

## Example

``` bash
ucp bootstrap --source-dir src --output-dir spec
```
