#[macro_use]
extern crate serde;

use neon::prelude::*;
use quaint::{pooled::Quaint, prelude::*};
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};
use tokio::runtime::Runtime;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: usize,
    name: String,
    age: u16,
}

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
}

pub struct Client {
    inner: Arc<InnerClient>,
    runtime: Arc<Runtime>,
}

declare_types! {
    pub class DatabaseClient for Client {
        init(mut cx) {
            let url: String = cx.argument::<JsString>(0)?.value();
            let mut runtime = Runtime::new().unwrap();

            let inner = runtime.block_on(async {
                InnerClient::new(&url).await
            }).unwrap();

            Ok(Client {
                inner: Arc::new(inner),
                runtime: Arc::new(runtime),
            })
        }

        method select(mut cx) {
            let this = cx.this();
            let func = cx.argument::<JsFunction>(0)?;

            let (inner, runtime) = {
                let guard = cx.lock();
                let client = this.borrow(&guard);

                (client.inner.clone(), client.runtime.clone())
            };

            let cb = EventHandler::new(&cx, this, func);

            runtime.spawn(async move {
                let result: anyhow::Result<()> = Ok(());
                let num = inner.select_1().await?;

                cb.schedule(move |cx| {
                    vec![cx.number(num as f64)]
                });

                result
            });

            Ok(cx.undefined().upcast())
        }

        method users(mut cx) {
            let this = cx.this();
            let func = cx.argument::<JsFunction>(0)?;

            let (inner, runtime) = {
                let guard = cx.lock();
                let client = this.borrow(&guard);

                (client.inner.clone(), client.runtime.clone())
            };

            let cb = EventHandler::new(&cx, this, func);

            runtime.spawn(async move {
                let result: anyhow::Result<()> = Ok(());
                let users = inner.users().await?;

                cb.schedule(move |cx| {
                    let ary = JsArray::new(cx, 2u32);

                    for (i, user) in users.into_iter().enumerate() {
                        let obj = JsObject::new(cx);

                        {
                            let id = cx.number(user.id as f64);
                            obj.set(cx, "id", id).unwrap();
                        }

                        {
                            let name = cx.string(user.name);
                            obj.set(cx, "name", name).unwrap();
                        }

                        {
                            let age = cx.number(user.age as f64);
                            obj.set(cx, "age", age).unwrap();
                        }

                        ary.set(cx, i as u32, obj).unwrap();
                    }

                    vec![ary]
                });

                result
            });

            Ok(cx.undefined().upcast())
        }
    }
}

register_module!(mut cx, {
    cx.export_class::<DatabaseClient>("DatabaseClient")?;
    Ok(())
});
