use bevy::prelude::*;

use crate::{
    fella::{BasicMotive, BasicMotives, Fella, Named, SelectedFella, ALL_MOTIVES},
    time::{SimulationTime, TimeScale},
};

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct SpeedButton(TimeScale);

const UI_BLUE: Color = Color::rgba(0.1, 0.1, 1.0, 0.8);

fn create_ui(asset_server: Res<'_, AssetServer>, mut commands: Commands<'_, '_>) {
    // Add bottom bar and time display
    let font = asset_server.load::<Font>("fonts/ComicNeue-Bold.ttf");

    // Root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        // Bottom bar
        .with_children(|parent| {
            create_motives_panel(parent, &font);

            parent
                .spawn(NodeBundle {
                    style: Style {
                        max_width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        flex_grow: 1.0,
                        justify_content: JustifyContent::SpaceBetween,
                        justify_self: JustifySelf::End,
                        align_self: AlignSelf::FlexEnd,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(8.0)),
                        column_gap: Val::Px(8.0),

                        ..default()
                    },
                    background_color: BackgroundColor(UI_BLUE),
                    ..default()
                })
                // Time display
                .with_children(|parent| {
                    let bundle = TextBundle::from_section(
                        "Foo Bar 00:00",
                        TextStyle {
                            font: font.clone(),
                            font_size: 26.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    );

                    parent.spawn(bundle).insert(TimeText);

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(8.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            create_speed_button(parent, &font, "||", TimeScale::Paused);
                            create_speed_button(parent, &font, "1x", TimeScale::Normal);
                            create_speed_button(parent, &font, "2x", TimeScale::Fast);
                            create_speed_button(parent, &font, "4x", TimeScale::Fastest);
                        });
                });
        });
}

fn create_speed_button(
    parent: &mut ChildBuilder<'_, '_, '_>,
    font: &Handle<Font>,
    label: &'static str,
    time_scale: TimeScale,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                flex_grow: 1.0,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            ..default()
        })
        .insert(SpeedButton(time_scale))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font: font.clone(),
                        font_size: 26.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                ..default()
            });
        });
}

#[derive(Component)]
struct MotiveBar(BasicMotive);

#[derive(Component)]
struct SelectedFellaLabel;

fn create_motives_panel(parent: &mut ChildBuilder<'_, '_, '_>, font: &Handle<Font>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                min_width: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(32.0),
                ..default()
            },
            background_color: BackgroundColor(UI_BLUE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                SelectedFellaLabel,
                TextBundle {
                    text: Text::from_section(
                        "No one selected",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::px(150.0), GridTrack::px(150.0)],
                        grid_auto_rows: vec![GridTrack::max_content()],
                        column_gap: Val::Px(16.0),
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Create bar for each need

                    for motive in ALL_MOTIVES {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(4.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        format!("{:?}", motive),
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 16.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                });
                                // Progress bar

                                // Container
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(16.0),
                                            padding: UiRect::all(Val::Px(2.0)),
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::rgb(
                                            0.0, 0.0, 0.2,
                                        )),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn((
                                            MotiveBar(motive),
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(50.0),
                                                    height: Val::Percent(100.0),
                                                    ..default()
                                                },
                                                background_color: BackgroundColor(Color::rgb(
                                                    0.0, 0.7, 0.0,
                                                )),
                                                ..default()
                                            },
                                        ));
                                    });
                            });
                    }
                });
        });
}

fn update_time_display(
    world_time: Res<SimulationTime>,
    time_scale: Res<TimeScale>,
    mut query: Query<&mut Text, With<TimeText>>,
) {
    if world_time.is_changed() || time_scale.is_changed() || world_time.is_first_tick() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{} {:?}", world_time.as_ref(), time_scale.as_ref());
        }
    }
}

fn handle_time_scale_button_events(
    mut time_scale: ResMut<TimeScale>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &SpeedButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut _background_color, speed_button) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *time_scale = speed_button.0;
            }
            _ => {}
        }
    }
}

fn on_fella_selected(
    selected_fella: Res<SelectedFella>,
    fellas: Query<&Named, With<Fella>>,
    mut fella_label: Query<&mut Text, With<SelectedFellaLabel>>,
) {
    if !selected_fella.is_changed() {
        return;
    }

    let label = if let Some(selected_fella) = selected_fella.0 {
        let name = fellas.get(selected_fella).unwrap().0.clone();
        name
    } else {
        String::from("No one selected")
    };

    fella_label.single_mut().sections[0].value = label;
}

fn update_motive_bars(
    selected_fella: Res<SelectedFella>,
    fellas: Query<&BasicMotives, With<Fella>>,
    mut query: Query<(&mut MotiveBar, &mut Style)>,
) {
    let selected_fella = match selected_fella.0 {
        Some(selected_fella) => selected_fella,
        None => return,
    };

    let motives = fellas.get(selected_fella).unwrap();

    for (motive_bar, mut style) in query.iter_mut() {
        let motive = motive_bar.0;
        let motive_value = motives.get(motive);
        style.width = Val::Percent(motive_value * 100.0);
    }
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_ui)
            .add_systems(
                Update,
                (
                    update_time_display,
                    handle_time_scale_button_events,
                    on_fella_selected,
                    update_motive_bars,
                ),
            )
            .insert_resource(SelectedFella(None));
    }
}
