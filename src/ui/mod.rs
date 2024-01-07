use bevy::prelude::*;

use crate::{
    fella::SelectFellaEvent,
    time::{SimulationTime, TimeScale},
};

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct SpeedButton(TimeScale);

fn create_ui(asset_server: Res<'_, AssetServer>, mut commands: Commands<'_, '_>) {
    // Add bottom bar and time display
    let font = asset_server.load::<Font>("fonts/ComicNeue-Bold.ttf");

    // Root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::End,
                ..default()
            },
            ..default()
        })
        // Bottom bar
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        max_width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        flex_grow: 1.0,
                        justify_content: JustifyContent::SpaceBetween,
                        align_self: AlignSelf::FlexEnd,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(8.0)),
                        column_gap: Val::Px(8.0),

                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.1, 0.1, 1.0, 0.8)),
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

#[derive(Resource)]
pub struct SelectedFella(Option<Entity>);

fn create_needs_panel(parent: &mut ChildBuilder<'_, '_, '_>, font: &Handle<Font>) {}

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

fn update_selected_fella(
    mut selected_fella: ResMut<SelectedFella>,
    mut events: EventReader<SelectFellaEvent>,
) {
    for event in events.read() {
        println!("Selected fella: {:?}", event.0);
        selected_fella.0 = Some(event.0);
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
                    update_selected_fella.run_if(on_event::<SelectFellaEvent>()),
                ),
            )
            .add_event::<SelectFellaEvent>()
            .insert_resource(SelectedFella(None));
    }
}
