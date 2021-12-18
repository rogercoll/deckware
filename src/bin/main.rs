use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid arguments!\n Usage: ./deckware */List of card numbers separeted by comma and without whitespaces/*");
    }
    let result = deckware::extract_value(&args[1]).unwrap();
    println!("{:?}", result);
}
