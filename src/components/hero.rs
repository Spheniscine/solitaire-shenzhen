use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::CardComponent, game::{Card, HonorSkin, Skin, Suit, SuitSkin}};

#[component]
pub fn Hero() -> Element {
    let width = 11f32;
    rsx! {
        div {
            id: "hero",
            for (y, skin) in [(0., Skin { honors: HonorSkin::Eastern, suits: SuitSkin::Eastern }), 
                (70., Skin { honors: HonorSkin::Mythical, suits: SuitSkin::Animals })] {
            
                
                for i in 1..=9 {
                    CardComponent { 
                        position: Vec2::new(3., y + 6. * i as f32),
                        width,
                        card: Card::Number { rank: i, suit: Suit::Red },
                        skin,
                    }
                }

                for i in 1..=9 {
                    CardComponent { 
                        position: Vec2::new(15., y + 6. * i as f32),
                        width,
                        card: Card::Number { rank: i, suit: Suit::Green },
                        skin,
                    }
                }

                for i in 1..=9 {
                    CardComponent { 
                        position: Vec2::new(27., y + 6. * i as f32),
                        width,
                        card: Card::Number { rank: i, suit: Suit::Blue },
                        skin,
                    }
                }

                CardComponent { 
                    position: Vec2::new(39., y + 6.),
                    width,
                    card: Card::Honor { suit: Suit::Red },
                    skin,
                }

                CardComponent { 
                    position: Vec2::new(51., y + 6.),
                    width,
                    card: Card::Honor { suit: Suit::Green },
                    skin,
                }

                CardComponent { 
                    position: Vec2::new(63., y + 6.),
                    width,
                    card: Card::Honor { suit: Suit::Blue },
                    skin,
                }

                CardComponent { 
                    position: Vec2::new(39., y + 20.),
                    width,
                    card: Card::Flower,
                    skin,
                }
            }
            
        }
    }
}