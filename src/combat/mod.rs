use bevy::prelude::*;

pub mod assets;
pub mod collision;
pub mod components;
pub mod death;
pub mod effects;
pub mod nova;
pub mod projectiles;
pub mod warlock;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<nova::NovaEvent>()
            .init_resource::<nova::NovaCooldown>()
            .add_systems(
                OnExit(crate::GameState::CharacterSelect),
                assets::setup_combat_assets,
            )
            .add_systems(
                Update,
                (
                    // Projectiles
                    projectiles::fire_fireballs,
                    projectiles::fire_orbs,
                    projectiles::move_fireballs,
                    projectiles::move_projectiles,
                    assets::animate_spell,
                    // Effects
                    effects::apply_knockback,
                    effects::handle_invincibility,
                    effects::handle_damage_flash,
                    // Collisions
                    collision::handle_spell_collisions,
                    collision::handle_enemy_player_collisions,
                    // Death
                    death::handle_death,
                    // Warlock
                    warlock::warlock_melee_drain,
                    warlock::warlock_life_drain,
                    // Nova
                    nova::trigger_nova,
                    nova::apply_nova,
                    nova::animate_nova_ring,
                )
                    .run_if(in_state(crate::GameState::Playing)),
            );
    }
}
