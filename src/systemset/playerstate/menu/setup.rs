use crate::{
    button
};

use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(Camera2dBundle::default());
    // root node
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    })
    .with_children(|parent| {
        // left vertical border node
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(15.0), Val::Percent(100.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..Default::default()
            },
            color: Color::rgb(0.65, 0.65, 0.65).into(),
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
                color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: UiRect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: button::default::NORMAL.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "play",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            }
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
                        margin: UiRect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: button::default::NORMAL.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "options",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            }
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
                        margin: UiRect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: button::default::NORMAL.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "exit",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            }
                        ),
                        ..Default::default()
                    });
                });
            });
        });
    });
}
