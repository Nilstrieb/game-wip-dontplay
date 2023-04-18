use egui::Ui;
#[cfg(feature = "derive")]
pub use egui_inspect_derive as derive;
use std::fmt::Debug;
pub trait Inspect: Debug {
    fn inspect(&self, ui: &mut Ui, id_source: u64);
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
}
pub trait UiExt {
    fn inspect<T: Inspect>(&mut self, what: &T, id_source: &mut u64);
    fn inspect_iter_with<'a, I, T, F>(
        &mut self,
        title: &str,
        into_iter: I,
        id_source: &mut u64,
        fun: F,
    ) where
        I: IntoIterator<Item = &'a T>,
        T: 'a,
        F: FnMut(&mut Ui, usize, &T, &mut u64);
    fn inspect_iter_with_mut<'a, I, T, F>(
        &mut self,
        title: &str,
        into_iter: I,
        id_source: &mut u64,
        fun: F,
    ) where
        I: IntoIterator<Item = &'a mut T>,
        T: 'a,
        F: FnMut(&mut Ui, usize, &mut T, &mut u64);
    fn inspect_mut<T: Inspect>(&mut self, what: &mut T, id_source: &mut u64);
    fn property<T: Inspect>(&mut self, name: &str, what: &mut T, id_source: &mut u64);
}
impl UiExt for Ui {
    fn inspect<T: Inspect>(&mut self, what: &T, id_source: &mut u64) {
        loop {}
    }
    fn inspect_iter_with<'a, I, T, F>(
        &mut self,
        title: &str,
        into_iter: I,
        id_source: &mut u64,
        mut fun: F,
    ) where
        I: IntoIterator<Item = &'a T>,
        T: 'a,
        F: FnMut(&mut Ui, usize, &T, &mut u64),
    {
        loop {}
    }
    fn inspect_iter_with_mut<'a, I, T, F>(
        &mut self,
        title: &str,
        into_iter: I,
        id_source: &mut u64,
        mut fun: F,
    ) where
        I: IntoIterator<Item = &'a mut T>,
        T: 'a,
        F: FnMut(&mut Ui, usize, &mut T, &mut u64),
    {
        loop {}
    }
    fn inspect_mut<T: Inspect>(&mut self, what: &mut T, id_source: &mut u64) {
        loop {}
    }
    fn property<T: Inspect>(&mut self, name: &str, what: &mut T, id_source: &mut u64) {
        loop {}
    }
}
impl Inspect for () {
    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        loop {}
    }
}
