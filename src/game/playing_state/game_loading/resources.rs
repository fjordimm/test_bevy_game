use std::{any::TypeId, collections::HashSet};

use bevy::prelude::*;

#[derive(Resource)]
pub struct GameLoadingInhibition {
    data: HashSet<TypeId>,
}

impl GameLoadingInhibition {
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    pub fn add_inhibitor<InhibitorId: 'static>(&mut self) {
        self.data.insert(TypeId::of::<InhibitorId>());
    }

    pub fn remove_inhibitor<Inhibitor: 'static>(&mut self) {
        self.data.remove(&TypeId::of::<Inhibitor>());
    }

    pub fn num_inhibitors(&self) -> usize {
        self.data.len()
    }
}
