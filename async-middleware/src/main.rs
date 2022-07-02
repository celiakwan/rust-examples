use action::{adder_mw, logger_mw, Action};
use middleware::SafeFnWrapper;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

mod action;
mod middleware;

#[tokio::main]
async fn main() {
    {
        let mut handles = Vec::<JoinHandle<Option<Action>>>::new();
        let mw_fun: SafeFnWrapper<Action> = logger_mw();
        handles.push(mw_fun.spawn(Action::Add(1, 2)));
        handles.push(mw_fun.spawn(Action::Add(3, 4)));

        for handle in handles {
            handle.await.unwrap();
        }
    }

    {
        let mut handles = Vec::<JoinHandle<Option<Action>>>::new();
        let data = Arc::new(Mutex::new(vec![]));
        let mw_fun: SafeFnWrapper<Action> = adder_mw(&data);
        handles.push(mw_fun.spawn(Action::Add(1, 2)));
        handles.push(mw_fun.spawn(Action::Add(3, 4)));
        
        for handle in handles {
            println!("{:?}", handle.await.unwrap());
        }

        println!("Vec size: {:?}", data.lock().unwrap().len());
    }
}
