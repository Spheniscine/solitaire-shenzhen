use dioxus::prelude::*;

use crate::{components::{Emoji, SkinTrait}, game::{Card, ColorMode, HonorSkin, Skin, Suit, SuitSkin}};

pub const KATEX_MAIN: &str = "KaTeX_Main";

impl Skin {
    fn render_rank_internal(&self, card: &Card, text_mode: bool) -> Element {
        match *card {
            Card::Number { rank, .. } => rsx! {
                span {
                    font_family: KATEX_MAIN,
                    "{rank}",
                }
            },
            Card::Honor { suit } => {
                let text = match self.honors {
                    HonorSkin::Mythical => match suit {
                        Suit::Red => "🐦‍🔥",
                        Suit::Green => "🐉",
                        Suit::Blue => "🦄",
                    },
                    HonorSkin::Abstract => match suit {
                        Suit::Red => "✴",
                        Suit::Green => "❄",
                        Suit::Blue => "✧",
                    },
                    HonorSkin::Greek => match suit {
                        Suit::Red => "X",
                        Suit::Green => "Ψ",
                        Suit::Blue => "Ω",
                    },
                    HonorSkin::Eastern => match suit {
                        Suit::Red => "中",
                        Suit::Green => "發",
                        Suit::Blue => "🀆",
                    },
                };

                let font = match self.honors {
                    HonorSkin::Mythical => "",
                    HonorSkin::Abstract => "'Noto Sans Symbols 2'",
                    HonorSkin::Greek => KATEX_MAIN,
                    HonorSkin::Eastern => "Mahjong",
                };

                if self.honors == HonorSkin::Mythical {
                    rsx! {
                        Emoji { text }
                    }
                } else {
                    rsx! {
                        span {
                            font_family: font,
                            position: if !text_mode && self.honors == HonorSkin::Abstract {"relative"},
                            top: if !text_mode && self.honors == HonorSkin::Abstract {"0.11em"},
                            {text}
                        }
                    }
                }
            },
            Card::Flower => rsx! { Emoji { text: "🌸" } },
        }
    }
    fn render_suit_internal(&self, card: &Card, text_mode: bool) -> Element {
        let Card::Number { suit, .. } = *card else {return rsx! {}};
        let text = match self.suits {
            SuitSkin::Animals => match suit {
                Suit::Red => "🦊",
                Suit::Green => "🐰",
                Suit::Blue => "🐧",
            },
            SuitSkin::Shapes => match suit {
                Suit::Red => "⬥",
                Suit::Green => "▲",
                Suit::Blue => "●",
            },
            SuitSkin::Western => match suit {
                Suit::Red => "♥",
                Suit::Green => "♣",
                Suit::Blue => "♠",
            },
            SuitSkin::Eastern => match suit {
                Suit::Red => "萬",
                Suit::Green => "🎋",
                Suit::Blue => "🞋",
            },
        };

        let font = match self.suits {
            SuitSkin::Animals => "",
            SuitSkin::Shapes => "'Noto Sans Symbols 2'",
            SuitSkin::Western => KATEX_MAIN,
            SuitSkin::Eastern => "Mahjong",
        };

        if self.suits == SuitSkin::Animals {
            rsx! {
                Emoji { text }
            }
        } else {
            rsx! {
                span {
                    font_family: font,
                    position: if !text_mode && self.suits == SuitSkin::Shapes {"relative"},
                    top: if !text_mode && self.suits == SuitSkin::Shapes {"0.11em"},
                    {text}
                }
            }
        }
    }
}

const COLOR_GREEN: [&str; 2] = ["#062", "#00ff55"];
const COLOR_RED: [&str; 2] = ["#f00", "#ff8888"];
const COLOR_BLUE: [&str; 2] = ["#00d", "#aaaaff"];

impl SkinTrait<Card> for Skin {
    fn get_color(&self, card: &Card, mode: ColorMode) -> String {
        let suit = match *card {
            Card::Number { suit, .. } => suit,
            Card::Honor { suit } => suit,
            Card::Flower => Suit::Red,
        };
        let res = match suit {
            Suit::Red => COLOR_RED,
            Suit::Green => COLOR_GREEN,
            Suit::Blue => COLOR_BLUE,
        };
        res[mode as usize].to_string()
    }

    fn render_rank(&self, card: &Card) -> Element {
        self.render_rank_internal(card, false)
    }

    fn render_rank_text(&self, card: &Card) -> Element {
        self.render_rank_internal(card, true)
    }

    fn render_suit(&self, card: &Card) -> Element {
        self.render_suit_internal(card, false)
    }

    fn render_suit_text(&self, card: &Card) -> Element {
        self.render_suit_internal(card, true)
    }
    
    
}