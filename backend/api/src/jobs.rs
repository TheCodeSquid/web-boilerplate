use std::{future::Future, sync::Arc};

use service::prelude::*;
use tokio::time::{self, Duration};

use crate::state::Api;

pub fn spawn_jobs(api: Arc<Api>) {
    let jobs = Jobs(api);

    jobs.run(session_pruning, Duration::from_secs(1));
}

// Jobs //

async fn session_pruning(api: Arc<Api>) -> eyre::Result<()> {
    session::prune(*api.config.session_lifetime, &api.db).await?;
    Ok(())
}

// Runner //

struct Jobs(Arc<Api>);
impl Jobs {
    fn run<F, Fut>(&self, f: F, interval: Duration)
    where
        F: FnMut(Arc<Api>) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = eyre::Result<()>> + Send + 'static,
    {
        let api = self.0.clone();
        tokio::spawn(async move {
            loop {
                time::sleep(interval).await;

                let mut f = f.clone();
                let api = api.clone();
                let res = tokio::spawn(async move {
                    if let Err(err) = f(api).await {
                        error!("job returned error: {}", err);
                    }
                })
                .await;
                if let Err(err) = res {
                    error!("job task panicked: {}", err);
                }
            }
        });
    }
}
