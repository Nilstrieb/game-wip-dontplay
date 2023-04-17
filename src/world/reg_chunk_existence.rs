use std::{fs::File, io::Read, path::Path};
#[derive(Clone, Copy)]
pub struct ExistenceBitset(pub u64);
impl ExistenceBitset {
    pub const EMPTY: Self = Self(0);
    pub fn read_from_file(f: &mut File) -> ExistenceBitset {
        loop {}
    }
    pub fn read_from_fs(path: &Path) -> ExistenceBitset {
        loop {}
    }
}
impl std::fmt::Debug for ExistenceBitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
