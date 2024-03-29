use bevy::prelude::*;

use crate::main_menu::{components::*, styles::*};

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu (
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity = commands.spawn(
    (
        NodeBundle {
            style: Style{
                flex_direction: FlexDirection::Column,

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,

                row_gap: Val::Px(8.0),
                column_gap: Val::Px(8.0),

                width: Val::Percent(100.0), 
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        MainMenu {},
    ))
    .with_children(|parent| {
        // == Title ==
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(300.0),
                    height: Val::Px(120.0),
                    ..default()
                },
                ..default()
            }
        ).with_children(|parent| {
            // Image 1
            parent.spawn(
                ImageBundle {
                    style: Style {
                        width: Val::Px(64.0),
                        height: Val::Px(64.0),
                        margin: UiRect::new(
                            Val::Px(8.0), 
                            Val::Px(8.0), 
                            Val::Px(8.0), 
                            Val::Px(8.0)
                        ),
                        ..default()
                    },
                    image: asset_server.load("sprites/ball_blue_large.png").into(),
                    ..default()
            });
            // Text
            parent.spawn(
                TextBundle {
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "Bevy Ball Game",
                            TextStyle { 
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
                                font_size: 64.0, 
                                color: Color::WHITE,
                            }
                        )
                    ],

                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            // Image 2
            parent.spawn(
                ImageBundle {
                    style: Style {
                        width: Val::Px(64.0),
                        height: Val::Px(64.0),
                        margin: UiRect::new(
                            Val::Px(8.0), 
                            Val::Px(8.0), 
                            Val::Px(8.0), 
                            Val::Px(8.0)
                        ),
                        ..default()
                    },
                    image: asset_server.load("sprites/ball_red_large.png").into(),
                    ..default()
            });
        });
        // == Play ==
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            PlayButton {}
        )
        ).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                }
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
        // == Quit ==
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            QuitButton {}
        )
        ).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Quit",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                }
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
    })
    .id();

    main_menu_entity
}