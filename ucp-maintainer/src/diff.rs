//! Structural diff between two `SynthesisOutput` specs.

use std::fmt::Write as _;

use serde::Serialize;
use ucp_core::Result;
use ucp_synthesizer::pipeline::SynthesisOutput;

/// Top-level report of a two-spec structural diff.
#[derive(Debug, Clone, Serialize)]
pub struct DiffReport {
    /// Component ids present in `b` but not in `a`.
    pub added_components: Vec<String>,
    /// Component ids present in `a` but not in `b`.
    pub removed_components: Vec<String>,
    /// Per-component prop-level differences.
    pub changed_components: Vec<ComponentDiff>,
}

/// Prop-level changes for one component.
#[derive(Debug, Clone, Serialize)]
pub struct ComponentDiff {
    /// CAM id of the component that changed.
    pub component_id: String,
    /// Props added in `b`.
    pub added_props: Vec<PropChange>,
    /// Props removed in `b`.
    pub removed_props: Vec<PropChange>,
    /// Props whose type changed between `a` and `b`.
    pub changed_props: Vec<PropChange>,
}

/// One added/removed/changed prop.
#[derive(Debug, Clone, Serialize)]
pub struct PropChange {
    /// Canonical prop name.
    pub prop_name: String,
    /// Type rendering in the old spec (`None` when the prop is new).
    pub old_value: Option<String>,
    /// Type rendering in the new spec (`None` when the prop was removed).
    pub new_value: Option<String>,
}

/// Compare two specs structurally and return a diff report.
///
/// # Example
/// ```rust
/// use ucp_maintainer::diff::diff_specs;
/// use ucp_synthesizer::pipeline::{SynthesisOutput, PipelineStats};
/// let a = SynthesisOutput {
///     ucp_version: "4.0.0".into(),
///     components: vec![],
///     provenance: None,
///     curation_log: None,
///     stats: PipelineStats {
///         files_scanned: 0,
///         files_parsed: 0,
///         components_found: 0,
///         conflicts_detected: 0,
///         cache_hits: 0,
///     },
/// };
/// let b = a.clone();
/// let report = diff_specs(&a, &b).unwrap();
/// assert!(report.added_components.is_empty());
/// ```
/// # Errors
///
/// Never errors in practice; reserved for future fallible diff backends.
pub fn diff_specs(spec_a: &SynthesisOutput, spec_b: &SynthesisOutput) -> Result<DiffReport> {
    use std::collections::HashMap;
    let comps_a: HashMap<&str, _> = spec_a
        .components
        .iter()
        .map(|c| (c.id.as_str(), c))
        .collect();
    let comps_b: HashMap<&str, _> = spec_b
        .components
        .iter()
        .map(|c| (c.id.as_str(), c))
        .collect();

    let added_components: Vec<String> = comps_b
        .keys()
        .filter(|k| !comps_a.contains_key(*k))
        .map(std::string::ToString::to_string)
        .collect();
    let removed_components: Vec<String> = comps_a
        .keys()
        .filter(|k| !comps_b.contains_key(*k))
        .map(std::string::ToString::to_string)
        .collect();

    let mut changed_components = Vec::new();
    for (id, comp_a) in &comps_a {
        if let Some(comp_b) = comps_b.get(id) {
            let props_a: HashMap<&str, _> = comp_a
                .props
                .iter()
                .map(|p| (p.canonical_name.as_str(), p))
                .collect();
            let props_b: HashMap<&str, _> = comp_b
                .props
                .iter()
                .map(|p| (p.canonical_name.as_str(), p))
                .collect();

            let added_props: Vec<_> = props_b
                .keys()
                .filter(|k| !props_a.contains_key(*k))
                .map(|k| PropChange {
                    prop_name: k.to_string(),
                    old_value: None,
                    new_value: props_b[k].concrete_type.clone(),
                })
                .collect();
            let removed_props: Vec<_> = props_a
                .keys()
                .filter(|k| !props_b.contains_key(*k))
                .map(|k| PropChange {
                    prop_name: k.to_string(),
                    old_value: props_a[k].concrete_type.clone(),
                    new_value: None,
                })
                .collect();
            let changed_props: Vec<_> = props_a
                .iter()
                .filter_map(|(name, pa)| {
                    props_b.get(name).and_then(|pb| {
                        let ta = format!("{:?}", pa.abstract_type);
                        let tb = format!("{:?}", pb.abstract_type);
                        if ta != tb || pa.concrete_type != pb.concrete_type {
                            Some(PropChange {
                                prop_name: name.to_string(),
                                old_value: Some(format!(
                                    "abstract={}, concrete={:?}",
                                    ta, pa.concrete_type
                                )),
                                new_value: Some(format!(
                                    "abstract={}, concrete={:?}",
                                    tb, pb.concrete_type
                                )),
                            })
                        } else {
                            None
                        }
                    })
                })
                .collect();

            if !added_props.is_empty() || !removed_props.is_empty() || !changed_props.is_empty() {
                changed_components.push(ComponentDiff {
                    component_id: id.to_string(),
                    added_props,
                    removed_props,
                    changed_props,
                });
            }
        }
    }
    Ok(DiffReport {
        added_components,
        removed_components,
        changed_components,
    })
}

/// Pretty‑print a diff report as human‑readable text.
/// Render a human-readable text diff of two specs.
///
/// # Errors
///
/// Never errors in practice; reserved for future fallible renderers.
pub fn diff_specs_text(spec_a: &SynthesisOutput, spec_b: &SynthesisOutput) -> Result<String> {
    let report = diff_specs(spec_a, spec_b)?;
    let mut out = String::new();
    if report.added_components.is_empty()
        && report.removed_components.is_empty()
        && report.changed_components.is_empty()
    {
        out.push_str("No differences found.\n");
        return Ok(out);
    }
    for c in &report.added_components {
        let _ = writeln!(out, "+ Component: {c}");
    }
    for c in &report.removed_components {
        let _ = writeln!(out, "- Component: {c}");
    }
    for cd in &report.changed_components {
        let _ = writeln!(out, "~ Component: {}", cd.component_id);
        for p in &cd.added_props {
            let _ = writeln!(out, "  + Prop: {} -> {:?}", p.prop_name, p.new_value);
        }
        for p in &cd.removed_props {
            let _ = writeln!(out, "  - Prop: {} (was {:?})", p.prop_name, p.old_value);
        }
        for p in &cd.changed_props {
            let _ = writeln!(
                out,
                "  ~ Prop: {} ({} -> {})",
                p.prop_name,
                p.old_value.as_deref().unwrap_or(""),
                p.new_value.as_deref().unwrap_or("")
            );
        }
    }
    Ok(out)
}
