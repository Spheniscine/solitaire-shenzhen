use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_BORDER_RADIUS_RATIO, CARD_HEIGHT_RATIO, CardComponent, CardFrame, Movement, SkinTrait, rem}, game::{AnimationAct, AnimationKey, Board, BoardPos, Card, DepotRole, NUM_DEPOTS, Skin, Suit}};

#[component]
pub fn BoardComponent(
    position: Vec2,
    board: Board,
    skin: Skin,
    #[props(default)]
    onclick: EventHandler<BoardPos>,
    #[props(default)]
    ondoubleclick: EventHandler<BoardPos>,
    #[props(default)]
    animation_key: AnimationKey,
    #[props(default)]
    is_won: bool,
) -> Element {
    let card_width = 11f32;
    let card_height = card_width * CARD_HEIGHT_RATIO;
    let spacer_x = 1f32;
    let spacer_y = 1.5f32;

    let pos_x = {
        let w = 8.;
        let left = 50. - (w * card_width + (w-1.) * spacer_x) / 2.;
        move |i: usize| {
            left + (card_width + spacer_x) * i as f32
        }
    };

    let start_y = 2f32;
    let pos_y = |i: usize| start_y + (card_height + spacer_y) * i as f32;
    let column_card_offset = Vec2::new(0., 6.);

    let get_pos = |depot: usize, ord: usize| {
        let (role, index) = DepotRole::role_and_subindex(depot).unwrap();
        match role {
            DepotRole::FreeCell | DepotRole::HonorFoundation =>
                Vec2::new(pos_x(index), pos_y(0)),
            DepotRole::FlowerFoundation => 
                Vec2::new(pos_x(3).midpoint(pos_x(4)), pos_y(0)),
            DepotRole::NumberFoundation => 
                Vec2::new(pos_x(5 + index), pos_y(0)),
            DepotRole::Tableau => 
                Vec2::new(pos_x(index), pos_y(1)) + column_card_offset * ord as f32,
        }
    };

    let symbol2 = |text: &str| rsx! {
        span {
            font_family: "'Noto Sans Symbols 2'",
            position: "relative",
            top: "0.12em",
            {text}
        }
    };

    let get_hint = |depot: usize| {
        let role = DepotRole::role(depot).unwrap();
        match role {
            DepotRole::FreeCell => Some(symbol2("✽")),
            DepotRole::HonorFoundation => None,
            DepotRole::FlowerFoundation => Some(symbol2("❀")),
            DepotRole::NumberFoundation => Some(skin.render_rank(&Card::Number { rank: 1, suit: Suit::Red })),
            DepotRole::Tableau => Some(rsx!{}),
        }
    };

    
    let is_face_up = |depot: usize| {
        DepotRole::role(depot).unwrap().is_face_up()
    };

    let selected_height = if let Some(BoardPos { depot_index, card_index }) = board.selected {
        let d = if DepotRole::role(depot_index).unwrap() == DepotRole::Tableau {
            board.depots[depot_index].len() - card_index - 1
        } else {
            0
        };

        card_height + column_card_offset.y * d as f32
    } else {0.};

    let anims = board.animation_acts.iter().enumerate().map(|(i, act)| {
        match act {
            AnimationAct::Move(cards, pos1, pos2) => {
                let mut pos1 = *pos1;
                let mut pos2 = *pos2;
                let nodes = cards.iter().map(move |card| {
                    let p1 = get_pos(pos1.depot_index, pos1.card_index);
                    let p2 = get_pos(pos2.depot_index, pos2.card_index);
                    let res = rsx! {
                        Movement {
                            src_translate_vec: p1 - p2,
                            CardComponent {
                                position: p2,
                                width: card_width,
                                card: *card,
                                skin,
                            }
                        }
                    };
                    pos1.card_index += 1;
                    pos2.card_index += 1;
                    res
                });

                rsx! {
                    Fragment {
                        key: "{animation_key},{i}", // needed to force remounts, so animations don't get "stale" and refuse to replay
                        {nodes}
                    }
                }
            },
        }
    });

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            for depot in 0..NUM_DEPOTS {
                if let Some(hint) = get_hint(depot) {
                    CardFrame { 
                        position: get_pos(depot, 0),
                        width: card_width,
                        hint,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, !0))
                        },
                    }
                }

                for i in 0..board.depots[depot].len() {
                    if board.selected == Some(BoardPos::new(depot, i)) {
                        div {
                            position: "absolute",
                            top: rem(get_pos(depot, i).y),
                            left: rem(get_pos(depot, i).x),
                            width: rem(card_width),
                            height: rem(selected_height),
                            background_color: "#ff0",
                            border_radius: rem(card_width * CARD_BORDER_RADIUS_RATIO),
                            class: "selected-halo",
                        }
                    }

                    CardComponent { 
                        position: get_pos(depot, i),
                        width: card_width,
                        card: if is_face_up(depot) {board.depots[depot][i]},
                        // number_hint: if !is_face_up(depot) {i + 1},
                        skin,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, i))
                        },
                        ondoubleclick: move |_| {
                            ondoubleclick.call(BoardPos::new(depot, i))
                        },
                    }
                }
            }

            {anims}

            if is_won {
                div {
                    position: "absolute",
                    top: rem(25.),
                    left: rem(17.5),
                    width: rem(59.),
                    background_color: "#505",
                    padding: rem(3.),
                    color: "#fff",
                    font_size: rem(7.),
                    border_radius: rem(2.),
                    text_align: "center",
                    "YOU WIN!",
                }
            }
        }
    }
}