//! Ratatui‑based interactive terminal UI for curation.

use crate::curate::{curate_spec, Resolution};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use ucp_core::cam::Conflict;
use ucp_core::Result;
use ucp_synthesizer::pipeline::SynthesisOutput;

/// Run the interactive curation TUI.
/// Run the interactive curation TUI over a merged spec.
///
/// # Errors
///
/// Errors when the terminal backend cannot be initialized (e.g. not a
/// TTY) or the event loop fails.
pub fn run_curation_tui(merged: &SynthesisOutput) -> Result<SynthesisOutput> {
    let conflicts: Vec<&Conflict> = merged
        .components
        .iter()
        .flat_map(|c| c.props.iter().flat_map(move |p| p.conflicts.iter()))
        .collect();

    if conflicts.is_empty() {
        println!("No conflicts to resolve.");
        return Ok(merged.clone());
    }

    // Setup terminal
    enable_raw_mode().map_err(ucp_core::UcpError::Io)?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).map_err(ucp_core::UcpError::Io)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(ucp_core::UcpError::Io)?;

    let mut resolutions: Vec<Resolution> = Vec::new();
    let mut current_conflict: usize = 0;

    let result = loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(f.area());

            // Title
            let title = Paragraph::new(Text::styled(
                format!("Conflict {}/{}", current_conflict + 1, conflicts.len()),
                Style::default().fg(Color::Yellow),
            ))
            .block(Block::default().borders(Borders::ALL).title("UCP Curate"));
            f.render_widget(title, chunks[0]);

            // Conflict details
            if let Some(conflict) = conflicts.get(current_conflict) {
                let details = format!(
                    "ID: {}\nField: {}\nPresent in: {}\nAbsent in: {}\nConfidence: {:.2}\nSuggestion: {:?}",
                    conflict.id,
                    conflict.field,
                    conflict.present_in.join(", "),
                    conflict.absent_in.join(", "),
                    conflict.confidence,
                    conflict.resolution_suggestion,
                );
                let details_widget = Paragraph::new(details)
                    .block(Block::default().borders(Borders::ALL).title("Details"));
                f.render_widget(details_widget, chunks[1]);
            }

            // Instructions
            let instructions = Paragraph::new(Text::styled(
                "← → navigate | a accept | r reject | s skip | q quit",
                Style::default().fg(Color::Gray),
            ));
            f.render_widget(instructions, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break finish(merged, &resolutions),
                KeyCode::Right | KeyCode::Char('n') => {
                    if current_conflict + 1 < conflicts.len() {
                        current_conflict += 1;
                    }
                }
                KeyCode::Left | KeyCode::Char('p') => {
                    current_conflict = current_conflict.saturating_sub(1);
                }
                KeyCode::Char('a') => {
                    if let Some(conflict) = conflicts.get(current_conflict) {
                        resolutions.push(Resolution {
                            conflict_id: conflict.id.clone(),
                            chosen_resolution: conflict.resolution_suggestion.clone(),
                            custom_rationale: Some("Accepted via TUI".into()),
                        });
                    }
                    if review_complete(current_conflict, conflicts.len()) {
                        break finish(merged, &resolutions);
                    }
                    current_conflict += 1;
                }
                KeyCode::Char('r' | 's') => {
                    // Reject / skip: leave the conflict unresolved, move on.
                    if review_complete(current_conflict, conflicts.len()) {
                        break finish(merged, &resolutions);
                    }
                    current_conflict += 1;
                }
                _ => {}
            }
        }
    };

    // Restore terminal
    disable_raw_mode().map_err(ucp_core::UcpError::Io)?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .map_err(ucp_core::UcpError::Io)?;
    terminal.show_cursor().map_err(ucp_core::UcpError::Io)?;

    result
}

/// True when `current` is the last conflict, i.e. the review session ends.
#[must_use]
fn review_complete(current: usize, total: usize) -> bool {
    current + 1 >= total
}

/// Apply the collected resolutions and produce the curated spec.
///
/// # Errors
///
/// Propagates errors from [`curate_spec`].
fn finish(merged: &SynthesisOutput, resolutions: &[Resolution]) -> Result<SynthesisOutput> {
    curate_spec(merged, resolutions)
}

#[cfg(test)]
mod tests {

    // TUI tests are integration tests and run in the tests/ directory
}
