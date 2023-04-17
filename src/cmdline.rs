use clap::Parser;
use crate::{command::Cmd, math::WorldPos};
#[derive(Parser)]
pub enum CmdLine {
    Quit,
    Freecam,
    Clear,
    Tp(Tp),
    Spawn,
    Give(Give),
}
#[derive(Parser)]
pub struct Tp {
    x: u32,
    y: u32,
    /// Relative to current position
    #[arg(short, long)]
    rel: bool,
}
impl Tp {
    fn to_world_pos(&self) -> WorldPos {
        loop {}
    }
}
#[derive(Parser)]
pub struct Give {
    name: String,
}
pub enum Dispatch {
    Cmd(Cmd),
    ClearConsole,
}
impl CmdLine {
    pub fn parse_cmdline(cmdline: &str) -> anyhow::Result<Self> {
        loop {}
    }
    pub(crate) fn dispatch(self) -> Dispatch {
        loop {}
    }
}
