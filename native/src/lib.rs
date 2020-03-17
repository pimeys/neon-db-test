use neon::prelude::*;
use quaint::{pooled::Quaint, prelude::*};
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct Client {
    pool: Arc<Quaint>,
    runtime: Arc<Runtime>,
}

declare_types! {
    pub class DatabaseClient for Client {
        init(mut cx) {
            let url: String = cx.argument::<JsString>(0).unwrap().value();
            let mut runtime = Runtime::new().unwrap();

            let pool = runtime.block_on(async {
                Quaint::new(&url).await
            }).unwrap();

            Ok(Client {
                pool: Arc::new(pool),
                runtime: Arc::new(runtime),
            })
        }

        method select(mut cx) {
            let this = cx.this();
            let func = cx.argument::<JsFunction>(0).unwrap();

            let (pool, runtime) = {
                let guard = cx.lock();
                let client = this.borrow(&guard);

                (client.pool.clone(), client.runtime.clone())
            };

            let cb = EventHandler::new(&cx, this, func);

            runtime.spawn(async move {
                let result: anyhow::Result<()> = Ok(());
                let conn = pool.check_out().await.unwrap();
                let res = conn.query_raw("SELECT 1", &[]).await.unwrap();

                cb.schedule(move |cx| {
                    let result = res.into_single().unwrap()[0].as_f64().unwrap();
                    vec![cx.number(result)]
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
