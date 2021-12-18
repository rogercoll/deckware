pub mod lehmer;

use std::error::Error;

const DECK_LENGTH: usize = 52;

fn has_unique_elements(slice: &[u8]) -> bool {
    !(1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
}

fn any_greater_than(slice: &[u8], max: u8) -> bool {
    slice.iter().any(|&i| i > max)
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
    if any_greater_than(&values, DECK_LENGTH as u8) {
        return Err(format!(
            "Invalid card numbers, any card can be higher than: {}",
            DECK_LENGTH
        )
        .into());
    }
    Ok(values)
}

pub fn extract_value(deck: &str) -> Result<String, Box<dyn Error>> {
    let cards: Vec<u8> = parse_deck(deck)?;
    Ok(lehmer::compute(&cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ectract_value_test() {
        assert!(extract_value("1,2,3").is_err());
        assert!(extract_value("1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,50,52").is_err());
        assert_eq!("19162120637135135995112400712879924970218301522662289893153979134230", extract_value("13,20,3,4,5,6,7,8,9,42,11,12,1,14,15,16,17,18,19,2,21,22,23,40,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,24,41,10,43,50,45,46,47,48,49,44,51,52").unwrap());
    }

    #[test]
    fn greater_than_test() {
        assert_eq!(false, any_greater_than(&[1, 2, 3], 4));
        assert_eq!(true, any_greater_than(&[1, 2, 3], 2));
        assert_eq!(true, any_greater_than(&[1, 2, 3], 0));
    }

    #[test]
    fn has_unique_elements_test() {
        assert_eq!(false, has_unique_elements(&[1, 2, 3, 1]));
        assert_eq!(false, has_unique_elements(&[2, 2, 3, 1]));
        assert_eq!(false, has_unique_elements(&[1, 2, 3, 3]));
        assert_eq!(false, has_unique_elements(&[1, 2, 3, 3]));
        assert_eq!(true, has_unique_elements(&[2, 3, 1]));
    }
}
