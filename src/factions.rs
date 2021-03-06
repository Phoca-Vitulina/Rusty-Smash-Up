#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Faction {
    Pirates,
    Aliens,
    Tricksters,
    Dinosaurs,
    Ninjas,
    Robots,
    Wizards,
    Zombies,
    MagicalGirls,
    IttyCritters,
    MegaTroopers,
    Kaiju,
    Innsmouth,
    MiskatonicUniversity,
    MinionsOfCthulhu,
    ElderThings,
    Vampires,
    Werewolves,
    GiantAnts,
    MadScientists,
    KillerPlants,
    Steampunks,
    BearCavalry,
    Ghosts,
    RussianFairytales,
    PolynesianVoyagers,
    GrimmsFairytales,
    AnansiTales,
    AncientIncas,
    Superheroes,
    Sharks,
    Tornados,
    Dragons,
    MythicGreeks,
}

impl Faction {
    pub fn name(self) -> &'static str {
        match self {
            Faction::Pirates => "Pirates",
            Faction::Aliens => "Aliens",
            Faction::Tricksters => "Tricksters",
            Faction::Dinosaurs => "Dinos",
            Faction::Ninjas => "Ninjas",
            Faction::Robots => "Robots",
            Faction::Wizards => "Wizards",
            Faction::Zombies => "Zombies",
            Faction::MagicalGirls => "Magical Girls",
            Faction::IttyCritters => "Itty Critters",
            Faction::MegaTroopers => "Mega Troopers",
            Faction::Kaiju => "Kaiju",
            Faction::Innsmouth => "Innsmouth",
            Faction::MiskatonicUniversity => "Miskatonic Uni",
            Faction::MinionsOfCthulhu => "Minions of Cthulhu",
            Faction::ElderThings => "Elder Things",
            Faction::Vampires => "Vamps",
            Faction::Werewolves => "Werewolves",
            Faction::GiantAnts => "Giant Ants",
            Faction::MadScientists => "Mad Scientists",
            Faction::KillerPlants => "Killer Plants",
            Faction::Steampunks => "Steampunks",
            Faction::BearCavalry => "Bear Cavalry",
            Faction::Ghosts => "Ghosts",
            Faction::RussianFairytales => "Russian Fairytales",
            Faction::PolynesianVoyagers => "Polynesian Voyagers",
            Faction::GrimmsFairytales => "Grimm's Fairytales",
            Faction::AnansiTales => "Anansi Tales",
            Faction::AncientIncas => "Ancient Incas",
            Faction::Superheroes => "Superheroes",
            Faction::Sharks => "Sharks",
            Faction::Tornados => "Tornados",
            Faction::Dragons => "Dragons",
            Faction::MythicGreeks => "Mythic Greeks",
        }
    }

    pub fn all() -> [Faction; 34] {
        [
            Faction::Pirates,
            Faction::Aliens,
            Faction::Tricksters,
            Faction::Dinosaurs,
            Faction::Ninjas,
            Faction::Robots,
            Faction::Wizards,
            Faction::Zombies,
            Faction::MagicalGirls,
            Faction::IttyCritters,
            Faction::MegaTroopers,
            Faction::Kaiju,
            Faction::Innsmouth,
            Faction::MiskatonicUniversity,
            Faction::MinionsOfCthulhu,
            Faction::ElderThings,
            Faction::Vampires,
            Faction::Werewolves,
            Faction::GiantAnts,
            Faction::MadScientists,
            Faction::KillerPlants,
            Faction::Steampunks,
            Faction::BearCavalry,
            Faction::Ghosts,
            Faction::RussianFairytales,
            Faction::PolynesianVoyagers,
            Faction::GrimmsFairytales,
            Faction::AnansiTales,
            Faction::AncientIncas,
            Faction::Superheroes,
            Faction::Sharks,
            Faction::Tornados,
            Faction::Dragons,
            Faction::MythicGreeks,
        ]
    }
}
