# CAM to W3C UI Specification Schema Mapping

## Overview

This document describes how UCP's Canonical Abstract Model (CAM) maps to the
W3C UI Specification Schema Community Group draft.

## Field Mapping

### Top-Level

| CAM Field | W3C Field | Notes |
|-----------|-----------|-------|
| `id` | `component.id` | Direct mapping |
| `id` (base name) | `component.name` | Extracted from the last segment after `:` |

### Props

| CAM Field | W3C Field | Notes |
|-----------|-----------|-------|
| `canonical_name` | `prop.name` | Direct mapping |
| `abstract_type` | `prop.type` | Serialized as lowercase string (e.g., "controlflag", "staticvalue") |
| `reactivity != Static` | `prop.required` | A prop is considered required unless it is Static or has a default value |
| `concrete_type` | `prop.description` | Provides human-readable original type (e.g., "bool", "String") |

### Events

| CAM Field | W3C Field | Notes |
|-----------|-----------|-------|
| `canonical_name` | `event.name` | Direct mapping |
| `abstract_payload` | `event.payload` | Serialized as string representation |

### State Machines

| CAM Field | W3C Field | Notes |
|-----------|-----------|-------|
| `extracted_state_machine` | *No direct equivalent* | State machines are not yet represented in the W3C draft |

### Context

| CAM Field | W3C Field | Notes |
|-----------|-----------|-------|
| `provided_context` | *No direct equivalent* | May be mapped to a custom extension |
| `consumed_contexts` | *No direct equivalent* | May be mapped to a custom extension |

## Usage

```bash
# Extract and produce both standard and W3C output
ucp bootstrap --source-dir ./src --output-dir ./out --w3c

# The W3C spec is written as ucp-spec.w3c.json alongside ucp-spec.json
```
