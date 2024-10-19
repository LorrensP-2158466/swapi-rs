use async_graphql::{dataloader::*, FieldError};
use futures::TryStreamExt;
use std::{collections::HashMap, sync::Arc};
pub struct CreditsDataLoader {
    pub pool: sqlx::PgPool,
}

/// Loader for loading just the credits
impl Loader<String> for CreditsDataLoader {
    type Value = i64;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        Ok(
            sqlx::query_as("SELECT user_id, amount FROM credits WHERE user_id = ANY($1)")
                .bind(keys)
                .fetch(&self.pool)
                .map_ok(|result: (String, i64)| (result.0, result.1))
                .try_collect()
                .await?,
        )
    }
}
