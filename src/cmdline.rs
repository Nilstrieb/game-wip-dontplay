use clap::Parser;
use crate::{command::Cmd, math::WorldPos};
#[derive(Parser)]
pub(crate) enum CmdLine {
    Quit,
    Freecam,
    Clear,
    Tp(Tp),
    Spawn,
    Give(Give),
}
#[derive(Parser)]
pub(crate) struct Tp {}
#[derive(Parser)]
pub(crate) struct Give {}
