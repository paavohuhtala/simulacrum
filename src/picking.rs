// My own picking impl because bevy_mod_picking doesn't seem to work with sprites?

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::{math::Rect, transform::components::GlobalTransform};

use crate::camera::MainCamera;

#[derive(Component, Debug, Clone)]
struct SpriteRect(Rect);

fn calculate_sprite_rects(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut sprites: Query<
        (
            &GlobalTransform,
            &TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut SpriteRect,
        ),
        Changed<GlobalTransform>,
    >,
) {
    for (global_transform, sprite, texture_atlas, mut rect) in sprites.iter_mut() {
        let texture_atlas = texture_atlases.get(texture_atlas).unwrap();
        let atlas_rect = texture_atlas.textures[sprite.index];
        let sprite_size = atlas_rect.size();

        if let Anchor::Center = sprite.anchor {
        } else {
            todo!("Implement non-center anchors");
        };

        let translation = global_transform.translation().xy();

        let sprite_rect = Rect {
            min: translation - sprite_size / 2.0,
            max: translation + sprite_size / 2.0,
        };

        rect.0 = sprite_rect;
    }
}

fn add_sprite_rect(
    mut commands: Commands,
    sprites: Query<
        (
            Entity,
            &GlobalTransform,
            &TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        Without<SpriteRect>,
    >,
) {
    for (entity, _, _, _) in sprites.iter() {
        commands.entity(entity).insert(SpriteRect(Rect::default()));
    }
}

#[derive(Component, Debug, Clone)]
pub struct Pickable;

#[derive(Event, Debug, Clone)]
pub struct OnPickEvent(pub Entity);

fn handle_on_click(
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    sprites: Query<(Entity, &SpriteRect), With<Pickable>>,
    mut pick_events: EventWriter<OnPickEvent>,
) {
    let clicking = buttons.just_pressed(MouseButton::Left);

    if !clicking {
        return;
    }

    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let cursor_world_position = camera.viewport_to_world_2d(camera_transform, cursor_position);

    let Some(cursor_world_position) = cursor_world_position else {
        return;
    };

    for (entity, sprite_rect) in sprites.iter() {
        if sprite_rect.0.contains(cursor_world_position) {
            println!("Clicked {:?}", entity);
            pick_events.send(OnPickEvent(entity));
        }
    }
}

pub struct MyPickingPlugin;

impl Plugin for MyPickingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<OnPickEvent>().add_systems(
            Update,
            (
                calculate_sprite_rects.after(add_sprite_rect),
                add_sprite_rect,
                handle_on_click.after(calculate_sprite_rects),
            ),
        );
    }
}
