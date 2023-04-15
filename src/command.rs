use crate::math::WorldPos;

/// A command that can change application or game state
pub enum Cmd {
    /// Quit the application
    QuitApp,
    ToggleFreecam,
    TeleportPlayer {
        pos: WorldPos,
        relative: bool,
    },
    TeleportPlayerSpawn,
    GiveItemByName(String),
}

pub type CmdVec = Vec<Cmd>;
