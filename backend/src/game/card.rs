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

impl TryFrom<u32> for Card {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let suit: Suit = (value / 13).try_into()?;
        let value: Value = (value % 13).try_into()?;
        Ok(Card{suit, value})
    }
}

impl From<Card> for u32 {
    fn from(value: Card) -> Self {
        let suit: u32 = value.suit.into();
        let value: u32 = value.value.into();
        suit * 13 + value
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

impl TryFrom<u32> for Suit {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let s = match value {
            0 => Suit::Diamonds,
            1 => Suit::Hearts,
            2 => Suit::Spades,
            3 => Suit::Clubs,
            other => anyhow::bail!("{} is not a valid suit", other),
        };
        Ok(s)
    }
}

impl From<Suit> for u32 {
    fn from(value: Suit) -> Self {
        value as u32
    }
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

impl TryFrom<u32> for Value {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let v = match value {
            0 => Value::Two,
            1 => Value::Three,
            2 => Value::Four,
            3 => Value::Five,
            4 => Value::Six,
            5 => Value::Seven,
            6 => Value::Eight,
            7 => Value::Nine,
            8 => Value::Ten,
            9 => Value::Jack,
            10 => Value::Queen,
            11 => Value::King,
            12 => Value::Ace,
            other => anyhow::bail!("{} is not a valid value", other)
        };
        Ok(v)
    }
}

impl From<Value> for u32 {
    fn from(value: Value) -> Self {
        value as u32
    }
}
