use rand::Rng;
use std::sync::Arc;
use tokio::{sync::RwLock, task::JoinHandle, time};

pub type SafeFn<A> = Arc<RwLock<dyn FnMut(A) -> Option<A> + Sync + Send>>;

pub struct SafeFnWrapper<A> {
    fn_mut: SafeFn<A>,
}

impl<A: Sync + Send + 'static> SafeFnWrapper<A> {
    pub fn new(fn_mut: impl FnMut(A) -> Option<A> + Sync + Send + 'static) -> SafeFnWrapper<A> {
        Self {
            fn_mut: Arc::new(RwLock::new(fn_mut)),
        }
    }

    pub fn spawn(&self, action: A) -> JoinHandle<Option<A>> {
        let arc_lock_fn_mut = self.fn_mut.clone();
        tokio::spawn(async move {
            println!("Spawning");
            let delay_ms = rand::thread_rng().gen_range(1_000..3_000) as u64;
            time::sleep(time::Duration::from_millis(delay_ms)).await;
            let mut fn_mut = arc_lock_fn_mut.write().await;
            let result = fn_mut(action);
            println!("Completed");
            result
        })
    }
}
