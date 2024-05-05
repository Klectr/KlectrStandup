use crate::{tui, ui};
use color_eyre::eyre::{Context, Ok};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::ListState;
use ratatui::Frame;

pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
}

#[derive(PartialEq)]
pub enum Pane {
    Todo,
    Completed,
}

pub struct App {
    pub exit: bool,
    pub adding_new: bool,
    pub new_value: String,
    pub todos: StatefulList,
    pub completed: StatefulList,
    pub pane_in_focus: Pane,
}

impl App {
    pub fn new() -> App {
        App {
            exit: false,
            adding_new: false,
            new_value: String::new(),
            pane_in_focus: Pane::Todo,
            todos: StatefulList {
                state: ListState::default(),
                items: vec![],
            },
            completed: StatefulList {
                state: ListState::default(),
                items: vec![],
            },
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        ui::Ui::default().build(frame, self);
    }

    fn handle_events(&mut self) -> Result<()> {
        // check for timeout event
        if !event::poll(std::time::Duration::from_millis(16))? {
            return Ok(());
        }

        match event::read()? {
            // check if event key is pressed
            Event::Key(key_event) => {
                // guard clause preventing
                if key_event.kind != KeyEventKind::Press {
                    return Ok(());
                }

                return self
                    .handle_key_event(key_event)
                    .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}"));
            }
            _ => Ok(()),
        }
    }

    fn add_new_todo(&mut self) {
        self.adding_new = true;
    }

    fn handle_enter_press(&mut self) {
        match self.pane_in_focus {
            Pane::Todo => self.complete_todo(),
            Pane::Completed => self.revert_todo(),
        }
    }

    fn prev_pane(&mut self) {
        match self.pane_in_focus {
            Pane::Todo => self.pane_in_focus = Pane::Completed,
            Pane::Completed => self.pane_in_focus = Pane::Todo,
        }
    }

    fn next_pane(&mut self) {
        match self.pane_in_focus {
            Pane::Todo => self.pane_in_focus = Pane::Completed,
            Pane::Completed => self.pane_in_focus = Pane::Todo,
        }
    }

    fn handle_edit_mode_key_events(&mut self, key_code: KeyCode) -> Result<()> {
        match key_code {
            KeyCode::Char(value) => {
                self.new_value.push(value);
                Ok(())
            }
            KeyCode::Backspace => {
                self.new_value.pop();
                Ok(())
            }
            KeyCode::Esc => {
                self.new_value = String::new();
                self.adding_new = false;
                Ok(())
            }
            KeyCode::Enter => {
                self.todos.items.push(self.new_value.clone());
                self.todos.state.select(Some(self.todos.items.len() - 1));
                self.adding_new = false;
                self.new_value = String::new();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn handle_normal_mode_key_events(&mut self, key_code: KeyCode) -> Result<()> {
        match key_code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('a') => self.add_new_todo(),
            KeyCode::Char('j') => self.go_next(),
            KeyCode::Char('k') => self.go_prev(),
            KeyCode::Char('h') => self.prev_pane(),
            KeyCode::Char('l') => self.next_pane(),
            KeyCode::Char('d') => self.delete(),
            KeyCode::Enter => self.handle_enter_press(),
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.adding_new {
            true => self.handle_edit_mode_key_events(key_event.code),
            false => self.handle_normal_mode_key_events(key_event.code),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn go_next(&mut self) {
        match self.pane_in_focus {
            Pane::Todo => {
                match self.todos.items.len() {
                    i if i > 0 => self.todos.next(),
                    _ => {}
                };
            }
            Pane::Completed => {
                match self.completed.items.len() {
                    i if i > 0 => self.completed.next(),
                    _ => {}
                };
            }
        };
    }

    fn go_prev(&mut self) {
        match self.pane_in_focus {
            Pane::Todo => {
                match self.todos.items.len() {
                    i if i > 0 => self.todos.previous(),
                    _ => {}
                };
            }
            Pane::Completed => {
                match self.completed.items.len() {
                    i if i > 0 => self.completed.previous(),
                    _ => {}
                };
            }
        }
    }

    fn complete_todo(&mut self) {
        if self.todos.items.len() < 1 {
            return;
        }
        let curr = self.todos.state.selected().unwrap();
        let item_at_curr = self.todos.items.get(curr).unwrap();
        self.completed.items.push(item_at_curr.to_string());
        self.todos.items.remove(curr);
    }

    fn revert_todo(&mut self) {
        if self.completed.items.len() < 1 {
            return;
        }
        let curr = self.completed.state.selected().unwrap();
        let item_at_curr = self.completed.items.get(curr).unwrap();
        self.todos.items.push(item_at_curr.to_string());
        self.completed.items.remove(curr);
    }

    fn delete(&mut self) {
        self.go_prev();
        match self.pane_in_focus {
            Pane::Todo => {
                let curr = self.todos.state.selected().unwrap();
                self.todos.items.remove(curr);
            }
            Pane::Completed => {
                let curr = self.completed.state.selected().unwrap();
                self.completed.items.remove(curr);
            }
        }
    }
}

impl StatefulList {
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
