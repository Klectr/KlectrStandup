use ratatui::prelude::*;

use crate::app::App;
use crate::widgets::add_new::AddNew;
use crate::widgets::instructions::Instructions;
use crate::widgets::left_pane::LeftPane;
use crate::widgets::right_pane::RightPane;
use crate::widgets::title::TitleWidget;

#[derive(Default)]
pub struct Ui {}

impl Ui {
    pub fn build(&self, frame: &mut Frame, app: &mut App) {
        let container = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Max(100),
                Constraint::Min(0),
            ])
            .split(frame.size());
        // layout containing a top middle and bottom panel
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2), // title
                Constraint::Length(3), // add new
                Constraint::Min(2),    // dual panes + add new
                Constraint::Length(2), // instructions
            ])
            .split(container[1]);

        let todo_panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[2]);

        TitleWidget::default().create(frame, main_layout[0]);
        AddNew::default().create(frame, main_layout[1], app);
        Instructions::default().create(frame, main_layout[3], app);

        LeftPane::default().create(frame, todo_panes[0], app);
        RightPane::default().create(frame, todo_panes[1], app);
    }
}
