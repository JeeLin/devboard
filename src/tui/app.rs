use crate::db::{get_connection, run_migrations};
use crate::models::task::{self, Task, TaskStatus};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs},
    Frame, Terminal,
};
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use super::event::EventHandler;

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Tasks,
    Kanban,
    TimeReport,
}

impl View {
    pub fn titles() -> Vec<&'static str> {
        vec!["Tasks", "Kanban", "Time"]
    }
    pub fn index(&self) -> usize {
        match self {
            View::Tasks => 0,
            View::Kanban => 1,
            View::TimeReport => 2,
        }
    }
}

pub struct App {
    pub should_quit: bool,
    pub current_view: View,
    pub tasks: Vec<Task>,
    pub task_state: ListState,
    pub kanban_columns: Vec<(TaskStatus, Vec<Task>)>,
    pub kanban_selected_col: usize,
    pub kanban_selected_row: usize,
    event_handler: EventHandler,
}

impl App {
    pub fn new() -> Self {
        let db_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".devboard")
            .join("data.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).ok();

        let tasks = if let Ok(conn) = get_connection(&db_path) {
            let _ = run_migrations(&conn);
            task::list_tasks(&conn, None, None, None).unwrap_or_default()
        } else {
            vec![]
        };

        let kanban_columns = Self::build_kanban(&tasks);

        App {
            should_quit: false,
            current_view: View::Tasks,
            tasks: tasks.clone(),
            task_state: ListState::default(),
            kanban_columns,
            kanban_selected_col: 0,
            kanban_selected_row: 0,
            event_handler: EventHandler::new(Duration::from_millis(100)),
        }
    }

    fn build_kanban(tasks: &[Task]) -> Vec<(TaskStatus, Vec<Task>)> {
        let statuses = [
            (TaskStatus::Todo, "Todo"),
            (TaskStatus::InProgress, "In Progress"),
            (TaskStatus::Review, "Review"),
            (TaskStatus::Done, "Done"),
        ];
        statuses
            .iter()
            .map(|(status, _)| {
                let filtered: Vec<Task> = tasks
                    .iter()
                    .filter(|t| &t.status == status)
                    .cloned()
                    .collect();
                (status.clone(), filtered)
            })
            .collect()
    }

    pub fn refresh_tasks(&mut self) {
        let db_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".devboard")
            .join("data.db");
        if let Ok(conn) = get_connection(&db_path) {
            if let Ok(tasks) = task::list_tasks(&conn, None, None, None) {
                self.tasks = tasks;
                self.kanban_columns = Self::build_kanban(&self.tasks);
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c'))
            | (KeyModifiers::NONE, KeyCode::Char('q')) => {
                self.should_quit = true;
            }
            (KeyModifiers::NONE, KeyCode::Char('1')) => self.current_view = View::Tasks,
            (KeyModifiers::NONE, KeyCode::Char('2')) => self.current_view = View::Kanban,
            (KeyModifiers::NONE, KeyCode::Char('3')) => self.current_view = View::TimeReport,
            _ => match self.current_view {
                View::Tasks => self.handle_tasks_key(key),
                View::Kanban => self.handle_kanban_key(key),
                View::TimeReport => {}
            },
        }
    }

    fn handle_tasks_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                let i = self
                    .task_state
                    .selected()
                    .map(|i| (i + 1) % self.tasks.len())
                    .unwrap_or(0);
                self.task_state.select(Some(i));
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let i = self
                    .task_state
                    .selected()
                    .map(|i| if i == 0 { self.tasks.len() - 1 } else { i - 1 })
                    .unwrap_or(0);
                self.task_state.select(Some(i));
            }
            _ => {}
        }
    }

    fn handle_kanban_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                if self.kanban_selected_col > 0 {
                    self.kanban_selected_col -= 1;
                    self.kanban_selected_row = 0;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.kanban_selected_col < self.kanban_columns.len() - 1 {
                    self.kanban_selected_col += 1;
                    self.kanban_selected_row = 0;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let col_len = self
                    .kanban_columns
                    .get(self.kanban_selected_col)
                    .map(|(_, t)| t.len())
                    .unwrap_or(0);
                if col_len > 0 {
                    self.kanban_selected_row = (self.kanban_selected_row + 1) % col_len;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let col_len = self
                    .kanban_columns
                    .get(self.kanban_selected_col)
                    .map(|(_, t)| t.len())
                    .unwrap_or(0);
                if col_len > 0 {
                    self.kanban_selected_row = if self.kanban_selected_row == 0 {
                        col_len - 1
                    } else {
                        self.kanban_selected_row - 1
                    };
                }
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(f.area());

        let tab_titles: Vec<Line> = View::titles()
            .iter()
            .map(|t| Line::from(Span::styled(*t, Style::default())))
            .collect();
        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title(" DevBoard "))
            .select(self.current_view.index())
            .style(Style::default().fg(Color::White));
        f.render_widget(tabs, chunks[0]);

        match self.current_view {
            View::Tasks => self.draw_task_list(f, chunks[1]),
            View::Kanban => self.draw_kanban(f, chunks[1]),
            View::TimeReport => {
                let text = Paragraph::new(
                    "Time Report - manual logging via CLI: devboard time log/report",
                )
                .block(Block::default().borders(Borders::ALL).title("Time Report "));
                f.render_widget(text, chunks[1]);
            }
        }

        let status = Paragraph::new(Line::from(vec![Span::styled(
            " 1:Tasks 2:Kanban 3:Time  q:Quit  j/k:Up/Down  h/l:Left/Right",
            Style::default().fg(Color::DarkGray),
        )]));
        f.render_widget(status, chunks[2]);
    }

    fn draw_task_list(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .tasks
            .iter()
            .map(|t| {
                let priority_icon = match t.priority.as_str() {
                    "urgent" => "! ",
                    "high" => "^ ",
                    _ => "  ",
                };
                let status_color = match t.status {
                    TaskStatus::Todo => Color::Gray,
                    TaskStatus::InProgress => Color::Yellow,
                    TaskStatus::Review => Color::Cyan,
                    TaskStatus::Done => Color::Green,
                };
                ListItem::new(Line::from(vec![
                    Span::styled(priority_icon, Style::default()),
                    Span::styled(
                        format!("[{:?}] ", t.status),
                        Style::default().fg(status_color),
                    ),
                    Span::styled(&t.title, Style::default().fg(Color::White)),
                    Span::styled(
                        format!(" ({})", t.task_type.as_str()),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Tasks ({}) ", self.tasks.len())),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(list, area, &mut self.task_state);
    }

    fn draw_kanban(&self, f: &mut Frame, area: Rect) {
        let column_width = area.width / 4;
        let columns: Vec<Rect> = (0..4)
            .map(|i| Rect {
                x: area.x + i * column_width + i,
                y: area.y,
                width: column_width.saturating_sub(1),
                height: area.height,
            })
            .collect();

        let titles = ["Todo", "In Progress", "Review", "Done"];
        for (i, ((_, tasks), col_rect)) in
            self.kanban_columns.iter().zip(columns.iter()).enumerate()
        {
            let items: Vec<ListItem> = tasks
                .iter()
                .enumerate()
                .map(|(j, t)| {
                    let style = if i == self.kanban_selected_col && j == self.kanban_selected_row {
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Line::from(Span::styled(&t.title, style)))
                })
                .collect();

            let block = Block::default().borders(Borders::ALL).title(format!(
                "{} ({})",
                titles[i],
                tasks.len()
            ));
            f.render_widget(List::new(items).block(block), *col_rect);
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| self.draw(f))?;
            if let Ok(key) = self.event_handler.next() {
                self.handle_key(key);
            } else {
                self.should_quit = true;
            }
            if self.should_quit {
                break;
            }
        }

        crossterm::terminal::disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen
        )?;
        terminal.show_cursor()?;
        Ok(())
    }
}
