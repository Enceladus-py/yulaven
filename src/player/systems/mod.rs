mod animation;
mod dash_trail;
mod movement;
mod teleport;

pub use animation::animate_player;
pub use dash_trail::animate_dash_trail;
pub use movement::move_player;
pub use teleport::handle_teleportation;
