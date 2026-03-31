use bevy::prelude::*;

use super::abilities::{ActiveAbilityKind, AttackType, PassiveAbility};
use super::selected::SelectedCharacter;

/// Complete character definition with all stats and configurations.
pub struct CharacterDefinition {
    // Visual
    pub sprite_path: &'static str,
    pub display_name: &'static str,
    pub accent_color: Color,
    pub card_color: Color,

    // Stats
    pub base_health: f32,
    pub base_speed_multiplier: f32,
    pub base_damage_multiplier: f32,
    pub base_attack_range: f32,

    // Attack Configuration
    pub attack_type: AttackType,
    pub attack_interval: f32,
    pub orb_charges_start: u8,

    // Passive Ability
    pub passive: PassiveAbility,

    // Active Ability
    pub active: ActiveAbilityKind,
    pub active_cooldown_secs: f32,
    pub active_description: &'static str,
    pub passive_description: &'static str,
}

impl CharacterDefinition {
    pub const MAGE: Self = Self {
        sprite_path: "outline/MiniMage.png",
        display_name: "Mage",
        accent_color: Color::srgb(0.3, 0.6, 1.0),
        card_color: Color::srgb(0.1, 0.15, 0.35),
        base_health: 100.0,
        base_speed_multiplier: 1.0,
        base_damage_multiplier: 1.0,
        base_attack_range: 500.0,
        attack_type: AttackType::ChargedFireball,
        attack_interval: 0.8,
        orb_charges_start: 0,
        passive: PassiveAbility::OrbCharge,
        active: ActiveAbilityKind::Blink,
        active_cooldown_secs: 5.0,
        active_description: "Blink — Teleport forward",
        passive_description: "Charges orbs → launches homing Fireball",
    };

    pub const ARCHER: Self = Self {
        sprite_path: "outline/MiniArcherMan.png",
        display_name: "Archer",
        accent_color: Color::srgb(0.2, 0.9, 0.4),
        card_color: Color::srgb(0.08, 0.22, 0.1),
        base_health: 75.0,
        base_speed_multiplier: 1.25,
        base_damage_multiplier: 0.8,
        base_attack_range: 650.0,
        attack_type: AttackType::RapidOrbs,
        attack_interval: 0.7,
        orb_charges_start: 255,
        passive: PassiveAbility::RapidFire,
        active: ActiveAbilityKind::ArrowRain,
        active_cooldown_secs: 10.0,
        active_description: "Arrow Rain — Fire orbs in all directions",
        passive_description: "Rapid auto-orbs targeting nearby enemies",
    };

    pub const WARLOCK: Self = Self {
        sprite_path: "outline/MiniSwordMan.png",
        display_name: "Warlock",
        accent_color: Color::srgb(0.7, 0.2, 1.0),
        card_color: Color::srgb(0.18, 0.05, 0.28),
        base_health: 150.0,
        base_speed_multiplier: 0.85,
        base_damage_multiplier: 1.5,
        base_attack_range: 80.0,
        attack_type: AttackType::MeleeDrain,
        attack_interval: 1.2,
        orb_charges_start: 0,
        passive: PassiveAbility::LifeDrain,
        active: ActiveAbilityKind::VoidNova,
        active_cooldown_secs: 10.0,
        active_description: "Void Nova — AoE damage + heal",
        passive_description: "Life Drain: heal on every kill",
    };

    pub fn from_selected(character: SelectedCharacter) -> Self {
        match character {
            SelectedCharacter::Mage => Self::MAGE,
            SelectedCharacter::Archer => Self::ARCHER,
            SelectedCharacter::Warlock => Self::WARLOCK,
        }
    }
}
