use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::app::{App, Pane};

#[derive(Default)]
pub struct Instructions {}
impl Instructions {
    pub fn create(self, frame: &mut Frame, area: Rect, app: &App) {
        let instructions = match app.adding_new {
            false => Self::create_add_new_instructions(app),
            true => Self::create_normal_instructions(),
        };
        let text = Line::from(instructions);
        let instructions_final = Paragraph::new(text).style(Style::default());

        frame.render_widget(instructions_final, area);
    }

    fn create_add_new_instructions(app: &App) -> Vec<Span<'static>> {
        vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Add Task ".into(),
            "<A> ".blue().bold(),
            " Up/Down ".into(),
            "<J/K> ".blue().bold(),
            " Left/Right".into(),
            "<H/L> ".blue().bold(),
            if app.pane_in_focus == Pane::Todo {
                " Complete Item ".into()
            } else {
                " Undo Item ".into()
            },
            "<Enter> ".blue().bold(),
            " Delete Item ".into(),
            "<D> ".blue().bold(),
        ]
    }

    fn create_normal_instructions() -> Vec<Span<'static>> {
        vec![
            " Exit Add Mode ".into(),
            "<esc> ".blue().bold(),
            " Submit Task ".into(),
            "<enter> ".blue().bold(),
        ]
    }
}
