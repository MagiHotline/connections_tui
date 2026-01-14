use rand::{Rng, seq::SliceRandom};
use serde::Deserialize;
use core::array;
use std::{iter::from_fn, mem};

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

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Card {
    pub content: String,
    pub position: u8,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Category {
    pub title: String,
    pub cards: [Card; 4],
}

#[derive(Deserialize, Debug, Clone)]
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
            categories: array::from_fn(|_| Category::default()),
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

#[derive(Debug, PartialEq, Clone)]
pub struct ConnectionsGrid {
    pub grid: [[Card; 4]; 4],
    pub cursor: (usize, usize),
    pub selected_cells: [(usize, usize); 4]
}

impl ConnectionsGrid {

    // Function to shuffle the cards inside the grid
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();

        let array = self.grid.as_flattened_mut();

        array.shuffle(&mut rng);

        let new_cards: [[Card; 4]; 4] = [
            array[0..4].to_vec().try_into().unwrap(),
            array[4..8].to_vec().try_into().unwrap(),
            array[8..12].to_vec().try_into().unwrap(),
            array[12..16].to_vec().try_into().unwrap()
        ];

        self.grid = new_cards;
    }

    // Creates a new connectionsGrid given the connections
    pub fn new(connections : Connections) -> Self {

        let cards = connections.categories.map(|c| c.cards);

        let mut connectionsgrid =
        ConnectionsGrid
        {
            grid: cards, cursor: (0, 0), selected_cells: array::from_fn(|_| { (usize::default(), usize::default()) })
        };

        connectionsgrid.shuffle();
        connectionsgrid
    }

}


impl Default for ConnectionsGrid {
    fn default() -> Self {
        Self {
            grid: array::from_fn(|_|
                array::from_fn(|_|
                    Card::default()
                )
            ),
            cursor: (0, 0),
            selected_cells: array::from_fn(|_| { (usize::default(), usize::default()) })
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

    // Cambiare di test da un tipo expect a "assertare che un certo puzzle nel passato venga
    // uguale ad uno aspettato"
    #[tokio::test]
    async fn test_fetching_puzzle() {
        let result = get_daily_puzzle().await;

        // let expected  = Ok(Connections { ... })

        match result {
            Ok(result) => println!("{:?}", result),
            Err(e) => panic!("Failed to fetch puzzle: {}", e),
        }
    }

    #[test]
    fn test_shuffle() {

        let cards: [[Card; 4]; 4] = [
            [
                Card { content: String::from("FAST"), position: 8 },
                Card { content: String::from("FIRM"), position: 4 },
                Card { content: String::from("SECURE"), position: 10 },
                Card { content: String::from("TIGHT"), position: 14 },
            ],
            [
                Card { content: String::from("ACCOUNT"), position: 11 },
                Card { content: String::from("CLIENT"), position: 6 },
                Card { content: String::from("CONSUMER"), position: 15 },
                Card { content: String::from("USE"), position: 12 },
            ],
            [
                Card { content: String::from("FROSTY"), position: 0 },
                Card { content: String::from("MISTLETOE"), position: 7 },
                Card { content: String::from("RAINMAKER"), position: 3 },
                Card { content: String::from("SNOWMAN"), position: 1 },
            ],
            [
                Card { content: String::from("AUCTION"), position: 2 },
                Card { content: String::from("MOVIE"), position: 5 },
                Card { content: String::from("PARTNER"), position: 9 },
                Card { content: String::from("TREATMENT"), position: 13 },
            ],
        ];

        let connections_grid = ConnectionsGrid
            {
                grid: cards,
                cursor: (0,0),
                selected_cells: array::from_fn(|_| { (usize::default(), usize::default()) })
            };

        let mut shuffled = connections_grid.clone();

        shuffled.shuffle();

        assert_ne!(shuffled, connections_grid);
    }

}
