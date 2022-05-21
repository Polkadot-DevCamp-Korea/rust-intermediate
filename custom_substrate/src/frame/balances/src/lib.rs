
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

    #[derive(PartialEq, Debug)]
    pub struct Pallet<T: Config> {
        pub balance: HashMap<T::AccountId, T::Balances>
    }

    impl<T: Config> Pallet<T> {
        
        pub fn new() -> Self {
            Self {
                balance: HashMap::new()
            }
        }

        pub fn set_balances(&mut self, account: T::AccountId, balance: T::Balances) {
            self.balance.insert(account, balance);
        }

        pub fn get_balances(&self, account: T::AccountId) -> T::Balances {
            *self.balance.get(&account).unwrap_or(&T::Balances::zero())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::pallet_prelude::*;

    #[derive(PartialEq, Debug)]
    struct Test {}
    struct Event {}

    impl<T: pallet::Config> From<pallet::Event<T>> for Event {
        fn from(_: pallet::Event<T>) -> Self {
            Self {}
        }
    }
    
    impl pallet::Config for Test {
        type Event = Event;
        type AccountId = i32;
        type Balances = i32;
    }

    #[test]
    fn balance_pallet_new_works() {
        let balance = pallet::Pallet::<Test>::new();
        assert_eq!(balance.balance, HashMap::new());
    }

    #[test]
    fn set_balance_should_work() {
        let mut pallet_balance = pallet::Pallet::<Test>::new();
        let user1 = 1;
        let user2 = 2;
        assert_eq!(pallet_balance.balance, HashMap::new());
        pallet_balance.set_balances(user1, 100);
        assert_eq!(pallet_balance.get_balances(user1), 100);
        assert_eq!(pallet_balance.get_balances(user2), 0);
    }
}