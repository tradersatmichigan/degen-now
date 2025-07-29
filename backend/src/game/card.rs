use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct Card(u8);

/// bitset representation of a set of cards
#[derive(Clone, Copy)]
pub struct CardSet(u64);

impl Add<Card> for CardSet {
    type Output = CardSet;

    fn add(self, rhs: Card) -> Self::Output {
        Self(self.0 | (1u64 << rhs.0))
    }
}

impl Sub<Card> for CardSet {
    type Output = CardSet;

    fn sub(self, rhs: Card) -> Self::Output {
        Self(self.0 & !(1u64 << rhs.0))
    }
}

impl CardSet {
    /// returns empty set of cards
    pub fn empty() -> Self {
        Self(0)
    }

    /// full 52 card deck constructor
    pub fn full() -> Self {
        Self((1u64 << 52) - 1)
    }

    /// return number of cards in set
    pub fn size(self) -> u32 {
        self.0.count_ones()
    }

    /// return the ith card (zero indexed) in the set if there are atleast i cards
    pub fn at(self, mut index: u32) -> Option<Card> {
        for i in 0..52 {
            if self.0 & (1u64 << i) != 0 {
                if index == 0 {
                    return Some(Card(i))
                }
                index -= 1;
            }
        }

        None
    }

    pub fn contains(self, other: &Self) -> bool {
        todo!()
    }

    pub fn remove(&mut self, other: &Self) {
        self.0 &= self.0 ^ other.0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cardset_construction() {
        assert_eq!(CardSet::full().size(), 52);
        assert_eq!(CardSet::empty().size(), 0);
    }
}
