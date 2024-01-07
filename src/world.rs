use bevy::ecs::component::Component;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TilePosition(bevy::math::IVec2);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct WorldPosition(pub bevy::math::Vec2);
