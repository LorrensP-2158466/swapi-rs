use std::sync::Arc;
use tokio::sync::Mutex;

use crate::starwars::models::Episode;
use async_graphql::*;
use slab::Slab;

// Dit kunnen we eigenlijk zien als een soort database connectie
// of een connectie naar een andera api endpoint
// of ...., ge snapt me wel
// dus ik doe hier dingen die raar kunnen lijken maar eignelijk moet ge kijken naar de code
// in `models`, want dat is wat we ook moeten doen. Het is daar ook beter uitgelegd

// Dit is het rare van deze "api", we combineren eigenljk humans and droids in 1 struct
// wat alles beetje gemakkelijker maakt
// ik kon ook een trait maken (denk Abstracte klasse)
// maar zo werken api's niet...
#[derive(Clone)]
pub(crate) struct APICharacter {
    // basicly to determine if it is a human or a droid
    // the fucked up part
    pub is_human: bool,

    /// id of this character
    pub id: String,

    /// name of this character
    pub name: String,

    /// integer id's of character friends
    /// maps in StarWars.characters
    pub friends: Vec<usize>,

    /// all the episodes this character appeared in
    pub appears_in: Vec<Episode>,

    /// Optional Home planet of  a Human
    pub home_planet: Option<usize>,

    /// the starship of a Human
    pub star_ship: Option<usize>,

    /// primary function of droid
    pub primary_function: Option<String>,

    /// mass of character (i.e. weight) in kg
    pub mass: usize,
}

// wil gewoon es met een builder pattern werken
// heb mezelf gwn meer werk gegeven eigenlijk

impl APICharacter {
    pub fn build(id: impl Into<String>, name: impl Into<String>) -> Self {
        APICharacter {
            is_human: false,
            id: id.into(),
            name: name.into(),
            friends: vec![],
            appears_in: vec![],
            home_planet: None,
            star_ship: None,
            primary_function: None,
            mass: 0,
        }
    }

    pub fn is_human(mut self) -> Self {
        self.is_human = true;
        self
    }

    pub fn is_droid(mut self) -> Self {
        self.is_human = false;
        self
    }

    pub fn set_friends(mut self, friends: Vec<usize>) -> Self {
        self.friends = friends;
        self
    }

    pub fn appeared_in(mut self, episodes: Vec<Episode>) -> Self {
        self.appears_in = episodes;
        self
    }

    pub fn home_planet(mut self, planet: usize) -> Self {
        self.home_planet = Some(planet);
        self
    }

    pub fn star_ship(mut self, starship: usize) -> Self {
        self.star_ship = Some(starship);
        self
    }

    pub fn primary_function(mut self, function: String) -> Self {
        self.primary_function = Some(function);
        self
    }

    pub fn mass(mut self, m: usize) -> Self {
        self.mass = m;
        self
    }
}

#[derive(Clone)]
pub struct APIStarShip {
    /// id of StarShop
    pub id: String,

    /// name of StarShip
    pub name: String,

    /// length of StarShip in meters
    pub length: f64,
}

#[derive(Clone)]
pub struct APIPlanet {
    /// id of planet
    pub id: String,

    /// the name of the planet
    pub name: String,

    pub climate: String,

    /// in kilometers
    pub diameter: usize,

    /// description
    pub gravity: String,

    pub population: usize,

    /// standard (sw) hours
    pub rotation_period: usize,

    /// standard (sw) days
    pub orbital_period: usize,
}

pub struct StarWarsAPI {
    // id counters for insertion
    char_id_counter: usize,
    starship_id_counter: usize,
    planet_id_counter: usize,

    luke_idx: usize,
    r2d2_idx: usize,

    // seperate locks for more performance haha
    characters: Arc<Mutex<Slab<APICharacter>>>,
    starships: Arc<Mutex<Slab<APIStarShip>>>,
    planets: Arc<Mutex<Slab<APIPlanet>>>,
}

impl Default for StarWarsAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl StarWarsAPI {
    pub fn new() -> Self {
        let mut starships = Slab::with_capacity(7);
        let xwing = starships.insert(APIStarShip {
            id: "1".into(),
            name: "X-Wing".into(),
            length: 12.49,
        });
        let tantive = starships.insert(APIStarShip {
            id: "2".into(),
            name: "Tantive IV".into(),
            length: 126.,
        });
        let tie = starships.insert(APIStarShip {
            id: "3".into(),
            name: "Tie Figter".into(),
            length: 9.2,
        });
        let death_star = starships.insert(APIStarShip {
            id: "4".into(),
            name: "Death Star".into(),
            length: 12.49,
        });
        let falcon = starships.insert(APIStarShip {
            id: "5".into(),
            name: "Millenium Falcon".into(),
            length: 34.75,
        });

        let mut characters = Slab::with_capacity(7);

        let luke = characters.insert(
            APICharacter::build("1", "Luke Skywalker")
                .is_human()
                .appeared_in(vec![Episode::Empire, Episode::NewHope, Episode::Jedi])
                .star_ship(xwing)
                .mass(77),
        );
        let vader = characters.insert(
            APICharacter::build("2", "Darth Vader")
                .is_human()
                .star_ship(tie)
                .appeared_in(vec![Episode::Empire, Episode::NewHope, Episode::Jedi])
                .mass(120),
        );
        let han = characters.insert(
            APICharacter::build("3", "Han Solo")
                .is_human()
                .appeared_in(vec![Episode::Empire, Episode::NewHope, Episode::Jedi])
                .star_ship(falcon)
                .mass(85),
        );
        let leia = characters.insert(
            APICharacter::build("4", "Leia Organa")
                .is_human()
                .star_ship(tantive)
                .appeared_in(vec![Episode::Empire, Episode::NewHope, Episode::Jedi])
                .mass(60),
        );
        let tarkin = characters.insert(
            APICharacter::build("5", "Wilhuff Tarkin")
                .is_human()
                .star_ship(death_star)
                .appeared_in(vec![Episode::Empire, Episode::NewHope, Episode::Jedi])
                .mass(90),
        );
        let r2 = characters.insert(
            APICharacter::build("6", "R2-D2")
                .is_droid()
                .appeared_in(vec![Episode::Empire, Episode::NewHope, Episode::Jedi])
                .mass(32)
                .primary_function("Astromech".into()),
        );
        let treepio = characters.insert(
            APICharacter::build("7", "C-3PO")
                .is_droid()
                .appeared_in(vec![Episode::Empire, Episode::NewHope, Episode::Jedi])
                .mass(75)
                .primary_function("Protocol".into()),
        );

        characters[luke].friends = vec![leia, han, r2, treepio];
        characters[leia].friends = vec![luke, han, r2, treepio];
        characters[han].friends = vec![leia, luke, r2, treepio];
        characters[r2].friends = vec![luke, leia, han, treepio];
        characters[treepio].friends = vec![luke, han, leia, r2];

        characters[tarkin].friends = vec![vader];
        characters[vader].friends = vec![tarkin];

        let mut planets = Slab::with_capacity(7);

        let tatooine = planets.insert(APIPlanet {
            id: "1".into(),
            climate: "arid".into(),
            diameter: 10_465,
            gravity: "Standard".into(),
            name: "Tatooine".into(),
            population: 200_000,
            rotation_period: 23,
            orbital_period: 304,
        });

        let alderaan = planets.insert(APIPlanet {
            id: "2".into(),
            climate: "arid".into(),
            diameter: 10_465,
            gravity: "Temperate".into(),
            name: "Alderaan".into(),
            population: 2_000_000_000,
            rotation_period: 24,
            orbital_period: 364,
        });

        characters[luke].home_planet = Some(tatooine);
        characters[vader].home_planet = Some(tatooine);
        characters[leia].home_planet = Some(alderaan);

        StarWarsAPI {
            char_id_counter: 7,
            starship_id_counter: 6,
            planet_id_counter: 3,
            luke_idx: luke,
            r2d2_idx: r2,
            characters: Arc::new(Mutex::new(characters)),
            starships: Arc::new(Mutex::new(starships)),
            planets: Arc::new(Mutex::new(planets)),
        }
    }

    pub async fn get_saga_hero(&self) -> APICharacter {
        self.characters.lock().await[self.luke_idx].clone()
    }

    pub async fn get_r2d2(&self) -> APICharacter {
        self.characters.lock().await[self.r2d2_idx].clone()
    }

    pub async fn get_hero(&self, episode: Episode) -> APICharacter {
        if episode == Episode::Empire {
            self.get_saga_hero().await
        } else {
            self.get_r2d2().await
        }
    }

    pub async fn get_human(&self, id: String) -> Option<APICharacter> {
        self.characters
            .lock()
            .await
            .iter()
            .find(|(_, c)| c.id == id)
            .map(|(_, c)| c)
            .filter(|c| c.is_human)
            .cloned()
    }

    pub async fn get_humans(&self) -> Vec<APICharacter> {
        self.characters
            .lock()
            .await
            .iter()
            .filter(|(_, c)| c.is_human)
            .map(|(_, c)| c)
            .cloned()
            .collect()
    }

    pub async fn get_droid(&self, id: String) -> Option<APICharacter> {
        self.characters
            .lock()
            .await
            .iter()
            .find(|(_, c)| c.id == id)
            .map(|(_, c)| c)
            .filter(|c| !c.is_human)
            .cloned()
    }

    pub async fn get_droids(&self) -> Vec<APICharacter> {
        self.characters
            .lock()
            .await
            .iter()
            .filter(|(_, c)| !c.is_human)
            .map(|(_, c)| c)
            .cloned()
            .collect()
    }

    pub async fn get_character(&self, idx: usize) -> Option<APICharacter> {
        self.characters.lock().await.get(idx).cloned()
    }

    pub async fn get_starship(&self, id: String) -> Option<APIStarShip> {
        self.starships
            .lock()
            .await
            .iter()
            .find(|(_, c)| c.id == id)
            .map(|(_, c)| c)
            .cloned()
    }
    pub async fn get_starship_by_idx(&self, s_idx: usize) -> Option<APIStarShip> {
        self.starships.lock().await.get(s_idx).cloned()
    }

    pub async fn get_planet_by_idx(&self, c_idx: usize) -> Option<APIPlanet> {
        self.planets.lock().await.get(c_idx).cloned()
    }
}
