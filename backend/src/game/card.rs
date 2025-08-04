use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct CardSet(u64);

impl Add<Card> for CardSet {
    type Output = Self;

    fn add(self, rhs: Card) -> Self::Output {
        let bit: u32 = rhs.into();
        Self(self.0 | (1u64 << bit))
    }
}

impl Sub<Card> for CardSet {
    type Output = Self;

    fn sub(self, rhs: Card) -> Self::Output {
        let bit: u32 = rhs.into();
        Self(self.0 & !(1u64 << bit))
    }
}

impl CardSet {

    /// number of cards in the set
    pub fn len(self) -> u32 {
        self.0.count_ones()
    }

    /// set with no cards in it
    pub fn empty() -> Self {
        Self(0)
    }

    /// set with 52 cards
    pub fn full() -> Self {
        Self((1u64 << 52) - 1)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Card {
    suit: Suit,
    value: Value,
}

impl Into<u32> for Card {
    fn into(self) -> u32 {
        13 * self.suit as u32 + self.value as u32
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
