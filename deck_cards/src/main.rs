
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]

struct Deck{
    cards:Vec<String>,

}


impl Deck{
    fn new() -> Self {
        //List of 'suits' - "hearts", "spades"
    //List of values - 'ace', 'two', 'three'
    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];

    let mut cards = vec![];


    for suit in suits{
        for value in values{
            let card = format!("{} of {}", value, suit);
            cards.push(card);

        }
    }


    // let deck = Deck { cards};
    // return deck;

    Deck {cards}


  }
    fn shufle(&mut self){

        let mut rng =  thread_rng();
        self.cards.shuffle(&mut rng);

    }
    fn deal(&mut self, num_cards:usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)


    }
}
fn main() {
    let mut deck = Deck ::new();

    //deck.shufle();
    //Add error handling
    let cards =  deck.deal(3);
    
    println!("Here is your deck: {:#?}", cards);
    println!("Here is your deck: {:#?}", deck);
    //println!("Here is your deck: {deck}");
}
