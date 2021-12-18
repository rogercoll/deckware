# Deckware

Random number generator from a shuffled deck of cards.

## Cards values

The program does some maths to get a unique number for each shuffled deck, thus we need to make some numercial assignments:

- :clubs: Clubs: Ace - King = 1 - 13
- :diamonds: Diamonds: Ace - King = 14 - 26 
- :hearts: Hearts: Ace - King = 27 - 39 
- :spades: Spades: Ace - King = 40 - 52 

## Steps

1. Suffle a deck of Poker cards (without jokers) and note their numerical value
2. Build the project (`cargo build`) and run it, or run it directly with:

```
cargo run 13,20,3,4,5,6,7,8,9,42,11,12,1,14,15,16,17,18,19,2,21,22,23,40,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,24,41,10,43,50,45,46,47,48,49,44,51,52
19162120637135135995112400712879924970218301522662289893153979134230
```

**IMPORTANT:** This project was done just for playing with Rust, I recommend NOT to use it for production nor a real project. Nonetheless, if you still want to shuffle a a deck of cards, do it and provide the list of numerical values to SHA-3 or any other secure random oracle function.


Algorithm => [https://pthree.org/2021/02/18/introducing-deckware-a-224-bit-entropy-extractor/](https://pthree.org/2021/02/18/introducing-deckware-a-224-bit-entropy-extractor/)
