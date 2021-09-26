#[allow(dead_code)]
mod event;
mod tabstate;
mod utils;

use event::{Event, Events};
use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};

#[derive(Deserialize)]
struct AppConfig {
    tabs: Vec<TabConfig>,
}

#[derive(Deserialize)]
struct TabConfig {
    name: String,
    command: String,
    color: Option<String>,
    env: Vec<EnvKey>,
    args: Vec<String>,
}

#[derive(Deserialize)]
struct EnvKey {
    key: String,
    value: String,
}

struct App {
    tabs: tabstate::TabsState,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("config.yml").expect("Something went wrong reading the file");
    let config: AppConfig = serde_yaml::from_str(&contents).unwrap();

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut tabs: Vec<String> = Vec::new();
    for tab in config.tabs.iter() {
        tabs.push(tab.name.to_string());
    }
    // App
    let mut app = App {
        tabs: tabstate::TabsState::new(tabs),
    };

    // Main loop
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let block = Block::default().style(Style::default().fg(Color::Black));
            f.render_widget(block, size);

            let titles = app
                .tabs
                .titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(first, Style::default().fg(Color::Yellow)),
                        Span::styled(rest, Style::default().fg(Color::Green)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.tabs.index)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::Black),
                );
            f.render_widget(tabs, chunks[0]);

            let tab = &config.tabs[app.tabs.index];

            let mut init = Command::new(tab.command.to_string());
            let mut cmd = init.args(tab.args.iter());
            for env in tab.env.iter() {
                cmd = cmd.env(env.key.to_string(), env.value.to_string());
            }
            let run = cmd.output().expect("failed to execute process");
            let output = String::from_utf8(run.stdout).unwrap();

            let block = Block::default()
                .title(app.tabs.titles[app.tabs.index].to_string())
                .style(Style::default().fg(Color::Cyan))
                .borders(Borders::ALL);
            let paragraph = Paragraph::new(output)
                .style(Style::default().fg(utils::color_from_str(&tab.color)))
                .block(block);
            f.render_widget(paragraph, chunks[1]);
        })?;

        loop {
            if let Event::Input(input) = events.next()? {
                match input {
                    Key::Char('q') => {
                        return Ok(());
                    }
                    Key::Right => {
                        app.tabs.next();
                        break;
                    }
                    Key::Left => {
                        app.tabs.previous();
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}
