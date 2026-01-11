use std::{array::from_fn, io};

use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Rect}, style::{Color, Style}, text::Text, widgets::{Block, BorderType, Paragraph, StatefulWidget}
};

use crate::connections::{Connections, ConnectionsGrid, Card, get_daily_puzzle};
use tui_big_text::{BigText, PixelSize};

/// Struct for the main data for the App.
pub struct App {
    solution: Connections,
    has_won: bool,
    content: ConnectionsGrid,
}

impl Default for App {
    fn default() -> Self {
        Self {
            solution: Connections::new(),
            has_won: false,
            content: ConnectionsGrid {
                grid: from_fn(|_|
                    from_fn(|_|
                        Card
                        {
                            content: String::from(""),
                            position:0
                        }
                    )
                ),
                selected_cell: (0, 0),
            },
        }
    }
}

pub struct Grid {
    cell_size: usize,
    cols: usize,
    rows: usize,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cell_size: 1,
            cols: 4,
            rows: 4,
        }
    }
}

impl StatefulWidget for Grid {
    type State = ConnectionsGrid;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let col_constraints =
            (0..self.cols).map(|_| Constraint::Length(self.cell_size as u16 + 4));
        let row_constraints =
            (0..self.rows).map(|_| Constraint::Length(self.cell_size as u16 + 2));

        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints);


        let rows = vertical.split(area);

        for (row_index, &row_area) in rows.iter().enumerate() {
            for (col_index, &col_area) in horizontal.split(row_area).to_vec().iter().enumerate() {

                let current_cell = state.grid[row_index][col_index].clone();

            }
        }

    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.solution = get_daily_puzzle()
            .await
            .expect("FAILED TO GET DAILY PUZZLE");

        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let grid = Grid::default();

        let horizontal = &Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Max((grid.cell_size as u16 + 5) * 4),
            Constraint::Fill(1),
        ]);

        let rects = horizontal.split(frame.area());

        let inner_layout = &Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Min((grid.cell_size as u16 + 2) * 6),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(rects[1]);

        self.render_header(frame, inner_layout[1]);
        frame.render_stateful_widget(grid, inner_layout[2], &mut self.content);
        self.render_footer(frame, inner_layout[4]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .style(Style::new().fg(Color::White))
            .centered()
            .lines(vec!["ConnectionsTUI".into()])
            .build();

        frame.render_widget(header, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(self.solution.editor.to_owned())
            .wrap(ratatui::widgets::Wrap { trim: false })
            .style(Style::new().fg(Color::White))
            .centered()
            .block(Block::bordered().border_style(Style::new().fg(Color::White)));

        frame.render_widget(info_footer, area);
    }
}
