pub mod lehmer;

use std::error::Error;

const DECK_LENGTH: usize = 52;
// 2^225-1
const MAX_VALUE: &str = "53919893334301279589334030174039261347274288845081144962207220498431";

fn has_unique_elements(slice: &[u8]) -> bool {
    !(1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
}

fn any_greater_than(slice: &[u8], max: u8) -> bool {
    slice.iter().any(|&i| i > max)
}

// we need to discard anything larger than MAX_VALUE
fn greater_than_max(value: &str) -> bool {
    match value.len() < MAX_VALUE.len() {
        true => return false,
        false => {
            if value.len() > MAX_VALUE.len() {
                return true;
            }
            for (d1, d2) in value.chars().zip(MAX_VALUE.chars()) {
                if d1 > d2 {
                    return true;
                } else if d1 < d2 {
                    return false;
                }
            }
            return false;
        }
    }
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
    let num = lehmer::compute(&cards);
    if greater_than_max(&num) {
        return Err("Please provide another shuffle, this shuffle is considered insecure: because the permutation space is larger than 225 bits but not quite 226 bits, we can't use the full space, or we'll end up with a biased extractor".into());
    }
    Ok(num)
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

    #[test]
    fn greater_than_max_test() {
        assert_eq!(false, greater_than_max("1"));
        assert_eq!(false, greater_than_max("123456789"));
        assert_eq!(
            false,
            greater_than_max(
                "11111113334301279589334030174039261347274288845081144962207220498431"
            )
        );
        assert_eq!(
            false,
            greater_than_max(
                "53919893334301279589334030174039261347274288845081144962207220498431"
            )
        );
        assert_eq!(
            true,
            greater_than_max(
                "53919893334301279589334030174039261347274288845081144962207220999991"
            )
        );
    }
}
