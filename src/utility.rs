use std::rc::Rc;

use bevy::ecs::entity::Entity;

use crate::fella::{BasicMotive, BasicMotives};

pub enum FellaAction {
    // TODO: Support multiple actions per object
    UseObject(Entity, Rc<ObjectDefinition>),
}

struct ObjectDefinition {
    motive_changes: Vec<(BasicMotive, f32)>,
}

struct ScoreActionInput {
    current_motives: BasicMotives,
    object: Rc<ObjectDefinition>,
    distance: f32,
}

pub fn score_action(input: &ScoreActionInput) -> f32 {
    0.0
}
