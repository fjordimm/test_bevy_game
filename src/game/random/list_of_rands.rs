macro_rules! list_of_rands {
    ($name:ident) => {
        $name! {
            // Add to the list here!
            GeneralRand,
        }
    };
}

pub(crate) use list_of_rands;
