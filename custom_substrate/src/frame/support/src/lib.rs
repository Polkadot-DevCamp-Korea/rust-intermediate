#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod pallet_prelude {

    pub use std::marker::PhantomData;
    pub use std::{
        collections::HashMap, 
        hash::Hash,
        default::Default,
    };
    pub use num::{Zero, CheckedAdd, CheckedSub};
}