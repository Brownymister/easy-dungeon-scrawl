use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{alloc::System, fs};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

mod custom_layer;
mod game;
mod map_gen;
use game::*;

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug, PartialEq, Clone)]
enum MenuItem {
    Game,
    Help,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    let mut global_game = Game::new();
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let mut active_menu_item = MenuItem::Game;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Min(2), Constraint::Length(5)].as_ref())
                .split(size);

            let start = SystemTime::now();
            let time = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();
            if time - global_game.info_message.time > 1000 && global_game.info_message.time != 0 {
                global_game.info_message = InfoMessage {
                    title: "".to_string(),
                    message: "".to_string(),
                    time: 0,
                }
            }

            let info_widget = Paragraph::new(global_game.info_message.message.as_str())
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title(global_game.info_message.title.as_str())
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(info_widget, chunks[1]);
            match active_menu_item {
                MenuItem::Game => rect.render_widget(render_home(&global_game), chunks[0]),
                MenuItem::Help => {
                    rect.render_widget(render_help(), chunks[0]);
                }
            }
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('w') => global_game.north(),
                KeyCode::Up => global_game.north(),
                KeyCode::Char('a') => global_game.west(),
                KeyCode::Left => global_game.west(),
                KeyCode::Char('s') => global_game.south(),
                KeyCode::Down => global_game.south(),
                KeyCode::Char('d') => global_game.east(),
                KeyCode::Right => global_game.east(),
                KeyCode::Char('i') => {
                    if active_menu_item == MenuItem::Game {
                        active_menu_item = MenuItem::Help;
                    } else {
                        active_menu_item = MenuItem::Game;
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn render_home<'a>(global_game: &'a Game) -> Paragraph<'a> {
    let map_str = map_gen::visulize_map(&global_game.cur_map, Some(&global_game.pos));
    return get_map_as_paragraph(map_str)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                // .title("Home")
                .border_type(BorderType::Plain),
        );
}

fn render_help<'a>() -> Paragraph<'a> {
    return Paragraph::new(vec![
        Spans::from(vec![Span::styled(
            "Welcome to",
            Style::default().fg(Color::Red),
        )]),
        Spans::from(vec![Span::styled(
            "Easy Dungeon Scrawl",
            Style::default().fg(Color::Green),
        )]),
        Spans::from(vec![Span::styled(
            "Simple rpg dungean scral Game.",
            Style::default().fg(Color::Blue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Controls",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::raw("w - move north")]),
        Spans::from(vec![Span::raw("a - move west")]),
        Spans::from(vec![Span::raw("s - move south")]),
        Spans::from(vec![Span::raw("d - move east")]),
        Spans::from(vec![Span::raw("i - toggle help")]),
        Spans::from(vec![Span::raw("q - quit")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            // .title("Home")
            .border_type(BorderType::Plain),
    );
}

fn get_map_as_paragraph(map: String) -> Paragraph<'static> {
    let mut map_spans = vec![];
    for line in map.lines() {
        map_spans.push(Spans::from(vec![Span::raw(line.to_string())]));
    }
    return Paragraph::new(map_spans);
}
