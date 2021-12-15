
use crate::{
    GameState,
    material::menubutton::MenuButton
};
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    menu_button_material: Res<MenuButtonMaterial>
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    // root node
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    })
    .with_children(|parent| {
        // left vertical border node
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(15.0), Val::Percent(100.0)),
                border: Rect::all(Val::Px(2.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            //left vertical content node
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    flex_wrap: FlexWrap::Wrap,
                    align_content: AlignContent::SpaceBetween,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: menu_button_material.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "play",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            })
            .with_children(|parent| {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: menu_button_material.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "options",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            })
            .with_children(|parent| {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: menu_button_material.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "exit",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            });
        });
    });
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_system(
    mut game_state: ResMut<State<GameState>>,
    button_materials: Res<MenuButtonMaterial>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                //text.sections[0].value = "Play".to_string();
                *material = button_materials.pressed.clone();
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
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                //text.sections[0].value = "Play".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}
