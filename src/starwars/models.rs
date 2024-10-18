use async_graphql::{Context, DataContext, Enum, Interface, Object};
use serde::{Deserialize, Serialize};

use crate::starwars::data::{APICharacter, APIPlanet, APIStarShip, StarWarsAPI};

/// One of the films in the Star Wars Trilogy
#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Episode {
    /// Released in 1977.
    NewHope,

    /// Released in 1980.
    Empire,

    /// Released in 1983.
    Jedi,
}

pub struct Human<'a>(&'a APICharacter);

#[Object]
impl<'a> Human<'a> {
    pub async fn id(&self) -> String {
        self.0.id.clone()
    }
    pub async fn name(&self) -> String {
        self.0.name.clone()
    }
    pub async fn friends<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Character<'ctx>> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        self.0
            .friends
            .iter()
            .filter_map(|&i| api.get_character(i))
            .map(Into::into)
            .collect()
    }

    pub async fn appears_in(&self) -> Vec<Episode> {
        self.0.appears_in.clone()
    }

    pub async fn mass(&self) -> usize {
        self.0.mass
    }

    pub async fn home_planet<'ctx>(&self, ctx: &Context<'ctx>) -> Option<Planet<'ctx>> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        self.0
            .home_planet
            .map(|id| api.get_planet_by_idx(id))
            .and_then(|p| p.map(Planet))
    }

    pub async fn starship<'ctx>(&self, ctx: &Context<'ctx>) -> Option<StarShip<'ctx>> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        self.0
            .star_ship
            .map(|id| api.get_starship_by_idx(id))
            .and_then(|p| p.map(StarShip))
    }
}
pub struct Droid<'a>(&'a APICharacter);

#[Object]
impl<'a> Droid<'a> {
    pub async fn id(&self) -> String {
        self.0.id.clone()
    }
    pub async fn name(&self) -> String {
        self.0.name.clone()
    }
    pub async fn friends<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Character<'ctx>> {
        // we weten dat StarWarsAPI bestaat duhhhh
        // in het echt moeten we dit doen:
        // let api = ctx.data::<StarWarsAPI>()?; // return error
        let api = ctx.data_unchecked::<StarWarsAPI>();
        self.0
            .friends
            .iter()
            .filter_map(|&i| api.get_character(i))
            .map(Into::into)
            .collect()
    }

    pub async fn appears_in<'ctx>(&self) -> Vec<Episode> {
        self.0.appears_in.clone()
    }

    /// The primary function of the droid.
    async fn primary_function(&self) -> Option<String> {
        self.0.primary_function.clone()
    }
}

/// A Star Wars starship
/// think of Milenium Falcon or X-Wing, ...
pub struct StarShip<'a>(&'a APIStarShip);

#[Object]
impl<'a> StarShip<'a> {
    /// the Id of the Starship
    async fn id(&self) -> String {
        self.0.id.clone()
    }

    /// name of StarShip
    async fn name(&self) -> String {
        self.0.name.clone()
    }

    async fn length(&self) -> f64 {
        self.0.length
    }
}

pub struct Planet<'a>(&'a APIPlanet);

#[Object]
impl<'a> Planet<'a> {
    async fn id(&self) -> String {
        self.0.id.clone()
    }
    async fn name(&self) -> String {
        self.0.name.clone()
    }
}

// wou gwn weten hoe het werkt met interfaces
#[derive(Interface)]
#[allow(clippy::duplicated_attributes)]
#[graphql(
    field(name = "id", ty = "String"),
    field(name = "name", ty = "String"),
    field(name = "friends", ty = "Vec<Character<'ctx>>"),
    field(name = "appears_in", ty = "Vec<Episode>")
)]
enum Character<'a> {
    Human(Human<'a>),
    Droid(Droid<'a>),
}

impl<'a> From<&'a APICharacter> for Character<'a> {
    fn from(value: &'a APICharacter) -> Self {
        if value.is_human {
            Human(value).into()
        } else {
            Droid(value).into()
        }
    }
}

/// The query object for starwars
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // returns hero based on episode, else it just returns the hero of the entire star wars sage, aka luke SKYWALKER
    async fn hero<'ctx>(&self, ctx: &Context<'ctx>, episode: Option<Episode>) -> Character<'ctx> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        episode.map_or_else(
            || Human(api.get_saga_hero()).into(),
            |ep| api.get_hero(ep).into(),
        )
    }

    async fn human<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Option<Human<'ctx>> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        api.get_human(id).map(Human)
    }

    async fn droid<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Option<Droid<'ctx>> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        api.get_droid(id).map(Droid)
    }

    async fn starship<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Option<StarShip<'ctx>> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        api.get_starship(id).map(StarShip)
    }

    async fn humans<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Human<'ctx>> {
        ctx.data_unchecked::<StarWarsAPI>()
            .get_humans()
            .iter()
            .map(|&h| Human(h))
            .collect()
    }

    async fn droids<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Droid<'ctx>> {
        ctx.data_unchecked::<StarWarsAPI>()
            .get_humans()
            .iter()
            .map(|&h| Droid(h))
            .collect()
    }
}
