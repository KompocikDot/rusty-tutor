use serde::Deserialize;

// path extractor
#[derive(Deserialize)]
pub struct Item {
    pub id: i32,
}
