use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, BorderType, Borders, List};

use crate::app::{App, Pane};

#[derive(Default)]
pub struct RightPane {}
impl RightPane {
    pub fn create(self, frame: &mut Frame, area: Rect, app: &mut App) {
        let focus_style = match app.pane_in_focus {
            Pane::Completed if app.adding_new == false => Style::default().fg(Color::LightGreen),
            _ => Style::default(),
        };

        let yesterday_pane = Block::default()
            .title(if app.pane_in_focus == Pane::Completed {
                Title::from(" Yesterday ".bold())
            } else {
                Title::from(" Yesterday ")
            })
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_alignment(Alignment::Center)
            .border_style(focus_style);

        if app.completed.items.len() > 0 && app.completed.state.selected().is_none() {
            app.completed.state.select(Some(0));
        }

        let items = app
            .completed
            .items
            .clone()
            .into_iter()
            .enumerate()
            .map(|(_i, item)| {
                let style = match app.completed.state.selected() {
                    Some(i) => match app.pane_in_focus {
                        Pane::Completed => {
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

                Text::styled(format!(" ✓ {}", item), style)
            });

        let right_pane_content = List::new(items).block(yesterday_pane);
        frame.render_widget(right_pane_content, area);
    }
}
