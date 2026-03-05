use bevy::prelude::Component;

#[derive(Component)]
pub struct ExperienceGem {
    pub amount: f32,
}

#[derive(Component)]
pub struct PlayerStats {
    pub level: u32,
    pub current_xp: f32,
    pub required_xp: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            level: 1,
            current_xp: 0.0,
            required_xp: 100.0,
        }
    }
}
