use std::collections::HashMap;
use bevy::prelude::*;
use bevy::prelude::shape::Capsule;
use zuri_net::proto::packet::add_actor::AddActor;
use zuri_net::proto::packet::add_player::AddPlayer;
use zuri_net::proto::packet::move_actor_absolute::MoveActorAbsolute;
use zuri_net::proto::packet::remove_actor::RemoveActor;

/// Plugin responsible for managing entities in the world.
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityMap::default())
            .add_system(add_entity_system)
            .add_system(move_entity_absolute_system)
            .add_system(remove_entity_system);
    }
}

/// Basic components required by every entity.
#[derive(Bundle, Default)]
pub struct BaseEntityBundle {
    #[bundle]
    pub render: PbrBundle,
}

/// A component for an entity with a head that has separate rotation from its body.
#[derive(Component, Default)]
pub struct Head {
    /// The rotation of the head relative to the body.
    pub rot: Quat,
    pub eye_height: f32,
}

#[derive(Default, Resource)]
pub struct EntityMap {
    pub m: HashMap<u64, Entity>, // todo: an api
}

impl EntityMap {}

/// A system responsible for spawning entities sent by the server.
fn add_entity_system(
    mut commands: Commands,
    mut map: ResMut<EntityMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut pks: EventReader<AddPlayer>,
    mut pks2: EventReader<AddActor>,
) {
    let spawn_func = &mut |runtime_id: u64, position: Vec3, color: Color| {
        // We spawn a capsule for now.
        let mut mat = StandardMaterial::from(color);
        mat.reflectance = 0.01;
        mat.metallic = 0.;
        if let Some(old_entity) = map.m.insert(runtime_id, commands.spawn(BaseEntityBundle {
            render: PbrBundle {
                mesh: meshes.add(Capsule {
                    ..default()
                }.into()),
                material: mats.add(mat).into(),
                transform: Transform::from_xyz(position.x, position.y, position.z),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            },
            ..default()
        }).id()) {
            commands.entity(old_entity).despawn_recursive();
            error!("Server tried to add already existing entity with ID {}", runtime_id);
        } else {
            info!("Server spawned an entity with ID {} at {}", runtime_id, position);
        }
    };

    for pk in pks.iter() {
        spawn_func(pk.entity_runtime_id, pk.position, Color::RED);
    }
    for pk in pks2.iter() {
        spawn_func(pk.entity_runtime_id, pk.position, Color::BLUE);
    }
}

/// Moves entities to an absolute position.
fn move_entity_absolute_system(
    map: Res<EntityMap>,
    mut pks: EventReader<MoveActorAbsolute>,
    mut query: Query<&mut Transform>,
) {
    for pk in pks.iter() {
        let e = map.m.get(&pk.entity_runtime_id).cloned().unwrap();
        let mut tr = query.get_mut(e).unwrap();
        tr.translation = pk.position;
    }
}

/// Despawns entities when requested.
fn remove_entity_system(
    mut commands: Commands,
    mut map: ResMut<EntityMap>,
    mut pks: EventReader<RemoveActor>,
) {
    for pk in pks.iter() {
        let w = &pk.entity_unique_id as *const i64 as *const u64;
        let w: &u64 = unsafe { &*w };
        commands.entity(match map.m.remove(w) {
            None => {
                error!("Server tried to remove unknown entity with ID {}", w);
                continue;
            }
            Some(e) => e,
        }).despawn_recursive();
        info!("Server removed entity with ID {}", w);
    }
}