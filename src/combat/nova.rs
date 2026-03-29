use bevy::{prelude::*, time::TimerMode};

use crate::{
    combat::components::Knockback,
    constant::{DAMAGE_FLASH_DURATION, ENEMY_KNOCKBACK_DURATION, ENEMY_KNOCKBACK_SPEED},
    core::components::Health,
    enemy::components::{DamageFlash, Enemy},
};

// ── Constants ────────────────────────────────────────────────────────────────
pub const NOVA_COOLDOWN_SECS: f32 = 8.0;
pub const NOVA_RADIUS: f32 = 380.0;
pub const NOVA_DAMAGE: f32 = 40.0;
/// Visual ring expands from 0 → `NOVA_RADIUS` in this many seconds then despawns.
pub const NOVA_RING_DURATION: f32 = 0.35;

// ── Resources ────────────────────────────────────────────────────────────────
#[derive(Resource)]
pub struct NovaCooldown(pub Timer);

impl Default for NovaCooldown {
    fn default() -> Self {
        let mut t = Timer::from_seconds(NOVA_COOLDOWN_SECS, TimerMode::Once);
        // Start ready so the first use is immediately available
        t.tick(std::time::Duration::from_secs_f32(NOVA_COOLDOWN_SECS));
        Self(t)
    }
}

// ── Events ───────────────────────────────────────────────────────────────────
#[derive(Message)]
pub struct NovaEvent {
    pub origin: Vec3,
}

// ── Components ───────────────────────────────────────────────────────────────
/// The expanding visual ring spawned when Nova fires.
#[derive(Component)]
pub struct NovaRing {
    pub lifetime: Timer,
}

/// Marker for Nova UI button.
#[derive(Component)]
pub struct NovaButton;

// ── Systems ──────────────────────────────────────────────────────────────────

/// Tick the cooldown and listen for Space bar (PC) or a dedicated on-screen button (mobile).
pub fn trigger_nova(
    mut cooldown: ResMut<NovaCooldown>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NovaButton>)>,
    player_query: Query<
        (&Transform, &crate::player::character::ActiveAbility),
        With<crate::player::components::Player>,
    >,
    mut ev_nova: MessageWriter<NovaEvent>,
) {
    cooldown.0.tick(time.delta());

    if !cooldown.0.is_finished() {
        return;
    }

    let mut pressed = keyboard.just_pressed(KeyCode::Space);
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            pressed = true;
        }
    }

    if pressed
        && let Ok((player_transform, ability)) = player_query.single()
        && ability.kind == crate::player::character::SelectedCharacter::Warlock
    {
        ev_nova.write(NovaEvent {
            origin: player_transform.translation,
        });
        cooldown.0.reset();
    }
}

/// Applies damage + knockback to every enemy within `NOVA_RADIUS` of the origin.
#[allow(clippy::type_complexity)]
pub fn apply_nova(
    mut commands: Commands,
    mut ev_nova: MessageReader<NovaEvent>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
    player_query: Query<&crate::player::components::PlayerStats>,
) {
    let damage_mult = player_query
        .single()
        .map(|s| s.damage_multiplier)
        .unwrap_or(1.0);

    for ev in ev_nova.read() {
        // Spawn the visual ring at the origin
        commands.spawn((
            Sprite {
                color: Color::srgba(0.6, 0.2, 1.0, 0.55),
                custom_size: Some(Vec2::splat(1.0)), // starts tiny, scaled by transform
                ..Default::default()
            },
            Transform {
                translation: ev.origin,
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            NovaRing {
                lifetime: Timer::from_seconds(NOVA_RING_DURATION, TimerMode::Once),
            },
        ));

        // Damage every enemy in range
        for (enemy_entity, enemy_transform, mut health) in &mut enemy_query {
            let dist = enemy_transform.translation.distance(ev.origin);
            if dist <= NOVA_RADIUS {
                health.0 -= NOVA_DAMAGE * damage_mult;

                let knockback_dir = (enemy_transform.translation - ev.origin)
                    .truncate()
                    .normalize_or_zero();

                commands.entity(enemy_entity).try_insert((
                    DamageFlash(Timer::from_seconds(DAMAGE_FLASH_DURATION, TimerMode::Once)),
                    Knockback {
                        velocity: knockback_dir * ENEMY_KNOCKBACK_SPEED * 2.5, // stronger than normal
                        timer: Timer::from_seconds(ENEMY_KNOCKBACK_DURATION * 3.0, TimerMode::Once),
                    },
                ));
            }
        }
    }
}

/// Animates the `NovaRing` — scales it up to `NOVA_RADIUS` over its lifetime, then despawns.
pub fn animate_nova_ring(
    mut commands: Commands,
    mut ring_query: Query<(Entity, &mut Transform, &mut Sprite, &mut NovaRing)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut sprite, mut ring) in &mut ring_query {
        ring.lifetime.tick(time.delta());

        let t = ring.lifetime.fraction(); // 0.0 → 1.0

        // Scale grows from 1 → NOVA_RADIUS*2 pixels (the sprite is 1px, so scale = world size)
        let world_size = t * NOVA_RADIUS * 2.0;
        transform.scale = Vec3::splat(world_size.max(1.0));

        // Fade out as it expands
        let alpha = (1.0 - t) * 0.7;
        sprite.color.set_alpha(alpha);

        if ring.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// HUD code removed from here, moving to hud.rs
