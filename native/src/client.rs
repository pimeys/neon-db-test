use crate::user::User;
use quaint::{pooled::Quaint, prelude::*};
use std::io::{Error, ErrorKind};

pub struct InnerClient {
    pool: Quaint,
}

impl InnerClient {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        Ok(Self {
            pool: Quaint::new(url).await?,
        })
    }

    pub async fn select_1(&self) -> anyhow::Result<i64> {
        let conn = self.pool.check_out().await?;
        let res = conn.query_raw("SELECT 1", &[]).await?;

        let row = res.into_single()?;
        let val = row.into_single()?;

        val.as_i64()
            .ok_or(Error::new(ErrorKind::InvalidData, "Not an integer.").into())
    }

    pub async fn users(&self) -> anyhow::Result<Vec<User>> {
        let conn = self.pool.check_out().await?;
        let rows = conn.select(Select::from_table("user")).await?;
        Ok(quaint::serde::from_rows(rows)?)
    }

    pub async fn big_users(&self) -> anyhow::Result<Vec<User>> {
        let conn = self.pool.check_out().await?;
        let rows = conn.select(Select::from_table("user2")).await?;
        Ok(quaint::serde::from_rows(rows)?)
    }
}
