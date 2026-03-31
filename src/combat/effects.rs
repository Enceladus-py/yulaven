use bevy::prelude::*;

use super::components::{DamageFlash, Invincible, Knockback};
use crate::enemy::components::EnemyKind;

/// System for applying knockback to entities.
pub fn apply_knockback(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Knockback)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut knockback) in &mut query {
        knockback.timer.tick(time.delta());

        transform.translation += (knockback.velocity * time.delta_secs()).extend(0.0);

        if knockback.timer.is_finished() {
            commands.entity(entity).remove::<Knockback>();
        }
    }
}

/// System for handling invincibility frames with visual flickering.
pub fn handle_invincibility(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut Invincible)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut invincible) in &mut query {
        invincible.0.tick(time.delta());

        if invincible.0.is_finished() {
            sprite.color.set_alpha(1.0);
            commands.entity(entity).remove::<Invincible>();
        } else {
            let t = invincible.0.elapsed_secs() * 10.0;
            let alpha = if t % 2.0 < 1.0 { 0.5 } else { 1.0 };
            sprite.color.set_alpha(alpha);
        }
    }
}

/// System for handling damage flash visual effect on enemies.
pub fn handle_damage_flash(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut DamageFlash, Option<&EnemyKind>)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut flash, opt_kind) in &mut query {
        flash.0.tick(time.delta());

        if flash.0.is_finished() {
            sprite.color = opt_kind.map_or(Color::WHITE, |k| k.color());
            commands.entity(entity).remove::<DamageFlash>();
        } else {
            sprite.color = Color::WHITE;
        }
    }
}
