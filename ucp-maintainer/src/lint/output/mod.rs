//! Output formatters for lint results.

use crate::lint::rules::LintResult;

pub trait OutputFormatter {
    fn format(&self, result: &LintResult) -> String;
}
