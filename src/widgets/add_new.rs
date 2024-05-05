use ratatui::prelude::*;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::app::App;

#[derive(Default)]
pub struct AddNew {}
impl AddNew {
    pub fn create(self, frame: &mut Frame, area: Rect, app: &App) {
        let active_style = match app.adding_new {
            true => Style::default().fg(Color::LightGreen),
            false => Style::default(),
        };

        let add_new_block = Block::new()
            .title(if app.adding_new {
                Title::from(" New Todo ".bold())
            } else {
                Title::from(" New Todo ")
            })
            .title_position(Position::Top)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(active_style);
        let add_new_final = Paragraph::new(app.new_value.clone()).block(add_new_block);

        frame.render_widget(add_new_final, area);
    }
}
