use async_graphql::{
    dataloader::DataLoader, Context, DataContext, Enum, Interface, Object, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::Executor;

use crate::starwars::data::{APICharacter, APIPlanet, APIStarShip, StarWarsAPI};
use futures::{future::Either, stream, FutureExt, StreamExt};

use super::credits_loader::CreditsDataLoader;
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

pub struct Human {
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

    /// mass of character (i.e. weight) in kg
    pub mass: usize,
}

impl From<APICharacter> for Human {
    fn from(value: APICharacter) -> Self {
        Self {
            id: value.id,
            name: value.name,
            friends: value.friends,
            appears_in: value.appears_in,
            home_planet: value.home_planet,
            star_ship: value.star_ship,
            mass: value.mass,
        }
    }
}

#[Object]
impl Human {
    pub async fn id(&self) -> &str {
        &self.id
    }
    pub async fn name(&self) -> &str {
        &self.name
    }
    pub async fn friends<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Character> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        stream::iter(self.friends.iter())
            .filter_map(|&i| async move { api.get_character(i).await })
            .map(Into::into)
            .collect()
            .await
    }

    pub async fn appears_in(&self) -> Vec<Episode> {
        self.appears_in.clone()
    }

    pub async fn mass(&self) -> usize {
        self.mass
    }

    pub async fn home_planet<'ctx>(&self, ctx: &Context<'ctx>) -> Option<Planet> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        let home_planet = self.home_planet?;
        api.get_planet_by_idx(home_planet).await.map(Into::into)
    }

    pub async fn starship<'ctx>(&self, ctx: &Context<'ctx>) -> Option<StarShip> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        let star_ship = self.star_ship?;
        api.get_starship_by_idx(star_ship).await.map(Into::into)
    }

    pub async fn credits<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<i64>> {
        // we know it exists
        let loader = ctx.data_unchecked::<DataLoader<CreditsDataLoader>>();
        loader.load_one(self.id.clone()).await
    }
}
pub struct Droid {
    /// id of this character
    pub id: String,

    /// name of this character
    pub name: String,

    /// integer id's of character friends
    /// maps in StarWars.characters
    pub friends: Vec<usize>,

    /// all the episodes this character appeared in
    pub appears_in: Vec<Episode>,

    /// primary function of droid
    pub primary_function: Option<String>,

    /// mass of character (i.e. weight) in kg
    pub mass: usize,
}

impl From<APICharacter> for Droid {
    fn from(value: APICharacter) -> Self {
        Self {
            id: value.id,
            name: value.name,
            friends: value.friends,
            appears_in: value.appears_in,
            mass: value.mass,
            primary_function: value.primary_function,
        }
    }
}

#[Object]
impl Droid {
    pub async fn id(&self) -> &str {
        &self.id
    }
    pub async fn name(&self) -> &str {
        &self.name
    }
    pub async fn friends<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Character> {
        // we weten dat StarWarsAPI bestaat duhhhh
        // in het echt moeten we dit doen:
        // let api = ctx.data::<StarWarsAPI>()?; // return error
        let api = ctx.data_unchecked::<StarWarsAPI>();
        stream::iter(self.friends.iter())
            .filter_map(|&i| api.get_character(i))
            .map(Into::into)
            .collect()
            .await
    }

    pub async fn appears_in<'ctx>(&self) -> Vec<Episode> {
        self.appears_in.clone()
    }

    /// The primary function of the droid.
    async fn primary_function(&self) -> Option<&str> {
        self.primary_function.as_deref()
    }
}

/// A Star Wars starship
/// think of Milenium Falcon or X-Wing, ...
pub struct StarShip(pub APIStarShip);

impl From<APIStarShip> for StarShip {
    fn from(value: APIStarShip) -> Self {
        Self(value)
    }
}
#[Object]
impl StarShip {
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

/// a Star Wars planet, think of Alderaan or Coruscant
pub struct Planet(APIPlanet);

impl From<APIPlanet> for Planet {
    fn from(value: APIPlanet) -> Self {
        Self(value)
    }
}

#[Object]
impl Planet {
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
    field(name = "id", ty = "&str"),
    field(name = "name", ty = "&str"),
    field(name = "friends", ty = "Vec<Character>"),
    field(name = "appears_in", ty = "Vec<Episode>")
)]
pub enum Character {
    Human(Human),
    Droid(Droid),
}

impl From<APICharacter> for Character {
    fn from(value: APICharacter) -> Self {
        if value.is_human {
            Self::Human(value.into())
        } else {
            Self::Droid(value.into())
        }
    }
}
