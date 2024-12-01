use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use std::io::{self};
use std::sync::{Arc, Mutex};

use crate::vm::VM;

pub fn run_tui(vm: Arc<Mutex<VM>>) -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut running = false;

    terminal.clear()?;

    Ok(loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(f.area());

            let left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(chunks[0]);

            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            let vm = vm.lock().unwrap();
            let (registers, memory, pc) = vm.get_state();

            let program_text: Vec<ratatui::prelude::Line> = vm
                .get_program()
                .iter()
                .enumerate()
                .map(|(idx, inst)| {
                    if idx == pc {
                        ratatui::prelude::Line::styled(
                            inst.clone(),
                            Style::default().fg(Color::Yellow).bg(Color::Blue),
                        )
                    } else {
                        ratatui::prelude::Line::from(inst.clone())
                    }
                })
                .collect();
            let program_panel = Paragraph::new(program_text)
                .block(Block::default().borders(Borders::ALL).title("Program"));

            let mut reg_text: Vec<ratatui::prelude::Line> = registers
                .iter()
                .map(|(reg, val)| ratatui::prelude::Line::from(format!("{}: {}\n", reg, val)))
                .collect();
            reg_text.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
            let registers_panel = Paragraph::new(reg_text)
                .block(Block::default().borders(Borders::ALL).title("Registers"));

            let mem_text: Vec<ratatui::prelude::Line> = memory
                .chunks(8)
                .enumerate()
                .map(|(i, chunk)| {
                    let address = format!("0x{:04X}: ", i * 8);

                    // Split chunk into two groups of 4 bytes each
                    let (left, right) = chunk.split_at(4);

                    // Format hex values
                    let hex_values: String = [left.iter(), right.iter()]
                        .iter()
                        .map(|group| {
                            group
                                .clone()
                                .enumerate()
                                .map(|(j, val)| {
                                    let separator = if j == 3 { "  " } else { " " };
                                    format!("{:02X}{}", val, separator)
                                })
                                .collect::<String>()
                        })
                        .collect::<Vec<String>>()
                        .join("");

                    // Format ASCII values
                    let ascii_values: String = [left.iter(), right.iter()]
                        .iter()
                        .map(|group| {
                            group
                                .clone()
                                .map(|&val| {
                                    if (val as u8).is_ascii_graphic() {
                                        (val as u8) as char
                                    } else {
                                        '.'
                                    }
                                })
                                .collect::<String>()
                        })
                        .collect::<Vec<String>>()
                        .join("  ");

                    let full_line = format!("{}{}|{}|", address, hex_values, ascii_values);
                    ratatui::prelude::Line::from(full_line)
                })
                .collect();

            let memory_panel = Paragraph::new(mem_text)
                .block(Block::default().borders(Borders::ALL).title("Memory"));

            f.render_widget(program_panel, left_chunks[0]);
            f.render_widget(registers_panel, right_chunks[0]);
            f.render_widget(memory_panel, right_chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Char('s') => {
                            let mut vm = vm.lock().unwrap();
                            vm.step();
                        }
                        KeyCode::Char('r') => {
                            running = !running;
                        }
                        _ => {}
                    }
                }
            }
        }

        if running {
            let mut vm = vm.lock().unwrap();
            if !vm.step() {
                running = false;
            }
        }
    })
}
