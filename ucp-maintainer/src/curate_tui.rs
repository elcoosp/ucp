//! Ratatui‑based interactive terminal UI for curation.

use crate::curate::{curate_spec, Resolution};
use ucp_core::Result;
use ucp_core::cam::Conflict;
use ucp_synthesizer::pipeline::SynthesisOutput;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

/// Run the interactive curation TUI.
pub fn run_curation_tui(
    merged: &SynthesisOutput,
) -> Result<SynthesisOutput> {
    let conflicts: Vec<&Conflict> = merged
        .components
        .iter()
        .flat_map(|c| {
            c.props.iter().flat_map(move |p| {
                p.conflicts.iter().map(move |conf| conf)
            })
        })
        .collect();

    if conflicts.is_empty() {
        println!("No conflicts to resolve.");
        return Ok(merged.clone());
    }

    // Setup terminal
    enable_raw_mode().map_err(|e| ucp_core::UcpError::Io(e))?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .map_err(|e| ucp_core::UcpError::Io(e))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|e| ucp_core::UcpError::Io(e))?;

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
        }).map_err(|e| ucp_core::UcpError::Io(e))?;

        if let Event::Key(key) = event::read().map_err(|e| ucp_core::UcpError::Io(e))? {
            match key.code {
                KeyCode::Char('q') => {
                    break Ok(curate_spec(merged, &resolutions)?);
                }
                KeyCode::Right | KeyCode::Char('n') => {
                    if current_conflict + 1 < conflicts.len() {
                        current_conflict += 1;
                    }
                }
                KeyCode::Left | KeyCode::Char('p') => {
                    if current_conflict > 0 {
                        current_conflict -= 1;
                    }
                }
                KeyCode::Char('a') => {
                    if let Some(conflict) = conflicts.get(current_conflict) {
                        resolutions.push(Resolution {
                            conflict_id: conflict.id.clone(),
                            chosen_resolution: conflict.resolution_suggestion.clone(),
                            custom_rationale: Some("Accepted via TUI".into()),
                        });
                        if current_conflict + 1 < conflicts.len() {
                            current_conflict += 1;
                        } else {
                            break Ok(curate_spec(merged, &resolutions)?);
                        }
                    }
                }
                KeyCode::Char('r') => {
                    if let Some(_conflict) = conflicts.get(current_conflict) {
                        // Reject the suggestion – leave conflict unresolved
                        // Just skip to next
                        if current_conflict + 1 < conflicts.len() {
                            current_conflict += 1;
                        } else {
                            break Ok(curate_spec(merged, &resolutions)?);
                        }
                    }
                }
                KeyCode::Char('s') => {
                    if current_conflict + 1 < conflicts.len() {
                        current_conflict += 1;
                    } else {
                        break Ok(curate_spec(merged, &resolutions)?);
                    }
                }
                _ => {}
            }
        }
    };

    // Restore terminal
    disable_raw_mode().map_err(|e| ucp_core::UcpError::Io(e))?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .map_err(|e| ucp_core::UcpError::Io(e))?;
    terminal.show_cursor().map_err(|e| ucp_core::UcpError::Io(e))?;

    result
}

#[cfg(test)]
mod tests {
    
    // TUI tests are integration tests and run in the tests/ directory
}
