use serde::{Serialize, Deserialize};

pub type ApiGameId = String;
pub type ApiSnakeId = String;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiSnakeConfig {
    pub color: String,
    pub head_type: String,
    pub tail_type: String,
}

#[derive(Deserialize)]
pub struct ApiGame {
    pub id: ApiGameId,
}

#[derive(Deserialize, Copy, Clone, PartialEq)]
pub struct ApiCoords {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize)]
pub struct ApiBoard {
    pub height: u32,
    pub width: u32,
    pub food: Vec<ApiCoords>,
    pub snakes: Vec<ApiSnake>,
}

#[derive(Deserialize, Clone)]
pub struct ApiSnake {
    pub id: ApiSnakeId,
    pub name: String,
    pub health: u32,
    pub body: Vec<ApiCoords>,
}

#[derive(Deserialize)]
pub struct ApiGameState {
    pub game: ApiGame,
    pub turn: u32,
    pub board: ApiBoard,
    pub you: ApiSnake,
}

#[derive(Serialize, Copy, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ApiDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize)]
pub struct ApiMove {
    #[serde(rename = "move")]
    pub move_dir: ApiDirection,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashMap;

    impl ApiGameState {

        pub fn parse_basic(s: &str) -> ApiGameState {
            let rows = s.lines().map(str::trim).filter(|l| l.starts_with('|')).collect::<Vec<_>>();
            let height = rows.len();
            let mut width = 0;
            let mut food = Vec::new();
            let mut snakes_coords: HashMap<String, Vec<ApiCoords>> = HashMap::new();
            let mut you_coords = Vec::new();

            for (y, row) in rows.iter().enumerate() {
                let cols = row.trim_start_matches('|').split_terminator('|').collect::<Vec<_>>();
                width = std::cmp::max(height, cols.len());
                for (x, &col) in cols.iter().enumerate() {
                    let coord = ApiCoords {x: x as u32, y: y as u32};
                    match col {
                        "  " => {},
                        "()" => {
                            food.push(coord);
                        },
                        content => {
                            if content.is_empty() {
                                continue;
                            }
                            let chars: Vec<char> = content.chars().collect();
                            let snake_name = chars[0].to_string();
                            let index = chars[1].to_digit(10).unwrap() as usize;
                            if snake_name == "Y" {
                                you_coords.resize(index + 1, coord);
                                you_coords[index] = coord;
                            } else if let Some(body) = snakes_coords.get_mut(&snake_name) {
                                body.resize(index + 1, coord);
                                body[index] = coord;
                            } else {
                                let mut body = Vec::new();
                                body.resize(index + 1, coord);
                                body[index] = coord;
                                snakes_coords.insert(snake_name, body);
                            }
                        }
                    }
                }
            }

            ApiGameState {
                game: ApiGame {id: ApiGameId::from("123")},
                turn: 0,
                board: ApiBoard {
                    height: height as u32,
                    width: width as u32,
                    food,
                    snakes: snakes_coords.iter().map(|(name, body)| ApiSnake {
                        id: format!("id_{}", name),
                        name: name.clone(),
                        health: 100,
                        body: body.clone()
                    }).collect(),
                },
                you: ApiSnake {
                    id: String::from("id_Y"),
                    name: String::from("Y"),
                    health: 100,
                    body: you_coords,
                }
            }
        }
    }
}
