use std::ops::Add;

use bevy::prelude::*;
use bevy_aseprite::AsepriteBundle;
use rand::Rng;

use crate::{
    picking::{OnPickEvent, Pickable},
    time::{update_simulation_time, SimulationDeltaTime, SimulationTime},
    world::WorldPosition,
};

// Legally distinct from a Sim
#[derive(Component)]
pub struct Fella;

#[derive(Component)]
pub struct Named(pub String);

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

pub const ALL_MOTIVES: [BasicMotive; 8] = [
    BasicMotive::Hunger,
    BasicMotive::Bathroom,
    BasicMotive::Energy,
    BasicMotive::Hygiene,
    BasicMotive::Social,
    BasicMotive::Fun,
    BasicMotive::Comfort,
    BasicMotive::Environment,
];

#[derive(Clone, Copy, Debug)]
pub struct MotiveValue(pub f32);

impl Add for MotiveValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MotiveValue(self.0 + rhs.0)
    }
}

impl Default for MotiveValue {
    fn default() -> Self {
        MotiveValue(0.5)
    }
}

#[derive(Component, Default, Clone)]
pub struct BasicMotives([MotiveValue; 8]);

#[derive(Default, Clone)]
pub struct BasicMotivesDelta([f32; 8]);

impl BasicMotivesDelta {
    pub fn add(&mut self, motive: BasicMotive, value: f32) {
        self.0[motive as usize] += value;
    }

    pub fn get(&self, motive: BasicMotive) -> f32 {
        self.0[motive as usize]
    }

    pub fn scale(&mut self, factor: f32) {
        for value in self.0.iter_mut() {
            *value *= factor;
        }
    }
}

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

    pub fn add(&mut self, delta: &BasicMotivesDelta) {
        for (motive, value) in delta.0.iter().enumerate() {
            self.0[motive].0 = (self.0[motive].0 + value).clamp(0.0, 1.0);
        }
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
        Pickable,
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

#[derive(Resource)]
pub struct SelectedFella(pub Option<Entity>);

fn select_fella(
    mut selected_fella: ResMut<SelectedFella>,
    mut select_fella_events: EventReader<OnPickEvent>,
    mut fellas: Query<(Entity, &mut TextureAtlasSprite), With<Fella>>,
) {
    for event in select_fella_events.read() {
        selected_fella.0 = Some(event.0);
        println!("Selected fella: {:?}", event.0);

        for (fella_entity, mut sprite) in fellas.iter_mut() {
            sprite.color = if fella_entity == event.0 {
                Color::RED
            } else {
                Color::WHITE
            };
        }
    }
}

fn apply_need_decay(
    delta: Res<SimulationDeltaTime>,
    mut query: Query<&mut BasicMotives, With<Fella>>,
) {
    let Some(delta) = delta.0 else {
        return;
    };

    let mut default_decays = BasicMotivesDelta([
        -0.006, // Hunger
        -0.005, // Bathroom
        -0.004, // Energy
        -0.005, // Hygiene
        -0.005, // Social
        -0.008, // Fun
        -0.005, // Comfort
        -0.009, // Environment
    ]);

    default_decays.scale(delta as f32);

    for mut basic_motives in query.iter_mut() {
        basic_motives.add(&default_decays);
    }
}

#[derive(Component)]
pub struct Waiting(pub u64);

pub struct FellaPlugin;

impl Plugin for FellaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedFella(None)).add_systems(
            Update,
            (
                assign_random_walk_target,
                move_to_walk_target,
                select_fella,
                apply_need_decay,
            )
                .after(update_simulation_time),
        );
    }
}
