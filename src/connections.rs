use serde::Deserialize;

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
    title: String,
    cards: [Card; 4],
}

#[derive(Deserialize, Debug)]
pub struct Connections {
    categories: [Category; 4],
}

impl Connections {
    pub fn new() -> Connections {
        Connections {
            categories: core::array::from_fn(|_| Category {
                title: (String::from("")),
                cards: core::array::from_fn(|_| Card {
                    content: String::from(""),
                    position: 0,
                }),
            }),
        }
    }
}

impl Default for Connections {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConnectionsGrid {
    pub grid: Connections,
    pub selected_cell: (usize, usize),
}

impl Default for ConnectionsGrid {
    fn default() -> Self {
        Self {
            grid: Connections::new(),
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

    let connections_data =
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
