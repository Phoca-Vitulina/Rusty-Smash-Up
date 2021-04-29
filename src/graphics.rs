use crate::support::System;
use glium::backend::Facade;
use glium::Texture2d;
use image::io::Reader as ImageReader;
use imgui::TextureId;
use imgui_glium_renderer::Renderer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn load_image(path: &str) -> image::RgbImage {
    let img = ImageReader::open(&path).expect(&format!("Oops! I can't open {}", path));
    let dyn_img = img.decode().expect("Failed to decode image");
    let rgb_img = dyn_img.into_rgb8();
    rgb_img
}

fn image_into_texture(
    img: image::RgbImage,
    display: &glium::Display,
    renderer: &mut Renderer,
) -> TextureId {
    let w = img.width();
    let h = img.height();
    let rawimg = glium::texture::RawImage2d::from_raw_rgb(img.into_raw(), (w, h));
    let ctx = display.get_context();
    let tex = Texture2d::new(ctx, rawimg).expect("Failed to create texture");
    let ref mut textures = renderer.textures();
    let new_tex = imgui_glium_renderer::Texture {
        texture: Rc::new(tex),
        sampler: glium::uniforms::SamplerBehavior::default(),
    };
    textures.insert(new_tex)
}

fn load_texture(image_path: &str, display: &glium::Display, renderer: &mut Renderer) -> TextureId {
    let img = load_image(image_path);
    image_into_texture(img, display, renderer)
}

fn unload_texture(id: TextureId, renderer: &mut Renderer) {
    let ref mut textures = renderer.textures();
    textures.remove(id);
}

pub struct TextureCache {
    textures: RefCell<HashMap<String, TextureId>>,
    display: Rc<glium::Display>,
    renderer: Rc<RefCell<Renderer>>,
}

impl TextureCache {
    pub fn new(sys: &System) -> TextureCache {
        TextureCache {
            textures: RefCell::new(HashMap::new()),
            display: sys.display.clone(),
            renderer: sys.renderer.clone(),
        }
    }

    pub fn get(self: &Self, image_path: &str) -> TextureId {
        let ref mut textures = self.textures.borrow_mut();
        match textures.get(image_path) {
            Some(texture_id) => texture_id.clone(),
            None => {
                let ref display = self.display.as_ref();
                let ref mut renderer = self.renderer.borrow_mut();
                let texture_id = load_texture(image_path, display, renderer);
                textures.insert(image_path.to_owned(), texture_id.clone());
                texture_id
            }
        }
    }
}

impl Drop for TextureCache {
    fn drop(self: &mut Self) {
        let ref mut renderer = self.renderer.borrow_mut();
        for (_, t) in &*self.textures.borrow() {
            unload_texture(t.clone(), renderer);
        }
    }
}
