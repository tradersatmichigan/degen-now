use std::ops::{Add, Sub};

use anyhow::Context;

/// NLH poker hands
pub enum NlhHand {

    // (top 5 cards...)
    HighCard(Value, Value, Value, Value, Value),

    // (pair, kickers...)
    Pair(Value, Value, Value, Value),

    // (high pair, low pair, kicker)
    TwoPair(Value, Value, Value),

    // (three cards, high kicker, lower kicker)
    ThreeOfAKind(Value, Value, Value),

    // (highest card)
    Straight(Value),

    // (highest card)
    Flush(Value, Value, Value, Value, Value),

    // (3 cards, 2 cards)
    FullHouse(Value, Value),

    // (4 cards, kicker)
    Quads(Value, Value),

    // (highest card)
    StraightFlush(Value),
}

impl From<CardSet> for NlhHand {
    fn from(value: CardSet) -> Self {
        let bits = value.0;

        // Straight flush
        {
            let len = 5;
            let sf_mask = (1 << len) - 1;
            for i in 0..Suit::COUNT {
                let suit_bits = bits >> (Value::COUNT * i);

                for j in Value::COUNT - len..=0 {
                    if (suit_bits >> j) & sf_mask == sf_mask {
                        return Self::StraightFlush(Value::try_from(j + len - 1).unwrap())
                    }
                }
            }
        }

        // Quads
        {
            let v_mask = (1 << Value::COUNT) - 1;
            let joined = (0..Suit::COUNT).fold(v_mask, |acc, i| {
                acc & (bits >> (i * Value::COUNT))
            });

            if joined != 0 {
                let four: Value = joined.trailing_zeros().try_into().unwrap();
                let all = (0..Suit::COUNT).fold(0, |acc, i| {
                    acc | (bits >> (i * Value::COUNT))
                });

                for i in Value::COUNT - 1..=0 {
                    if ((all & !joined) >> i) & 1 == 1 {
                        return Self::Quads(four, i.try_into().unwrap())
                    }
                }
                unreachable!()
            }
        }

        // Flush
        {
            let v_mask = (1 << Value::COUNT) - 1;
            for i in 0..Suit::COUNT {
                let suit_bits = (bits >> (Value::COUNT * i)) & v_mask;

                if suit_bits.count_ones() >= 5 {
                    let mut idx = [Value::Two; 5];
                    let mut cnt = 0;

                    for i in (0..Value::COUNT).rev() {
                        if (suit_bits >> i) & 1 == 1 {
                            idx[cnt] = i.try_into().unwrap();
                            cnt += 1;

                            if cnt == 5 {
                                break
                            }
                        }
                    }

                    assert_eq!(cnt, 5);
                    return Self::Flush(idx[0], idx[1], idx[2], idx[3], idx[4]);
                }
            }
        }

        todo!()
    }
}

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
        Self((1u64 << Card::COUNT) - 1)
    }

    /// return the ith card in set, ordered by suit then
    /// ordered by rank within suit
    pub fn at(self, mut idx: usize) -> Option<Card> {
        for i in 0..Card::COUNT {
            if (self.0 >> i) & 1 == 1 {
                if idx > 0 {
                    idx -= 1;
                } else {
                    return Some(Card::try_from(i).unwrap())
                }
            }
        }

        None
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    const COUNT: u32 = Suit::COUNT * Value::COUNT;
}

impl TryFrom<u32> for Card {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value >= Self::COUNT {
            anyhow::bail!("{} is not a valid card", value);
        }

        let suit: Suit = (value / 13).try_into().unwrap();
        let value: Value = (value % 13).try_into().unwrap();
        Ok(Card{suit, value})
    }
}

impl From<Card> for u32 {
    fn from(value: Card) -> Self {
        let suit: u32 = value.suit.into();
        let value: u32 = value.value.into();
        suit * Value::COUNT + value
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

impl Suit {
    const COUNT: u32 = 4;
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
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

impl Value {
    const COUNT: u32 = 13;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_suit_u32() {
        let suits = vec![Suit::Diamonds, Suit::Hearts, Suit::Spades, Suit::Clubs];

        for suit in suits {
            let s: u32 = suit.into();
            let s: Suit = s.try_into().unwrap();
            assert_eq!(suit, s);
        }
    }

    #[test]
    fn test_value_u32() {
        let values = vec![
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
            Value::Ace,
        ];

        for value in values {
            let v: u32 = value.into();
            let v: Value = v.try_into().unwrap();
            assert_eq!(value, v);
        }
    }

    #[test]
    fn test_card_u32() {
        for i in 0..Card::COUNT {
            let c: Card = i.try_into().unwrap();
            let c: u32 = c.into();
            assert_eq!(i, c);
        }
    }
}
