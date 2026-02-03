//! Minimal TUI diagnostic - tests if keyboard events work at all.
//!
//! Run with: cargo run --example tui_diag

use std::io;
use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use futures::StreamExt;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_diagnostic(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn run_diagnostic(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut event_stream = EventStream::new();
    let mut tick_timer = tokio::time::interval(Duration::from_millis(100));
    let mut messages: Vec<String> = vec![
        "=== TUI Diagnostic ===".to_string(),
        "Press any key to test. Press 'q' to quit.".to_string(),
        "".to_string(),
    ];
    let mut tick_count = 0u32;

    // Initial render
    render(terminal, &messages, tick_count)?;

    loop {
        tokio::select! {
            maybe_event = event_stream.next() => {
                match maybe_event {
                    Some(Ok(Event::Key(key))) => {
                        let msg = format!("[KEY] code={:?} modifiers={:?}", key.code, key.modifiers);
                        messages.push(msg);
                        if messages.len() > 20 {
                            messages.remove(3); // Keep header
                        }
                        if key.code == KeyCode::Char('q') {
                            messages.push("Quitting...".to_string());
                            render(terminal, &messages, tick_count)?;
                            break;
                        }
                        render(terminal, &messages, tick_count)?;
                    }
                    Some(Ok(Event::Resize(w, h))) => {
                        messages.push(format!("[RESIZE] {}x{}", w, h));
                        render(terminal, &messages, tick_count)?;
                    }
                    Some(Ok(Event::Mouse(m))) => {
                        messages.push(format!("[MOUSE] {:?}", m.kind));
                        render(terminal, &messages, tick_count)?;
                    }
                    Some(Ok(Event::FocusGained)) => {
                        messages.push("[FOCUS] gained".to_string());
                        render(terminal, &messages, tick_count)?;
                    }
                    Some(Ok(Event::FocusLost)) => {
                        messages.push("[FOCUS] lost".to_string());
                        render(terminal, &messages, tick_count)?;
                    }
                    Some(Ok(Event::Paste(s))) => {
                        messages.push(format!("[PASTE] {:?}", s));
                        render(terminal, &messages, tick_count)?;
                    }
                    Some(Err(e)) => {
                        messages.push(format!("[ERROR] {:?}", e));
                        render(terminal, &messages, tick_count)?;
                    }
                    None => {
                        messages.push("[STREAM] ended".to_string());
                        render(terminal, &messages, tick_count)?;
                        break;
                    }
                }
            }
            _ = tick_timer.tick() => {
                tick_count += 1;
                // Update tick counter in status bar
                render(terminal, &messages, tick_count)?;
            }
        }
    }

    Ok(())
}

fn render(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    messages: &[String],
    tick: u32,
) -> io::Result<()> {
    terminal.draw(|f| {
        let area = f.area();

        // Build lines
        let mut lines: Vec<Line> = messages
            .iter()
            .map(|m| Line::from(Span::styled(m.as_str(), Style::default().fg(Color::White))))
            .collect();

        // Add status line at bottom
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("Tick: {} | Event stream active | Press 'q' to quit", tick),
            Style::default().fg(Color::Cyan),
        )));

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, area);
    })?;
    Ok(())
}
