#![allow(unused)]

use bevy::prelude::*;

macro_rules! warned_ok {
    ($input:expr) => {
        if let Ok(input) = $input {
            Some(input)
        } else {
            error!(
                "Didn't get an Ok from something supposed to get one ({}:{}:{}).",
                file!(),
                line!(),
                column!(),
            );

            None
        }
    };
}

pub(crate) use warned_ok;

macro_rules! warned_some {
    ($input:expr) => {
        if let Some(input) = $input {
            Some(input)
        } else {
            error!(
                "Didn't get a Some from something supposed to get one ({}:{}:{}).",
                file!(),
                line!(),
                column!(),
            );

            None
        }
    };
}

pub(crate) use warned_some;

#[derive(Event)]
pub struct TempOnCreation(pub Entity);

// For when you need to use an event, but don't want it to do anything.
// That means you should never observe this event, as it may be unpredictable.
#[derive(Event)]
pub struct DummyEventToTrigger;

// For when you need to use an event, but don't want it to ever activate.
// That means you should never trigger this event.
#[derive(Event)]
pub struct DummyEventToObserve;
