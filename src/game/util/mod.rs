#![allow(unused)]

use bevy::prelude::*;

// "Alerted Make-Sure-It-Is Ok"
// Takes a Result and...
//  - If it's an Ok, then just return the inner part in a Some.
//  - If it's an Err, then return None but report an error!().
macro_rules! alrmo {
    ($input:expr) => {
        if let Ok(input) = $input {
            Some(input)
        } else {
            error!(
                "Didn't get an Ok from something that was supposed to get one ({}:{}:{}). IMPORTANT: This probably has broken the functionality of whatever function it was in.",
                file!(),
                line!(),
                column!(),
            );

            None
        }
    };
}

pub(crate) use alrmo;

// "Alerted Make-Sure-It-Is Some"
// Takes an Option and...
//  - If it's a Some, then just return the inner part in a Some.
//  - If it's a None, then return None but report an error!().
macro_rules! alrms {
    ($input:expr) => {
        if let Some(input) = $input {
            Some(input)
        } else {
            error!(
                "Didn't get a Some from something that was supposed to get one ({}:{}:{}). IMPORTANT: This probably has broken the functionality of whatever function it was in.",
                file!(),
                line!(),
                column!(),
            );

            None
        }
    };
}

pub(crate) use alrms;

// "Alerted Unwrap-Or-Return Ok"
// Similar to .unwrap, but instead of panicking,
//  it will return from the function it is in as well as reporting an error!().
macro_rules! alrro {
    ($input:expr) => {
        if let Ok(input) = $input {
            input
        } else {
            error!(
                "Didn't get an Ok from something that was supposed to get one ({}:{}:{}). IMPORTANT: This has returned prematurely from whatever function it was in.",
                file!(),
                line!(),
                column!(),
            );

            return;
        }
    };
}

pub(crate) use alrro;

// "Alerted Unwrap-Or-Return Some"
// Similar to .unwrap, but instead of panicking,
//  it will return from the function it is in as well as reporting an error!().
macro_rules! alrrs {
    ($input:expr) => {
        if let Some(input) = $input {
            input
        } else {
            error!(
                "Didn't get a Some from something that was supposed to get one ({}:{}:{}). IMPORTANT: This has returned prematurely from whatever function it was in.",
                file!(),
                line!(),
                column!(),
            );

            return;
        }
    };
}

pub(crate) use alrrs;

// For when you need to use an event, but don't want it to do anything.
// That means you should never observe this event, as it may be unpredictable.
#[derive(Event)]
pub struct DummyEventToTrigger;

// For when you need to use an event, but don't want it to ever activate.
// That means you should never trigger this event.
// TODO: add an observer for this event that reports an error!().
#[derive(Event)]
pub struct DummyEventToObserve;

pub fn get_entity_components(world: &World, entity: Entity) -> String {
    let mut ret = String::from("-----Entity-----");

    if let Some(components) = alrmo!(world.inspect_entity(entity)) {
        components.into_iter().for_each(|component_info| {
            ret.push_str("\n    ");
            ret.push_str(&component_info.name().as_string());

            let a = 'e';
        });
    }

    ret
}

pub fn col_to_array(col: Color) -> [f32; 3] {
    let c = col.to_linear();
    [c.red, c.green, c.blue]
}

pub fn col_to_array4(col: Color) -> [f32; 4] {
    let c = col.to_linear();
    [c.red, c.green, c.blue, c.alpha]
}

pub fn seed_from_u64(inp: u64) -> [u8; 8] {
    inp.to_le_bytes()
}

pub mod mathf32 {
    pub fn lerp_remap(
        x: f32,
        lower_bound_from: f32,
        upper_bound_from: f32,
        lower_bound_to: f32,
        upper_bound_to: f32,
    ) -> f32 {
        ((x - lower_bound_from) / upper_bound_from) * (upper_bound_to - lower_bound_to)
            + lower_bound_to
    }
}

pub mod mathf64 {
    pub fn lerp_remap(
        x: f64,
        lower_bound_from: f64,
        upper_bound_from: f64,
        lower_bound_to: f64,
        upper_bound_to: f64,
    ) -> f64 {
        ((x - lower_bound_from) / upper_bound_from) * (upper_bound_to - lower_bound_to)
            + lower_bound_to
    }

    pub fn sigmoid(x: f64) -> f64 {
        1. / (1. + 2f64.powf(-x))
    }
}
