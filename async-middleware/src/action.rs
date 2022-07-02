use crate::middleware::SafeFnWrapper;
use std::sync::{Arc, Mutex};

pub fn logger_mw() -> SafeFnWrapper<Action> {
    let logger_lambda = |action: Action| {
        println!("Logging: {:?}", action);
        None
    };
    SafeFnWrapper::new(logger_lambda)
}

pub fn adder_mw(data: &Arc<Mutex<Vec<i32>>>) -> SafeFnWrapper<Action> {
    let data = data.clone();
    let adder_lambda = move |action: Action| match action {
        Action::Add(a, b) => {
            let sum = a + b;
            let mut data = data.lock().unwrap();
            data.push(sum);
            Some(Action::Result(sum))
        },
        _ => None,
    };
    SafeFnWrapper::new(adder_lambda)
}

#[derive(Clone, Debug)]
pub enum Action {
    Add(i32, i32),
    Result(i32),
}
