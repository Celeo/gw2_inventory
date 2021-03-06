use crate::{
    models::FullItem,
    util::{filter, paginate},
};
use anyhow::Result;
use log::error;
use std::{io, sync::mpsc, thread, time::Duration};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

const TICK_RATE: Duration = Duration::from_millis(100);
const EXIT_KEY: Key = Key::Alt('q');

struct App {
    items: Vec<FullItem>,
    input: String,
    page: usize,
    page_size: u16,
}

impl App {
    fn new(items: Vec<FullItem>) -> Self {
        Self {
            items,
            input: String::new(),
            page: 0,
            page_size: 0,
        }
    }

    fn get_all_slots_count(&self) -> usize {
        self.items.len()
    }

    fn search_status(&self) -> String {
        format!(
            "Page {}/{}, showing {} per page, {} total items",
            self.page + 1,
            self.max_pages(),
            self.page_size,
            self.get_all_slots_count()
        )
    }

    fn max_pages(&self) -> usize {
        ((self.get_all_slots_count() as f64) / (self.page_size as f64)).ceil() as usize
    }

    fn filtered_items(&self) -> Vec<String> {
        let filtered: Vec<_> = filter(&self.items, &self.input)
            .iter()
            .map(|&i| i.clone())
            .collect();
        let on_page = paginate(&filtered, self.page, self.page_size);
        on_page
            .iter()
            .map(|item| format!("{} (x{}) - {}", item.name, item.count, item.character))
            .collect()
    }
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug)]
struct Events {
    rx: mpsc::Receiver<Event<Key>>,
}

impl Events {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let tx_ = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                if let Ok(key) = evt {
                    if let Err(err) = tx_.send(Event::Input(key)) {
                        error!("{}", err);
                        return;
                    }
                    if key == EXIT_KEY {
                        return;
                    }
                }
            }
        });
        thread::spawn(move || loop {
            if tx.send(Event::Tick).is_err() {
                break;
            }
            thread::sleep(TICK_RATE);
        });
        Events { rx }
    }

    fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}

pub fn run(items: Vec<FullItem>) -> Result<()> {
    // ===========
    //    Setup
    // ===========
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();
    let mut app = App::new(items);

    loop {
        // ============
        //    Render
        // ============
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            app.page_size = chunks[2].height - 2;

            let mut msg = Text::from(Span::raw("Type to filter, use Alt+Q to exit"));
            msg.patch_style(Style::default());
            let header = Paragraph::new(msg);
            f.render_widget(header, chunks[0]);

            let search_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);

            let input = Paragraph::new(app.input.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, search_chunks[0]);

            let status_msg = Paragraph::new(app.search_status())
                .style(Style::default())
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(status_msg, search_chunks[1]);

            let filtered_items = app.filtered_items();
            let list_items: Vec<_> = filtered_items
                .iter()
                .map(|s| ListItem::new(Span::raw(s)))
                .collect();
            let list_items_wrapper =
                List::new(list_items).block(Block::default().borders(Borders::ALL).title("Items"));
            f.render_widget(list_items_wrapper, chunks[2]);

            f.set_cursor(chunks[1].x + app.input.width() as u16 + 1, chunks[1].y + 1);
        })?;

        // ===========
        //    Input
        // ===========

        if let Event::Input(input) = events.next()? {
            match input {
                EXIT_KEY => break,
                Key::Esc => {
                    app.input.clear();
                    app.page = 0;
                }
                Key::PageDown => {
                    if app.page < app.max_pages() - 1 {
                        app.page += 1;
                    }
                }
                Key::PageUp => {
                    if app.page > 0 {
                        app.page -= 1;
                    }
                }
                Key::Char(c) => {
                    app.input.push(c);
                    app.page = 0;
                }
                Key::Backspace => {
                    app.input.pop();
                    app.page = 0;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
