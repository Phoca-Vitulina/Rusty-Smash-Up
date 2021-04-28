use crate::support::init;
use crate::support::System;
use glium::backend::Facade;
use glium::Texture2d;
use image::io::Reader as ImageReader;
use imgui::*;
use rand::Rng;
use std::rc::Rc;

mod support;

pub fn choose_index_nr(choices: usize, used: &[usize]) -> usize {
    loop {
        let i = rand::thread_rng().gen_range(0, choices);
        if used.iter().any(|j| *j == i) {
            continue;
        };
        break i;
    }
}

pub fn choose_players_factions(factions: &[&str], used: &mut Vec<usize>) {
    for j in 0..2 {
        let i = choose_index_nr(factions.len(), &used);
        used.push(i);
        print!(" {}", factions[i]);
        if j != 1 {
            print!(" &")
        }
    }
}

fn load_image(name: &str) -> image::RgbImage {
    let path = format!("resources/icons/{}.png", name);
    let img = ImageReader::open(&path).expect(&format!("Oops! I can't open {}", path));
    let dyn_img = img.decode().expect("Failed to decode image");
    let rgb_img = dyn_img.into_rgb8();
    rgb_img
}

fn image_into_texture(img: image::RgbImage, system: &mut System) -> TextureId {
    let w = img.width();
    let h = img.height();
    let rawimg = glium::texture::RawImage2d::from_raw_rgb(img.into_raw(), (w, h));
    let ctx = system.display.get_context();
    let tex = Texture2d::new(ctx, rawimg).expect("Failed to create texture");
    let ref mut textures = system.renderer.textures();
    let new_tex = imgui_glium_renderer::Texture {
        texture: Rc::new(tex),
        sampler: glium::uniforms::SamplerBehavior::default(),
    };
    textures.insert(new_tex)
}

pub fn main() {
    let mut system = init(file!());

    let img = load_image("aliens");
    let tex_id = image_into_texture(img, &mut system);

    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                Image::new(tex_id, [50f32, 50f32]).build(ui);
            });
    });

    let factions = [
        "pirates",
        "aliens",
        "tricksters",
        "dinos",
        "ninjas",
        "robots",
        "wizards",
        "zombies",
        "magical girls",
        "itty critters",
        "mega troopers",
        "kaiju",
        "innsmouth",
        "miskatonic uni",
        "minions of cthulhu",
        "elder things",
        "vamps",
        "werewolves",
        "giant ants",
        "mad scientists",
        "killer plants",
        "steam punks",
        "bear cavalry",
        "ghosts",
        "russian fairytales",
        "polynesian voyagers",
        "grimms' fairytales",
        "anansi tales",
        "ancient incas",
    ];
    let players = ["Dinah", "Tim"];
    let mut used: Vec<usize> = vec![];

    for name in players.iter() {
        print!("\n");
        print!("{}, you will battle with", name);
        choose_players_factions(&factions, &mut used);
        print!("\n");
    }
    print!("\n");
}
