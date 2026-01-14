use std::{array::{self, from_fn}, io};

use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Rect}, style::{Color, Style}, text::Text, widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget}
};

use crate::connections::{Connections, ConnectionsGrid, Card, get_daily_puzzle};
use tui_big_text::{BigText, PixelSize};

/// Struct for the main data for the App.
pub struct App {
    solution: Connections,
    mistakes: u8,
    has_won: bool,
    content: ConnectionsGrid,
}

impl Default for App {
    fn default() -> Self {
        Self {
            solution: Connections::new(),
            mistakes: 0,
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
                cursor: (0, 0),
                selected_cells: array::from_fn(|_| { (usize::default(), usize::default()) })
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
            cell_size: 3,
            cols: 4,
            rows: 4,
        }
    }
}

impl StatefulWidget for Grid {
    type State = ConnectionsGrid;
    // Maybe create two states? One for the grid and the other for the difficulties?

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let col_constraints =
            (0..self.cols).map(|_| Constraint::Length(self.cell_size as u16 + 10)); // Width
        let row_constraints =
            (0..self.rows).map(|_| Constraint::Length(self.cell_size as u16 + 2)); // Height

        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);

        for (row_index, &row_area) in rows.iter().enumerate() {
            for (col_index, &col_area) in horizontal.split(row_area).to_vec().iter().enumerate() {

                let current_cell = state.grid[row_index][col_index].clone();

                Paragraph::new(
                    Text::from(format!("{}", current_cell.content.to_uppercase()))
                        .style(Style::new().fg(Color::White)),
                )
                .block(Block::bordered().border_type(BorderType::Rounded))
                .centered()
                // When guessed right we need to
                // change the color of the cell
                // to the color of the difficulty associated with that Category
                .style(Style::new().fg(Color::White))
                .render(col_area, buf);
            }
        }

    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.solution = get_daily_puzzle()
            .await
            .expect("FAILED TO GET DAILY PUZZLE");

        // Get the content of the grid from the solution
        self.content.grid = self.solution.clone().categories.map(|c| c.cards);

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
            Constraint::Max((grid.cell_size as u16 + 15) * 4),
            Constraint::Fill(1),
        ]);

        let rects = horizontal.split(frame.area());

        let inner_layout = &Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Min((grid.cell_size as u16 + 2) * 6),
            Constraint::Length(1),
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
        let info_footer =
            Paragraph::new(format!("Credits to: {}", self.solution.editor.to_owned()))
            .wrap(ratatui::widgets::Wrap { trim: false })
            .style(Style::new().fg(Color::White))
            .centered()
            .block(Block::bordered().border_style(Style::new().fg(Color::White)));

        frame.render_widget(info_footer, area);
    }
}
