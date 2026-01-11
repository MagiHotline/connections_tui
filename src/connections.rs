use serde::Deserialize;
use core::array::from_fn;

#[derive(Debug)]
pub enum Difficulty {
    STRAIGHTFORWARD,
    MEDIUM,
    HARD,
    TRICKY,
}

impl Into<ratatui::style::Color> for Difficulty {
    fn into(self) -> ratatui::style::Color {
        match self {
            Difficulty::STRAIGHTFORWARD => ratatui::style::Color::Yellow,
            Difficulty::MEDIUM => ratatui::style::Color::Green,
            Difficulty::HARD => ratatui::style::Color::Blue,
            Difficulty::TRICKY => ratatui::style::Color::Magenta,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Card {
    pub content: String,
    pub position: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    pub title: String,
    pub cards: [Card; 4],
}

#[derive(Deserialize, Debug)]
pub struct Connections {
    pub id: u32,
    pub print_date: String,
    pub editor: String,
    pub categories: [Category; 4],
}

impl Connections {
    pub fn new() -> Connections {
        Connections {
            id: 0,
            print_date: String::from(""),
            editor: String::from(""),
            categories: from_fn(|_| Category {
                title: (String::from("")),
                cards: from_fn(|_| Card {
                    content: String::from(""),
                    position: 0,
                }),
            }),
        }
    }

    pub fn with_difficulties(self) -> Vec<(Category, Difficulty)> {
        let difficulties = [
            Difficulty::STRAIGHTFORWARD,
            Difficulty::MEDIUM,
            Difficulty::HARD,
            Difficulty::TRICKY,
        ];

        self.categories.into_iter().zip(difficulties).collect()
    }
}

pub struct ConnectionsGrid {
    pub grid: [[Card; 4]; 4],
    pub selected_cell: (usize, usize),
}

impl Default for ConnectionsGrid {
    fn default() -> Self {
        Self {
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
        }
    }
}

pub async fn get_daily_puzzle() -> std::result::Result<Connections, reqwest::Error> {
    let today_date = chrono::Local::now();

    let url = format!(
        "https://www.nytimes.com/svc/connections/v2/{}.json",
        today_date.format("%Y-%m-%d")
    );

    let response = reqwest::get(&url).await?.text().await?;

    let connections_data: Connections =
        serde_json::from_str(&response).expect("Failed to convert json to Connections");

    Ok(connections_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetching_puzzle() {
        let result = get_daily_puzzle().await;

        match result {
            Ok(result) => println!("{:?}", result),
            Err(e) => panic!("Failed to fetch puzzle: {}", e),
        }
    }
}
