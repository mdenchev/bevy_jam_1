use bevy::{
    app::AppExit,
    math::Size,
    prelude::{
        info, AssetServer, BuildChildren, Button, ButtonBundle, Changed, Color, Commands,
        Component, EventWriter, NodeBundle, Query, Res, ResMut, State, TextBundle, UiCameraBundle,
        With, Without,
    },
    text::Text,
    ui::{FlexDirection, Interaction, JustifyContent, Style, UiColor, Val},
};
use bevy_kira_audio::Audio;

use crate::{menus::common, resources::audio_channels::AudioChannels, GameState};

use super::common::{Disabled, HOVERED_COLOR, NORMAL_COLOR, PRESSED_COLOR};

#[derive(Debug, Clone, Copy, Component)]
pub enum ButtonId {
    SinglePlayer,
    Settings,
    Credits,
    Quit,
}

pub fn handle_buttons(
    mut game_state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ButtonId),
        (Changed<Interaction>, With<Button>, Without<Disabled>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
) -> anyhow::Result<()> {
    for (interaction, mut color, button_id) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_COLOR;
                match button_id {
                    ButtonId::SinglePlayer => {
                        game_state.overwrite_set(GameState::Playing)?;
                    }
                    ButtonId::Quit => {
                        app_exit_events.send(AppExit);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_COLOR;
            }
            Interaction::None => {
                *color = NORMAL_COLOR;
            }
        }
    }
    Ok(())
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
) {
    info!("[Scene:MainMenu:setup]");
    commands.spawn_bundle(UiCameraBundle::default());

    // Play bg music
    audio.play_looped_in_channel(asset_server.load("music/OutThere_0.ogg"), &channels.music);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: common::button_style(),
                    color: NORMAL_COLOR,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: common::text_style(),
                        text: Text::with_section(
                            "New Game",
                            common::text_textstyle(&*asset_server),
                            common::button_text_alignment(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(ButtonId::SinglePlayer);
            parent
                .spawn_bundle(ButtonBundle {
                    style: common::button_style(),
                    color: NORMAL_COLOR,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: common::text_style(),
                        text: Text::with_section(
                            "Settings",
                            common::text_textstyle(&*asset_server),
                            common::button_text_alignment(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(ButtonId::Settings)
                .insert(Disabled);
            parent
                .spawn_bundle(ButtonBundle {
                    style: common::button_style(),
                    color: NORMAL_COLOR,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: common::text_style(),
                        text: Text::with_section(
                            "Credits",
                            common::text_textstyle(&*asset_server),
                            common::button_text_alignment(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(ButtonId::Credits)
                .insert(Disabled);
            parent
                .spawn_bundle(ButtonBundle {
                    style: common::button_style(),
                    color: NORMAL_COLOR,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: common::text_style(),
                        text: Text::with_section(
                            "Quit",
                            common::text_textstyle(&*asset_server),
                            common::button_text_alignment(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(ButtonId::Quit);
        });
}
