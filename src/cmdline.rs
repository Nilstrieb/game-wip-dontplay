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
        WorldPos {
            x: self.x,
            y: self.y,
        }
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
        let words = std::iter::once(" ").chain(cmdline.split_whitespace());
        Ok(Self::try_parse_from(words)?)
    }

    pub(crate) fn dispatch(self) -> Dispatch {
        match self {
            CmdLine::Quit => Dispatch::Cmd(Cmd::QuitApp),
            CmdLine::Freecam => Dispatch::Cmd(Cmd::ToggleFreecam),
            CmdLine::Clear => Dispatch::ClearConsole,
            CmdLine::Tp(tp) => Dispatch::Cmd(Cmd::TeleportPlayer {
                pos: tp.to_world_pos(),
                relative: tp.rel,
            }),
            CmdLine::Spawn => Dispatch::Cmd(Cmd::TeleportPlayerSpawn),
            CmdLine::Give(give) => Dispatch::Cmd(Cmd::GiveItemByName(give.name)),
        }
    }
}
