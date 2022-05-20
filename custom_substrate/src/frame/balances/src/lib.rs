
pub use pallet::*;

pub mod pallet {

    use frame_support::pallet_prelude::*;
    
    pub trait Config: Sized {
        type Event: From<Event<Self>>;
        type AccountId: Eq + Hash;
        type Balances: Eq + Hash + Default + Zero + Copy + CheckedSub + CheckedAdd;
    }

    pub enum Event<T: Config> {
        Dummy(PhantomData<T>)
    }

    pub struct Pallet<T: Config> {
        pub balance: HashMap<T::AccountId, T::Balances>
    }

    impl<T: Config> Pallet<T> {
        
        pub fn new() -> Self {
            Self {
                balance: HashMap::new()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::Runtime;

    #[test]
    fn balance_pallet_new_works() {
        let balance = pallet::Pallet<Runtime>;
        assert_eq!(balance::new(), HashMap::new());
    }
}