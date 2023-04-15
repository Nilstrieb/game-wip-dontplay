use std::{collections::HashMap, path::Path};

use crate::math::IntRect;
use sfml::{graphics::Texture, SfBox};
use texture_packer::{texture::Texture as _, TexturePacker, TexturePackerConfig};

pub type RectMap = HashMap<String, IntRect>;

#[derive(Debug)]
pub struct AtlasBundle {
    pub tex: SfBox<Texture>,
    // Key could be `tiles/dirt` for example, derived from folder+filename without extension
    pub rects: RectMap,
}

impl AtlasBundle {
    pub fn new() -> anyhow::Result<Self> {
        let cfg = TexturePackerConfig {
            max_width: 4096,
            max_height: 4096,
            allow_rotation: false,
            border_padding: 0,
            texture_padding: 0,
            texture_extrusion: 0,
            trim: true,
            texture_outlines: false,
        };
        let mut packer = TexturePacker::new_skyline(cfg);
        walk_graphics(|path| {
            let img = image::open(path).unwrap();
            let key = path_img_key(path);
            packer.pack_own(key, img).unwrap();
            dbg!(path);
        });
        let mut rects = HashMap::new();
        let mut tex = Texture::new().unwrap();
        log::info!(
            "Texture atlas size is: {}x{}",
            packer.width(),
            packer.height()
        );
        if !tex.create(packer.width(), packer.height()) {
            panic!("Failed to create texture");
        }
        let pixbuf = make_pix_buf(&packer);
        unsafe {
            tex.update_from_pixels(&pixbuf, packer.width(), packer.height(), 0, 0);
        }
        for (k, frame) in packer.get_frames() {
            rects.insert(
                k.clone(),
                IntRect {
                    x: frame.frame.x as i32,
                    y: frame.frame.y as i32,
                    w: frame.frame.w as i32,
                    h: frame.frame.h as i32,
                },
            );
        }
        Ok(AtlasBundle { tex, rects })
    }
}

fn make_pix_buf(packer: &TexturePacker<image::DynamicImage, String>) -> Vec<u8> {
    let (w, h) = (packer.width(), packer.height());
    let px_size = 4;
    let mut vec = vec![0; w as usize * h as usize * px_size as usize];
    dbg!(w, h);
    for y in 0..h {
        for x in 0..w {
            let idx = ((y * w + x) * px_size) as usize;
            if let Some(px) = packer.get(x, y) {
                vec[idx..idx + px_size as usize].copy_from_slice(&px.0);
            }
        }
    }
    vec
}

fn path_img_key(path: &Path) -> String {
    let mut rev_iter = path.components().rev();
    let fname = rev_iter.next().unwrap();
    let folder = rev_iter.next().unwrap();
    let fname: &Path = fname.as_ref();
    let folder: &Path = folder.as_ref();
    folder
        .join(fname.file_stem().unwrap())
        .display()
        .to_string()
}

#[test]
fn test_path_img_key() {
    assert_eq!(
        &path_img_key("/home/person/res/graphics/tiles/foo.png".as_ref()),
        "tiles/foo"
    );
}

fn walk_graphics(mut f: impl FnMut(&Path)) {
    for en in walkdir::WalkDir::new("res/graphics") {
        let en = en.unwrap();
        if en.file_type().is_file() {
            f(en.path());
        }
    }
}
