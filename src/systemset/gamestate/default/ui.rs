use crate::{
    GameState,
    button
};

use bevy::prelude::*;

pub fn button_system(
    mut game_state: ResMut<State<GameState>>,
    mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                //text.sections[0].value = "Play".to_string();
                *material = button::default::PRESSED.into();
                //text.sections[0].value {
                //    "exit" => {
                //        std::process::exit(0);
                //    }
                //}
                match text.sections[0].value.as_str() {
                    "play" => {
                        let state = game_state.current();
                        match *state {
                            GameState::Default => {
                                game_state.set(GameState::Playing).unwrap();
                                return;
                            },
                            _ => {

                            }
                        }
                    },
                    "exit" => {
                        std::process::exit(0);
                    },
                    _ => {
                        std::process::exit(-1);
                    }
                }
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Play".to_string();
                *material = button::default::HOVERED.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Play".to_string();
                *material = button::default::NORMAL.into();
            }
        }
    }
}
