use std::fmt::Debug;
use std::marker::PhantomData;
use eframe::{egui, App, Frame, NativeOptions};
use egui_inspect::inspect;
use egui_inspect::{derive::Inspect, Inspect};
use rand::{distributions::Alphanumeric, prelude::SliceRandom, thread_rng, Rng};
struct Testbed {
    entities: Vec<GameEntity>,
    some_string: String,
}
#[derive(Inspect, Debug)]
struct GameEntity {
    name: String,
    position: Vector2,
    hp: i32,
    godmode: bool,
    dir: Dir,
    #[opaque]
    #[allow(dead_code)]
    something_opaque: MyOpaque,
    #[inspect_with(custom_inspect)]
    custom: MyOpaque,
    tuple: TupleStruct,
    generic: Generic<String>,
    phantom: PhantomData<NonInspect>,
    unit: (),
}
struct NonInspect;
#[derive(Inspect, Debug)]
struct TupleStruct(u32);
#[derive(Default, Debug)]
struct MyOpaque {
    field1: i32,
    field2: String,
    field3: f32,
}
#[derive(Inspect, Debug)]
struct Generic<T: Inspect> {
    field: T,
}
fn custom_inspect(o: &mut MyOpaque, ui: &mut egui::Ui, _id_source: u64) {
    loop {}
}
#[derive(Inspect, Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}
impl GameEntity {
    fn rand() -> Self {
        loop {}
    }
}
#[derive(Inspect, Debug)]
struct Vector2 {
    x: f32,
    y: f32,
}
impl Vector2 {
    fn rand() -> Self {
        loop {}
    }
}
impl Default for Testbed {
    fn default() -> Self {
        loop {}
    }
}
impl App for Testbed {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        loop {}
    }
}
fn main() {
    loop {}
}
