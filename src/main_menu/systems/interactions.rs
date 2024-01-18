use bevy::{prelude::*, app::AppExit};

use crate::{main_menu::{components::{PlayButton, QuitButton}, styles::{HOVER_BUTTON_COLOR, PRESSED_BUTTON_COLOR, NORMAL_BUTTON_COLOR}}, AppState};

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor), 
        (Changed<Interaction>, With<PlayButton>)
    >,
    mut app_state_next: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = HOVER_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                app_state_next.set(AppState::Game);
            }
            Interaction::None => {
                *bg_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor), 
        (Changed<Interaction>, With<QuitButton>)
    >
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = HOVER_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                app_exit_writer.send(AppExit);
            }
            Interaction::None => {
                *bg_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}