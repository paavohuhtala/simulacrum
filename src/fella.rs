use bevy::prelude::*;
use bevy_aseprite::AsepriteBundle;
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{ListenerInput, On},
    PickableBundle,
};
use rand::Rng;

use crate::{
    time::{update_simulation_time, SimulationDeltaTime, SimulationTime},
    world::WorldPosition,
};

// Legally distinct from a Sim
#[derive(Component)]
pub struct Fella;

#[derive(Component)]
pub struct Named(String);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum BasicMotive {
    Hunger = 0,
    Bathroom = 1,
    Energy = 2,
    Hygiene = 3,
    Social = 4,
    Fun = 5,
    Comfort = 6,
    Environment = 7,
}

pub struct MotiveValue(pub f32);

impl Default for MotiveValue {
    fn default() -> Self {
        MotiveValue(0.5)
    }
}

#[derive(Component, Default)]
struct BasicMotives([MotiveValue; 8]);

#[allow(dead_code)]
impl BasicMotives {
    pub fn get(&self, motive: BasicMotive) -> f32 {
        self.0[motive as usize].0
    }

    pub fn set(&mut self, motive: BasicMotive, value: f32) {
        self.0[motive as usize].0 = value.clamp(0.0, 1.0);
    }

    pub fn change(&mut self, motive: BasicMotive, delta: f32) {
        let motive = &mut self.0[motive as usize].0;
        *motive = (*motive + delta).clamp(0.0, 1.0);
    }
}

#[derive(Event)]
pub struct SelectFellaEvent(pub Entity);

impl From<ListenerInput<Pointer<Click>>> for SelectFellaEvent {
    fn from(input: ListenerInput<Pointer<Click>>) -> Self {
        SelectFellaEvent(input.target)
    }
}

pub fn create_fella(
    commands: &mut Commands,
    name: impl Into<String>,
    sprite_path: &'static str,
    position: Vec2,
    asset_server: &AssetServer,
) {
    commands.spawn((
        Fella,
        BasicMotives::default(),
        Named(name.into()),
        AsepriteBundle {
            aseprite: asset_server.load(sprite_path),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        WorldPosition(position),
        WalkTarget {
            target: position,
            assigned_at: SimulationTime::default(),
        },
        PickableBundle::default(),
        On::<Pointer<Click>>::send_event::<SelectFellaEvent>(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

#[derive(Component, Debug)]
pub struct WalkTarget {
    target: Vec2,
    assigned_at: SimulationTime,
}

fn assign_random_walk_target(
    time: Res<SimulationTime>,
    mut query: Query<(&mut WalkTarget, &WorldPosition)>,
) {
    let mut rng = rand::thread_rng();

    for (mut walk_target, world_position) in query.iter_mut() {
        // If the target is reached, assign a new one
        if walk_target.target.distance(world_position.0) < 0.1 {
            // Wait a few ticks before assigning a new target to avoid jitter
            if time.time_since_ticks(&walk_target.assigned_at) < 2 {
                continue;
            };

            let x: f32 = rng.gen_range(-8.0..8.0);
            let y: f32 = rng.gen_range(-4.0..4.0);
            assert!(x.is_finite() && y.is_finite());

            walk_target.target = Vec2::new(x, y);
            walk_target.assigned_at = time.clone();

            println!("Assigned new walk target: {:?}", walk_target);
        }
    }
}

fn move_to_walk_target(
    delta: Res<SimulationDeltaTime>,
    mut query: Query<(&mut WorldPosition, &WalkTarget)>,
) {
    let Some(delta) = delta.0 else {
        return;
    };

    for (mut world_position, walk_target) in query.iter_mut() {
        let direction = (walk_target.target - world_position.0).normalize();

        // If direction is NaN, just skip this
        if !direction.is_finite() {
            continue;
        }

        world_position.0 += direction * 1.0 * delta as f32;
    }
}

#[derive(Component)]
pub struct Waiting(pub u64);

pub struct FellaPlugin;

impl Plugin for FellaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (assign_random_walk_target, move_to_walk_target).after(update_simulation_time),
        );
    }
}
