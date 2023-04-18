use egui::Ui;
#[cfg(feature = "derive")]
pub use egui_inspect_derive as derive;
use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
    fmt::Debug,
    marker::PhantomData,
};
pub trait Inspect: Debug {
    fn inspect(&self, ui: &mut Ui, id_source: u64);
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
}
impl Inspect for String {
    fn inspect_mut(&mut self, ui: &mut Ui, _id_source: u64) {
        loop {}
    }
    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        loop {}
    }
}
impl<T: Inspect> Inspect for Vec<T> {
    fn inspect_mut(&mut self, ui: &mut Ui, mut id_source: u64) {
        loop {}
    }
    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
}
impl<T: Inspect> Inspect for Option<T> {
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
}
impl Inspect for OsString {
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        loop {}
    }
}
impl<T: Inspect> Inspect for HashSet<T> {
    fn inspect(&self, ui: &mut Ui, mut id_source: u64) {
        loop {}
    }
}
impl<T: Inspect> Inspect for &mut T {
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
}
impl<T: Inspect, const N: usize> Inspect for [T; N] {
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
}
impl<K: Debug, V: Inspect, S> Inspect for HashMap<K, V, S> {
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
    fn inspect(&self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
}
impl<'a> Inspect for &'a str {
    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        loop {}
    }
}
impl Inspect for bool {
    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        loop {}
    }
    fn inspect_mut(&mut self, ui: &mut Ui, _id_source: u64) {
        loop {}
    }
}
macro_rules! impl_num_inspect {
    ($($ty:ty),*) => {
        $(impl Inspect for $ty { fn inspect_mut(& mut self, ui : & mut Ui, _id_source :
        u64) { ui.add(egui::DragValue::new(self)); } fn inspect(& self, ui : & mut Ui,
        _id_source : u64) { ui.label(self.to_string()); } })*
    };
}
impl_num_inspect!(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize);
impl<T, U> Inspect for (T, U)
where
    T: Inspect,
    U: Inspect,
{
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
    fn inspect(&self, ui: &mut Ui, id_source: u64) {
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
macro_rules! inspect_iter_with_body {
    ($self:expr, $title:expr, $into_iter:expr, $id_source:expr, $fun:expr) => {
        egui::CollapsingHeader::new($title)
            .id_source(*$id_source)
            .show($self, |ui| {
                for (i, item) in $into_iter.into_iter().enumerate() {
                    $fun(ui, i, item, $id_source);
                }
            });
    };
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
impl<T> Inspect for PhantomData<T> {
    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        loop {}
    }
}
impl Inspect for () {
    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        loop {}
    }
}
#[macro_export]
macro_rules! inspect {
    ($ui:expr, $($rest:tt)*) => {
        { let mut id_source = 0; $crate ::inspect_helper! { $ui id_source $($rest)* } }
    };
}
#[macro_export]
macro_rules! inspect_helper {
    ($ui:tt $id_source:tt) => {};
    ($ui:tt $id_source:tt $name:literal : $arg:expr $(, $($rest:tt)*)?) => {
        $crate ::UiExt::property($ui, $name, & mut $arg, & mut $id_source); $($crate
        ::inspect_helper! { $ui $id_source $($rest)* })?
    };
    ($ui:tt $id_source:tt $arg:expr $(, $($rest:tt)*)?) => {
        $crate ::UiExt::property($ui, ::core::stringify!($arg), & mut $arg, & mut
        $id_source); $($crate ::inspect_helper! { $ui $id_source $($rest)* })?
    };
}
