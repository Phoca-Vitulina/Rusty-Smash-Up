use crate::factions::Faction;
use crate::graphics::TextureCache;
use crate::support::{init, System};
use imgui::*;
use rand::Rng;

mod factions;
mod graphics;
mod support;

pub fn sanitize(s: &str) -> String {
    let mut ss = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'a'..='z' => ss.push(c),
            'A'..='Z' => ss.push(c.to_ascii_lowercase()),
            '0'..='9' => ss.push(c),
            _ => (),
        }
    }
    ss
}

pub fn faction_icon_path(faction: Faction) -> String {
    format!("resources/icons/{}.png", sanitize(faction.name()))
}

pub fn choose_index_nr(choices: usize, used: &[usize]) -> usize {
    loop {
        let i = rand::thread_rng().gen_range(0, choices);
        if used.iter().any(|j| *j == i) {
            continue;
        };
        break i;
    }
}

pub fn choose_players_factions(factions: &[Faction], used: &mut Vec<usize>) {
    for j in 0..2 {
        let i = choose_index_nr(factions.len(), &used);
        used.push(i);
        print!(" {}", factions[i].name());
        if j != 1 {
            print!(" &")
        }
    }
}

pub fn main() {
    let system: System = init("Smash Up");

    let texture_cache = TextureCache::new(&system);

    let faction_icon = |tc: &TextureCache, f: Faction| {
        let icon_path = faction_icon_path(f);
        Image::new(tc.get(&icon_path), [50f32, 50f32])
    };

    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                faction_icon(&texture_cache, Faction::Aliens).build(ui);
                faction_icon(&texture_cache, Faction::BearCavalry).build(ui);
                faction_icon(&texture_cache, Faction::IttyCritters).build(ui);
                faction_icon(&texture_cache, Faction::MinionsOfCthulhu).build(ui);
            });
    });

    let all_factions = Faction::all();
    let players = ["Dinah", "Tim"];
    let mut used: Vec<usize> = vec![];

    for name in players.iter() {
        print!("\n");
        print!("{}, you will battle with", name);
        choose_players_factions(&all_factions, &mut used);
        print!("\n");
    }
    print!("\n");
}
