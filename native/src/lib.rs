#[macro_use]
extern crate serde;

pub mod client;
pub mod user;

use client::InnerClient;
use neon::prelude::*;
use std::sync::Arc;
use tokio::runtime::Runtime;

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
                    let ary = neon_serde::to_value(cx, &users).unwrap();

                    vec![ary]
                });

                result
            });

            Ok(cx.undefined().upcast())
        }

        method big(mut cx) {
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
                let users = inner.big_users().await?;

                cb.schedule(move |cx| {
                    let ary = neon_serde::to_value(cx, &users).unwrap();

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
