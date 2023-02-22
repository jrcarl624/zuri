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
    entity_map: HashMap<u64, (i64, Entity)>,
    id_map: HashMap<i64, u64>,
}

impl EntityMap {
    pub fn insert_entity(&mut self, unique_id: i64, runtime_id: u64, entity: Entity) -> Option<(i64, Entity)> {
        self.id_map.insert(unique_id, runtime_id);
        if let Some(prev_entity) = self.entity_map.insert(runtime_id, (unique_id, entity)) {
            return Some(prev_entity)
        }
        None
    }

    pub fn entity_by_rid(&self, rid: u64) -> Option<Entity> {
        self.entity_map.get(&rid).map(|(_, v)| *v)
    }

    pub fn entity_by_uid(&self, uid: i64) -> Option<Entity> {
        self.entity_map.get(match self.id_map.get(&uid) {
            Some(v) => v,
            None => return None,
        }).map(|(_, v)| *v)
    }

    pub fn remove_entity(&mut self, uid: i64) -> Option<Entity> {
        let rid = match self.id_map.get(&uid) {
            Some(v) => v,
            None => return None,
        };
        self.entity_map.remove(rid).map(|(_, v)| v)
    }
}

/// A system responsible for spawning entities sent by the server.
fn add_entity_system(
    mut commands: Commands,
    mut map: ResMut<EntityMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut pks: EventReader<AddPlayer>,
    mut pks2: EventReader<AddActor>,
) {
    let spawn_func = &mut |unique_id: i64, runtime_id: u64, position: Vec3, color: Color| {
        // We spawn a capsule for now.
        let mut mat = StandardMaterial::from(color);
        mat.reflectance = 0.01;
        mat.metallic = 0.;
        if let Some((_, old_entity)) = map.insert_entity(unique_id, runtime_id, commands.spawn(BaseEntityBundle {
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
            error!("Server tried to add already existing entity with ID {}", unique_id);
        } else {
            info!("Server spawned an entity with ID {} at {}", unique_id, position);
        }
    };

    for pk in pks.iter() {
        spawn_func(pk.ability_data.entity_unique_id, pk.entity_runtime_id, pk.position, Color::RED);
    }
    for pk in pks2.iter() {
        spawn_func(pk.entity_unique_id, pk.entity_runtime_id, pk.position, Color::BLUE);
    }
}

/// Moves entities to an absolute position.
fn move_entity_absolute_system(
    map: Res<EntityMap>,
    mut pks: EventReader<MoveActorAbsolute>,
    mut query: Query<&mut Transform>,
) {
    for pk in pks.iter() {
        let e = map.entity_by_rid(pk.entity_runtime_id).unwrap();
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
        commands.entity(match map.remove_entity(pk.entity_unique_id) {
            None => {
                error!("Server tried to remove unknown entity with ID {}", pk.entity_unique_id);
                continue;
            }
            Some(e) => e,
        }).despawn_recursive();
        info!("Server removed entity with ID {}", pk.entity_unique_id);
    }
}