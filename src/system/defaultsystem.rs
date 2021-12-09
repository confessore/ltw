use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    let normal = materials.add(Color::rgb(0.15, 0.15, 0.15).into());
    let hovered = materials.add(Color::rgb(0.25, 0.25, 0.25).into());
    let pressed = materials.add(Color::rgb(0.35, 0.75, 0.35).into());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(ButtonBundle {
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
        material: normal.clone(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Button",
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

}
