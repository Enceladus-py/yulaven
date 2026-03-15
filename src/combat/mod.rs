use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(crate::GameState::Playing),
            systems::setup_combat_assets,
        )
        .add_systems(
            Update,
            (
                systems::fire_fireballs,
                systems::fire_orbs,
                systems::move_fireballs,
                systems::move_orbs,
                systems::animate_spell,
                systems::apply_knockback,
                systems::handle_invincibility,
                systems::handle_damage_flash,
                systems::handle_spell_collisions,
                systems::handle_enemy_player_collisions,
                systems::handle_death,
            )
                .run_if(in_state(crate::GameState::Playing)),
        );
    }
}
