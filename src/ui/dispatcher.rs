use super::{action::Action, store::Store};

pub struct Dispatcher {}

impl Dispatcher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn dispatch(&mut self, store: &mut Store, action: Action) {
        store.update(action);
    }
}
