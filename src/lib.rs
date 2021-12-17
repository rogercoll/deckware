use std::error::Error;

const DECK_LENGTH: usize = 52;

fn has_unique_elements(slice: &[u8]) -> bool {
    !(1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
}

fn greater_than(slice: &[u8], max: u8) -> bool {
    !(1..slice.len()).any(|i| i > 52)
}

fn parse_deck(deck: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let values: Vec<u8> = deck
        .split(",")
        .map(|x| x.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()?;
    if values.len() != DECK_LENGTH {
        return Err(format!(
            "Not enough cards, got: {} want {}",
            values.len(),
            DECK_LENGTH
        )
        .into());
    }
    if !has_unique_elements(&values) {
        return Err("Duplicated cards on the provided deck".into());
    }
    if greater_than(&values, DECK_LENGTH as u8) {
        return Err(format!("Invalid card numbers").into());
    }
    Ok(values)
}

pub fn extract_value(deck: &str) -> Result<&str, Box<dyn Error>> {
    let cards: Vec<u8> = parse_deck(deck)?;
    Ok(deck)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ectract_value_test() {
        assert!(extract_value("1,2,3").is_err());
        assert!(extract_value("1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,50,52").is_err());
    }
}
