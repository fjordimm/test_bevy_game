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

pub(crate) use warned_ok;
pub(crate) use warned_some;
