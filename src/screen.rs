use crate::{
    cache::Cache,
    models::{for_display, Inventory},
};
use anyhow::Result;
use log::error;
use std::{cmp::max, collections::HashMap, io, sync::mpsc, thread, time::Duration};
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

const TICK_RATE: Duration = Duration::from_millis(250);
const EXIT_KEY: Key = Key::Alt('q');

struct App {
    inventories: HashMap<String, Inventory>,
    cache: Cache,
    input: String,
    page: u64,
}

impl App {
    fn new(inventories: HashMap<String, Inventory>, cache: Cache) -> Self {
        Self {
            inventories,
            cache,
            input: String::new(),
            page: 0,
        }
    }

    fn filtered_items(&self) -> Vec<String> {
        // TODO filtering and pagination
        // TODO combine all items of the same type per character into a single "stack"
        let mut ret = Vec::new();
        let slots = self
            .inventories
            .values()
            .flat_map(|inv| inv.all_content())
            .collect::<Vec<_>>();
        for slot in slots {
            let info = self.cache.lookup(&slot.id);
            ret.push(for_display(&slot, info));
        }
        ret
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

pub fn run(inventories: HashMap<String, Inventory>, cache: Cache) -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();
    let mut app = App::new(inventories, cache);

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

            let mut msg = Text::from(Span::raw("Type to filter, use Alt+Q to exit"));
            msg.patch_style(Style::default());
            let header = Paragraph::new(msg);
            f.render_widget(header, chunks[0]);

            let input = Paragraph::new(app.input.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, chunks[1]);

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
            if EXIT_KEY == input {
                break;
            }
            match input {
                EXIT_KEY => break,
                Key::Esc => app.input.clear(),
                Key::PageDown => app.page += 1,
                Key::PageUp => app.page = max(0, app.page - 1),
                Key::Char(c) => app.input.push(c),
                Key::Backspace => {
                    app.input.pop();
                }
                _ => {}
            }
        }
    }

    Ok(())
}