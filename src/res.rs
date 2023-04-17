use sfml::audio::Music;
use crate::texture_atlas::AtlasBundle;
#[derive(Debug)]
pub(crate) struct Res {
    pub(crate) atlas: AtlasBundle,
    pub(crate) surf_music: Music<'static>,
}
