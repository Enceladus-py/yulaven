use bevy::prelude::*;

use super::super::components::Teleporting;

/// System for handling player teleportation animation.
pub fn handle_teleportation(
    mut commands: Commands,
    mut pl_query: Query<(Entity, &mut Transform, &mut Teleporting)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut teleporting) in &mut pl_query {
        teleporting.timer.tick(time.delta());
        let t = teleporting.timer.fraction();

        let new_pos = teleporting
            .original_translation
            .lerp(teleporting.target_translation, t);
        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;

        if teleporting.timer.just_finished() {
            commands.entity(entity).remove::<Teleporting>();
        }
    }
}
