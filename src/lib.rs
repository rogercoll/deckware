use std::error::Error;

const DECK_LENGTH: usize = 52;


pub fn extract_value(cards: &str) -> Result<&str, Box<dyn Error>> {
    let values: Vec<&str> = cards.split(",").collect();
    if values.len() != DECK_LENGTH {
        return Err(format!("Not enough cards, got: {} want {}", values.len(), DECK_LENGTH).into())
    }
    Ok(cards)
}
