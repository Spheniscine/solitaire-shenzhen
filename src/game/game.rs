use std::time::Duration;

use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::game::{Board, BoardPos, Card, DECK_SIZE, DepotRole, HONOR_COPIES, NUM_RANKS, RANKS, Skin, Suit};

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ActionRecord {
    pos1: BoardPos, pos2: BoardPos,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ScreenState {
    #[default] Game, 
    Settings, Help,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub undo_stack: Vec<usize>,
    pub already_won: bool,
    pub num_wins: i32,

    pub screen_state: ScreenState,

    pub allow_undo: bool,
    pub auto_play: bool,
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS {
            for suit in Suit::iter() {
                deck.push(Card::Number { rank, suit })
            }
        }
        for suit in Suit::iter() {
            for _ in 0..HONOR_COPIES {
                deck.push(Card::Honor { suit })
            }
        }
        deck.push(Card::Flower);

        deck.shuffle(rng);
        deck
    }

    pub fn init() -> Self {
        let mut res = Self {
            board: Board::empty(),
            deal: vec![],
            animation_key: 0,
            history: vec![],
            undo_stack: vec![],
            already_won: false,
            num_wins: 0,
            screen_state: ScreenState::Game,
            allow_undo: true,
            auto_play: true,
            skin: Skin::default(),
        };

        res.new_game();
        res
    }

    pub fn new_game(&mut self) {
        let deal = Self::new_deal(&mut rand::rng());
        self.board = Board::from_deal(&deal);
        self.deal = deal;
        self.history.clear();
        self.undo_stack.clear();
        self.already_won = false;
        // LocalStorage.save_game_state(&self);
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn is_won(&self) -> bool {
        use DepotRole::*;
        HonorFoundation.range().all(|d| self.board.depots[d].len() == HONOR_COPIES) &&
        FlowerFoundation.range().all(|d| self.board.depots[d].len() == 1) &&
        NumberFoundation.range().all(|d| self.board.depots[d].len() == NUM_RANKS)
    }

    pub fn can_stack(&self, back: Card, front: Card) -> bool {
        if let (
            Card::Number { rank: back_rank, suit: back_suit },
            Card::Number { rank: front_rank, suit: front_suit },
        ) = (back, front) {
            back_suit != front_suit && front_rank + 1 == back_rank
        } else {
            false
        }
    }

    pub fn can_sort(&self, back: Card, front: Card) -> bool {
        if let (
            Card::Number { rank: back_rank, suit: back_suit },
            Card::Number { rank: front_rank, suit: front_suit },
        ) = (back, front) {
            back_suit == front_suit && back_rank + 1 == front_rank
        } else {
            false
        }
    }

    pub fn can_select(&self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if ord >= self.board.depots[depot].len() {
            return false;
        }
        let slice = &self.board.depots[depot][ord..];

        let Some(role) = DepotRole::role(depot) else { return false };
        match role {
            DepotRole::FreeCell => slice.len() <= 1,
            DepotRole::HonorFoundation => false,
            DepotRole::FlowerFoundation => false,
            DepotRole::NumberFoundation => false,
            DepotRole::Tableau => slice.windows(2).all(|w| self.can_stack(w[0], w[1])),
        }
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }

        if let Some(src) = self.board.selected {
            if pos == src { 
                self.board.selected = None; 
                return;
            }
            if src.depot_index == pos.depot_index && self.can_select(pos) {
                self.board.selected = Some(pos);
                return;
            }

            let dest = BoardPos::new(pos.depot_index, pos.card_index.wrapping_add(1));
            self.move_intent(src, dest);
        } else {
            if self.can_select(pos) {
                self.board.selected = Some(pos);
            }
        }
    }

    fn do_move_raw(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.board.do_move(pos1, pos2);
        self.history.push(ActionRecord { pos1, pos2 })
    }

    /// dest = freecell / honor foundation index
    fn honor_sort(&mut self, suit: Suit, mut dest: Option<usize>) -> bool {
        use DepotRole::*;
        let history_len = self.history.len();
        dest = dest.or_else(|| {
            (0..FreeCell.number_of()).find(|&i| {
                self.board.depots[HonorFoundation.id(i)].is_empty() &&
                self.board.depots[FreeCell.id(i)].last()
                    .is_none_or(|&c| c == Card::Honor { suit })
            })
        });
        let Some(dest) = dest else {return false};
        let dest = HonorFoundation.id(dest);

        // find exposed honors
        let srcs = [FreeCell, Tableau].into_iter()
            .flat_map(|role| role.range())
            .filter(|&d| {
                self.board.depots[d].last() == Some(&Card::Honor { suit })
            }).collect::<Vec<_>>();
        if srcs.len() != HONOR_COPIES { return false };

        for i in 0..HONOR_COPIES {
            let src = srcs[i];
            self.do_move_raw(self.board.last_pos(src), BoardPos::new(dest, i));
        }

        self.undo_stack.push(history_len);
        true
    }

    fn move_intent(&mut self, pos1: BoardPos, pos2: BoardPos) -> bool {
        if pos1.depot_index == pos2.depot_index { return false; }
        let depot1 = &self.board.depots[pos1.depot_index];
        let depot2 = &self.board.depots[pos2.depot_index];
        let num_moved = depot1.len() - pos1.card_index;
        if pos2.card_index != depot2.len() { return false; }

        let card = depot1[pos1.card_index];
        let Some((role, ix)) = DepotRole::role_and_subindex(pos2.depot_index) else { return false };

        let history_len = self.history.len();
        match role {
            DepotRole::FreeCell => {
                if !self.board.depots[DepotRole::HonorFoundation.id(ix)].is_empty() { return false; }
                if num_moved != 1 { return false; }
                if let Some(&c) = depot2.last() {
                    let Card::Honor { suit } = c else {return false};
                    if c != card { return false; }
                    return self.honor_sort(suit, Some(ix));
                } else {
                    self.do_move_raw(pos1, pos2);
                }
            },
            DepotRole::HonorFoundation => return false,
            DepotRole::FlowerFoundation => {
                if card != Card::Flower { return false; }
                self.do_move_raw(pos1, pos2);
            },
            DepotRole::NumberFoundation => {
                let ok = num_moved == 1 && if let Some(&c) = depot2.last() {
                    self.can_sort(c, card)
                } else {
                    matches!(card, Card::Number { rank: 1, .. })
                };
                if !ok { return false; }
                self.do_move_raw(pos1, pos2);
            },
            DepotRole::Tableau => {
                let ok = depot2.last().is_none_or(|&c| self.can_stack(c, card));
                if !ok { return false; }
                self.do_move_raw(pos1, pos2);
            },
        }

        self.undo_stack.push(history_len);
        true
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.history.is_empty()
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            // self.check_auto_moves();
        }

        // if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }
}