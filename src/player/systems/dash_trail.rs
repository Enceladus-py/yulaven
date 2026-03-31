use bevy::prelude::*;

use super::super::components::DashTrail;

/// System for animating and fading dash trail effects.
pub fn animate_dash_trail(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DashTrail, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut trail, mut sprite) in &mut query {
        trail.lifetime.tick(time.delta());

        let t = trail.lifetime.fraction(); // 0.0 to 1.0
        let alpha = (1.0 - t) * 0.85; // start at 0.85, fade out

        // Scale thickness down as it fades
        let thickness = 24.0 * (1.0 - t);
        if let Some(size) = sprite.custom_size.as_mut() {
            size.y = thickness;
        }

        sprite.color.set_alpha(alpha);

        if trail.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
