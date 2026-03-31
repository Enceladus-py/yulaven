mod abilities;
mod definition;
mod selected;

// Re-exported for use in other modules (UI, combat systems, etc.)
#[allow(unused_imports)]
pub use abilities::{ActiveAbilityKind, AttackType, PassiveAbility};
#[allow(unused_imports)]
pub use definition::CharacterDefinition;
pub use selected::{ActiveAbility, SelectedCharacter};
