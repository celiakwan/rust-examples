use action::{adder_mw, logger_mw, Action};
use middleware::SafeFnWrapper;

mod action;
mod middleware;

#[tokio::main]
async fn main() {
    {
        let mw_fun: SafeFnWrapper<Action> = logger_mw();
        mw_fun.spawn(Action::Add(1, 2)).await.unwrap();
        mw_fun.spawn(Action::Add(1, 2)).await.unwrap();
    }

    {
        let mw_fun: SafeFnWrapper<Action> = adder_mw();
        println!("{:?}", mw_fun.spawn(Action::Add(1, 2)).await.unwrap());
        println!("{:?}", mw_fun.spawn(Action::Add(1, 2)).await.unwrap());
    }
}
