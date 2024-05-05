use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, BorderType, Borders, List};

use crate::app::{App, Pane};

#[derive(Default)]
pub struct LeftPane {}
impl LeftPane {
    pub fn create(self, frame: &mut Frame, area: Rect, app: &App) {
        let focus_style = match app.pane_in_focus {
            Pane::Todo if app.adding_new == false => Style::default().fg(Color::LightGreen),
            _ => Style::default(),
        };

        let left_pane = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(if app.pane_in_focus == Pane::Todo {
                Title::from(" Today ".bold())
            } else {
                Title::from(" Today ")
            })
            .title_alignment(Alignment::Center)
            .border_style(focus_style);

        let items = app
            .todos
            .items
            .clone()
            .into_iter()
            .enumerate()
            .map(|(_i, item)| {
                let style = match app.todos.state.selected() {
                    Some(i) => match app.pane_in_focus {
                        Pane::Todo => {
                            if i == _i {
                                Style::new().fg(Color::LightBlue).bg(Color::Blue)
                            } else {
                                Style::default()
                            }
                        }
                        _ => Style::default(),
                    },
                    None => Style::default(),
                };

                Text::styled(format!(" ☐ {}", item), style)
            });

        let left_pane_content = List::new(items).block(left_pane);

        // Create a List from all list items and highlight the currently selected one
        frame.render_widget(left_pane_content, area);
    }
}
