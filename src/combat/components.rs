use bevy::{
    math::Vec2,
    prelude::{Component, Handle, Image, Resource, TextureAtlasLayout, Transform},
    sprite::Sprite,
    time::Timer,
};

/// Pre-loaded projectile assets — loaded ONCE to avoid per-shot GPU buffer churn
/// that causes `FlushedStagingBuffer::drop` crashes in the Mali Vulkan driver.
#[derive(Resource)]
pub struct CombatAssets {
    pub projectile_image: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, Default)]
#[require(Sprite, Transform)]
pub struct Fireball {
    pub progress: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct FireballAnimation {
    pub timer: Timer,
    pub last_frame: usize,
    pub first_frame: usize,
}

/// Type of projectile (determines sprite and behavior).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProjectileKind {
    /// Homing orb (Mage)
    Orb,
    /// Straight-flying arrow (Archer)
    Arrow,
}

/// Unified projectile component (orbs and arrows share movement logic).
#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Projectile {
    pub direction: Vec2,
    pub kind: ProjectileKind,
    pub speed: f32,
}

#[derive(Component, Default)]
pub struct Spell {
    pub damage: f32,
}

#[derive(Component)]
pub struct Invincible(pub Timer);

#[derive(Component)]
pub struct Knockback {
    pub velocity: Vec2,
    pub timer: Timer,
}

#[derive(Component)]
pub struct DamageFlash(pub Timer);

/// Experience gem dropped by enemies on death.
#[derive(Component)]
pub struct ExperienceGem {
    #[allow(dead_code)]
    pub amount: f32,
}
