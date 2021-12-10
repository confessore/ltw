use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    // root node
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(15.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::Wrap,
            align_content: AlignContent::SpaceBetween,
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    })
    .with_children(|parent| {
        // left vertical border node
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                    align_content: AlignContent::SpaceBetween,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
                        align_content: AlignContent::SpaceBetween,
                        justify_content: JustifyContent::SpaceBetween,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            align_content: AlignContent::SpaceBetween,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Text Example",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE
                            },
                            TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                vertical: VerticalAlign::Center
                            }),
                        ..Default::default()
                    });
                });
            });
        });
    });
}
