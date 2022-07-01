use crate::middleware::SafeFnWrapper;

pub fn logger_mw() -> SafeFnWrapper<Action> {
    let logger_lambda = |action: Action| {
        println!("logging: {:?}", action);
        None
    };
    SafeFnWrapper::new(logger_lambda)
}

pub fn adder_mw() -> SafeFnWrapper<Action> {
    let adder_lambda = move |action: Action| match action {
        Action::Add(a, b) => Some(Action::Result(a + b)),
        _ => None,
    };
    SafeFnWrapper::new(adder_lambda)
}

#[derive(Clone, Debug)]
pub enum Action {
    Add(i32, i32),
    Result(i32),
}
