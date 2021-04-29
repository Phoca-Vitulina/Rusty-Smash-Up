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

pub fn choose_players_factions(factions: &[Faction], used: &mut Vec<usize>) -> (Faction, Faction) {
    let i = choose_index_nr(factions.len(), &used);
    let first_faction = factions[i];
    used.push(i);
    let i = choose_index_nr(factions.len(), &used);
    let second_faction = factions[i];
    used.push(i);
    (first_faction, second_faction)
}

pub fn main() {
    let size = (500f64, 500f64);
    let system: System = init("Smash Up", size);

    let texture_cache = TextureCache::new(&system);

    let faction_icon = |tc: &TextureCache, f: Faction| {
        let icon_path = faction_icon_path(f);
        Image::new(tc.get(&icon_path), [100f32, 100f32])
    };

    let all_factions = Faction::all();
    let players = ["Dinah", "Tim"];
    let mut used: Vec<usize> = vec![];

    let mut player_factions: Vec<(String, Faction, Faction)> = Vec::new();
    for name in players.iter() {
        let (f1, f2) = choose_players_factions(&all_factions, &mut used);
        player_factions.push((String::from(*name), f1, f2));
    }

    system.main_loop(move |_, ui| {
        Window::new(im_str!("x"))
            .size([10000.0, 10000.0], Condition::Always)
            //.size(ui.window_size(), Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .focused(true)
            .flags(
                WindowFlags::NO_TITLE_BAR
                    //| WindowFlags::ALWAYS_AUTO_RESIZE
                    | WindowFlags::NO_COLLAPSE
                    | WindowFlags::NO_MOVE,
            )
            .build(ui, || {
                for (name, f1, f2) in player_factions.iter() {
                    let s = format!(
                        "{}, you will battle with {} & {}",
                        name,
                        f1.name(),
                        f2.name()
                    );
                    ui.text(ImString::new(s));
                    faction_icon(&texture_cache, *f1).build(ui);
                    ui.same_line_with_spacing(10.0, 150.0);
                    faction_icon(&texture_cache, *f2).build(ui);
                    ui.separator();
                }
                // ui.text(im_str!(""));
                // faction_icon(&texture_cache, Faction::Aliens).build(ui);
            });
    });

    print!("\n");
}
