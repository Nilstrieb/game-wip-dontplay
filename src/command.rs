/// A command that can change application or game state
pub(crate) enum Cmd {
    /// Quit the application
    QuitApp,
    ToggleFreecam,
    TeleportPlayer {},
    TeleportPlayerSpawn,
    GiveItemByName(),
}
pub(crate) type CmdVec = Vec<Cmd>;
