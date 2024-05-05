use ratatui::prelude::*;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::Block;

#[derive(Default)]
pub struct TitleWidget {}
impl TitleWidget {
    pub fn create(self, frame: &mut Frame, area: Rect) {
        frame.render_widget(&self, area);
    }
}
impl Widget for &TitleWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let _ = Block::new()
            .title(Title::from(" Daily Standup ".bold()))
            .title_position(Position::Top)
            .title_alignment(Alignment::Center)
            .render(area, buf);
    }
}
