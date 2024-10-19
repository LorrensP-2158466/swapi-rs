use async_graphql::{Context, Object};
use futures::future::Either;

use crate::starwars::models::Droid;

use super::{
    models::{Character, Episode, Human, StarShip},
    StarWarsAPI,
};

/// The query object for starwars
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // returns hero based on episode, else it just returns the hero of the entire star wars sage, aka luke SKYWALKER
    async fn hero<'ctx>(&self, ctx: &Context<'ctx>, episode: Option<Episode>) -> Character {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        episode
            .map_or_else(
                || Either::Left(async { api.get_saga_hero().await }),
                |ep| Either::Right(async move { api.get_hero(ep).await }),
            )
            .await
            .into()
    }

    async fn human<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Option<Human> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        api.get_human(id).await.map(Into::into)
    }

    async fn droid<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Option<Droid> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        api.get_droid(id).await.map(Into::into)
    }

    async fn starship<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Option<StarShip> {
        let api = ctx.data_unchecked::<StarWarsAPI>();
        api.get_starship(id).await.map(StarShip)
    }

    async fn humans<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Human> {
        ctx.data_unchecked::<StarWarsAPI>()
            .get_humans()
            .await
            .into_iter()
            .map(Into::into)
            .collect()
    }

    async fn droids<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Droid> {
        ctx.data_unchecked::<StarWarsAPI>()
            .get_humans()
            .await
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn transact<'a, 'ctx>(
        &self,
        ctx: &Context<'ctx>,
        from_user_id: String,
        to_user_id: String,
        amount: usize,
    ) -> Result<bool, String> {
        let db = ctx.data_unchecked::<sqlx::PgPool>();
        Ok(true)
    }
}
