/// Defines the attack behavior for a character.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    /// Mage: charges orbs → fires homing fireball
    ChargedFireball,
    /// Archer: rapid auto-orbs targeting nearby enemies
    RapidOrbs,
    /// Warlock: close-range melee drain, no projectile
    MeleeDrain,
}

/// Defines the passive ability for a character.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PassiveAbility {
    /// Mage: collect orb charges, fire fireball at 5 charges
    OrbCharge,
    /// Archer: continuous rapid orb attacks (always active)
    RapidFire,
    /// Warlock: heal on every enemy kill
    LifeDrain,
}

/// Defines the active ability for a character.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveAbilityKind {
    /// Mage: teleport forward
    Blink,
    /// Archer: fire 8 orbs in a ring
    ArrowRain,
    /// Warlock: `AoE` damage + self heal
    VoidNova,
}
