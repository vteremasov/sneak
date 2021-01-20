use crate::lib::types::Issue;
use crate::util::event::{Event, Events};
use crate::util::StatefulList;
use std::io;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Widget};
use tui::Terminal;

pub fn ui_get_to_report(todos: Vec<Issue>) -> Vec<Issue> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let events = Events::new();
    let mut list_state = StatefulList::with_items(todos);

    loop {
        terminal.draw(|f| {
            // Create two chunks with equal horizontal screen space
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let todo_items: Vec<ListItem> = list_state
                .items
                .iter()
                .enumerate()
                .map(|(idx, issue)| {
                    let mut header_elements = vec![
                        Span::styled(
                            format!("{:<9}", issue.file_name),
                            Style::default().fg(Color::Magenta),
                        ),
                        Span::raw(" "),
                        Span::styled(
                            issue.file_path.to_str().unwrap(),
                            Style::default().add_modifier(Modifier::ITALIC),
                        ),
                    ];

                    if list_state.should_report.contains(&idx) {
                        header_elements.push(Span::raw(" "));
                        header_elements.push(Span::styled(
                            "✔",
                            Style::default().add_modifier(Modifier::BOLD),
                        ))
                    }

                    let header = Spans::from(header_elements);

                    let log = Spans::from(vec![Span::raw(&issue.todo)]);

                    ListItem::new(vec![
                        Spans::from("-".repeat(chunks[1].width as usize)),
                        header,
                        Spans::from(""),
                        log,
                    ])
                })
                .collect();

            // Create a List from all list items and highlight the currently selected one
            let items = List::new(todo_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("TODOs into tasks"),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(items, chunks[0], &mut list_state.state);
        });

        // This is a simple example on how to handle events
        // 1. This breaks the loop and exits the program on `q` button press.
        // 2. The `up`/`down` keys change the currently selected item in the App's `items` list.
        // 3. `left` unselects the current item.
        match events.next().unwrap() {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('r') => {
                    list_state.report();
                }
                Key::Left => {
                    list_state.unselect();
                }
                Key::Down => {
                    list_state.next();
                }
                Key::Up => {
                    list_state.previous();
                }
                _ => {}
            },
            Event::Tick => {
                // println!("Event tick")
            }
        }
    }

    list_state
        .items
        .iter()
        .enumerate()
        .filter(|(idx, _)| list_state.should_report.contains(idx))
        .map(|(_, el)| *el)
        .collect()
}
