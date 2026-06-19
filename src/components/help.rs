use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{components::{CardText, KATEX_MAIN, VIDEO_GAMEPLAY, rem}, game::{Card, ColorMode, GameState, ScreenState, Suit}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

#[component]
pub fn Help(mut game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let skin = st.skin;

    let stack_example = || {
        let mut ite = [
            Card::Number { rank: 5, suit: Suit::Green },
            Card::Number { rank: 4, suit: Suit::Red },
            Card::Number { rank: 3, suit: Suit::Green },
            Card::Number { rank: 2, suit: Suit::Blue },
        ].into_iter().map(|card| {
            rsx! {
                CardText { 
                    card, skin, color_mode: ColorMode::Light,
                }
            }
        });


        let last = ite.next().unwrap();
        rsx! {
            {ite.next().unwrap()},
            for x in ite { "–", {x} },
            " can be placed on the ", {last}
        }
    };

    let honors = || {
        let mut first = true;
        Suit::iter().map(move |suit| {
            let res = rsx! {
                if !first {" "}
                CardText { card: Card::Honor { suit }, skin, color_mode: ColorMode::Light, }
            };
            first = false;

            res
        })
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 3.5rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The deck is a special 40-card deck, whose cards are in one of three categories:"
                    ul {
                        li { "27 ", Emph{"number"}, " cards: of ranks ",
                            span {
                                font_family: KATEX_MAIN,
                                font_size: "1.2em",
                                "1~9"
                            }, ", in each of 3 suits."
                        },

                        li {
                            "One ", Emph{"flower"}, " card."
                        },

                        li {
                            "12 ", Emph{"honor"}, " cards: 4 copies each of 3 kinds.",br{},"(", {honors()}, ")"
                        },
                    }
                }

                p {
                    "The ",Emph{"foundation"}," thus consists of 7 slots of 3 types, corresponding to the card categories."
                }

                p {
                    Emph{"Number"}, " cards stack in the ", Emph{"tableau"}, " by ", Emph {"descending rank"}, " and " Emph {"unlike suit"},
                    ". Such stacks of any size can be moved as a unit. (e.g. ",{stack_example()},"). They are put away to the ",
                    Emph{"number slots"}, " at the top-right, in ascending order by suit."
                }

                p {
                    "The ",Emph{"flower"}, " card may be put away to the ", Emph{"flower slot"}, " once it is exposed."
                }

                p {
                    Emph{"Honor"}, " cards do not stack on other cards. The ", Emph{"honor slots"}, " at the top-left also act as ",
                    Emph{"free cells"}, " that may each store one card of any kind. When all 4 copies of a type of honor card are exposed, 
                    you may put them away by either moving two of them to the same honor slot, or double-clicking one of them. 
                    They will then permanently occupy an honor slot."
                }

                p {
                    "Empty tableau columns may be filled by any card or stack."
                }

                p {
                    "To ",Emph{"win the game"},", put all the cards away to their respective foundation slots."
                }

                p {
                    Emph{"Shortcut note:"}," Double-clicking on a card will either try to put it away if possible, or send it to a free cell
                    if not."
                }

                div {
                    position: "absolute",
                    bottom: rem(2.),
                    width: "92rem",
                    display: "flex",
                    justify_content: "center",

                    a {
                        href: VIDEO_GAMEPLAY,
                        target: "_blank",
                        text_decoration: "none",
                        margin_right: rem(4.),
                        div {
                            width: rem(30.),
                            position: "relative",
                            class: "game-button",
                            "Example video"
                        }
                    }

                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                        "Back to game"
                    }
                }
            }
        }
    }
}