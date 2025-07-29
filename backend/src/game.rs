use std::{collections::BTreeMap, sync::{Arc, Mutex}, time::{Duration}};

use crate::game::card::CardSet;

mod card;

pub type GameId = u64;

/// container for all active games
pub struct Manager {
    id_generator: IdGenerator,
    games: BTreeMap<GameId, Arc<Mutex<Game>>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            id_generator: IdGenerator::new(),
            games: BTreeMap::new(),
        }
    }

    pub fn create_game(&mut self) -> GameId {
        let game_id = self.id_generator.get();
        self.games.insert(game_id, Arc::new(Mutex::new(Game::new())));
        game_id
    }

    pub fn get(&self, game_id: u64) -> Option<Arc<Mutex<Game>>> {
        self.games.get(&game_id).cloned()
    }

    pub fn tick(&self, seconds: Duration) {
        for (_, game) in self.games.iter() {
            game.lock().unwrap().tick(seconds);
        }
    }
}

#[derive(Clone, Copy)]
enum Street {
    None,
    Exchange(usize),
    PostFlop(usize),
    Turn(usize),
    River(usize),
    Showdown,
}

impl Street {
    fn get_action(self) -> Option<usize> {
        match self {
            Street::Exchange(action) => Some(action),
            Street::PostFlop(action) => Some(action),
            Street::Turn(action) => Some(action),
            Street::River(action) => Some(action),
            _ => None,
        }
    }

    /// return whos turn it is to bet
    fn is_betting_street(self) -> bool {
        match self {
            Street::PostFlop(_) => true,
            Street::Turn(_) => true,
            Street::River(_) => true,
            _ => false,
        }
    }
}

pub struct Game {
    street: Street,
    street_expire: Duration,
    dealer_pos: usize,
    deck: CardSet,
    board: CardSet,
    players: Vec<Player>,
    buyins: BTreeMap<String, i64>,

    running: bool,
    pending_buyins: Vec<PendingBuyin>,
}

impl Game {
    fn new() -> Self {
        todo!()
    }

    fn tick(&mut self, seconds: Duration) {
        if seconds > self.street_expire {
            self.next();
        }
    }

    fn exchange(&mut self, name: String, cards: CardSet) -> anyhow::Result<()> {
        match self.street {
            Street::Exchange(_) => {
                let action = self.get_action().unwrap();
                let mut player = self.players[action].clone();

                if player.name != name {
                    anyhow::bail!("Not ur turn big dawg");
                }

                if !player.cards.contains(&cards) {
                    anyhow::bail!("U cant discard ts big dawg");
                }

                player.cards.remove(&cards);

                // deal new cards and call next
                todo!()
            }
            _ => anyhow::bail!("Cannot exchange right now")
        }
    }

    fn bet(&mut self, name: String, amount: u64) -> anyhow::Result<()> {
        if self.street.is_betting_street() {
            let action = self.get_action().unwrap();
            let mut player = self.players[action].clone();

            if player.name == name {

                if player.current_stack >= amount {

                    player.current_stack -= amount;
                    player.current_bet += amount;
                    player.hand_bet += amount;

                    self.players[action] = player;
                    self.next();
                    Ok(())

                } else {
                    anyhow::bail!("You dont have the facilities for that big man");
                }

            } else {
                anyhow::bail!("It is not your turn to bet");
            }
        } else {
            anyhow::bail!("Betting not allowed right now");
        }
    }

    fn get_action(&self) -> Option<usize> {
        if let Some(action) = self.street.get_action() {
            Some((self.dealer_pos + action) % self.players.len())
        } else {
            None
        }
    }

    fn next(&mut self) {
        todo!()
    }

    fn request_buyin(&mut self, name: String, amount: u64) {
        self.pending_buyins.push(PendingBuyin::new(name, amount));
    }
}

struct PendingBuyin {
    name: String,
    amount: u64,
    oked: bool,
}

impl PendingBuyin {
    fn new(name: String, amount: u64) -> Self {
        Self {
            name,
            amount,
            oked: false,
        }
    }
}

#[derive(Clone)]
struct Player {
    name: String,
    is_owner: bool,
    folded: bool,
    current_stack: u64,

    /// total amount player has bet in this hand
    hand_bet: u64,
    current_bet: u64,
    cards: CardSet,
}

/// sequential number generator that wraps back arround to zero
struct IdGenerator(u64);

impl IdGenerator {
    fn new() -> Self {
        Self(0)
    }

    fn get(&mut self) -> GameId {
        let ans = self.0;
        self.0 = ans.wrapping_add(1);
        ans
    }
}
