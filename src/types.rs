use std::collections::HashMap;

use serde::Deserialize;

use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GamesRoot {
    pub id: String,
    pub games: Vec<Game>,
    pub doc_created_time: CreatedTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    pub comment: Option<String>,
    pub players: Vec<String>,
    pub matches: Vec<HashMap<String, f64>>,
    pub created_time: CreatedTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatedTime {
    #[serde(rename = "_seconds")]
    pub seconds: i64,
    #[serde(rename = "_nanoseconds")]
    pub nanoseconds: i64,
}
