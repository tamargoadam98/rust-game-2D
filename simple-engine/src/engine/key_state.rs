use std::collections::{HashMap, HashSet};

use super::key::Key;

#[derive(Default)]
pub struct KeyState {
    bindings: HashMap<Key, String>,
    held: HashSet<Key>,
}

impl KeyState {
    pub fn bind(&mut self, key: Key, action: impl Into<String>) {
        self.bindings.insert(key, action.into());
    }

    pub fn key_down(&mut self, key: Key) {
        self.held.insert(key);
    }

    pub fn key_up(&mut self, key: Key) {
        self.held.remove(&key);
    }

    pub fn is_active(&self, action: &str) -> bool {
        self.held
            .iter()
            .any(|k| self.bindings.get(k).is_some_and(|a| a == action))
    }
}
