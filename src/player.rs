use egui_inspect::derive::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Inspect, Serialize, Deserialize)]
pub(crate) struct Player {}
