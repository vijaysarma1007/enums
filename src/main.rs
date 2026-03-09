#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

fn main() {
    let first_card = CardSuit::Hearts;

    let mut second_card = CardSuit::Diamonds;
    second_card = CardSuit::Clubs;

    let card_suits = [CardSuit::Clubs, CardSuit::Diamonds];

    let card_suits_tuple = (CardSuit::Clubs, CardSuit::Diamonds);

    println!("{:?}", first_card);
}
