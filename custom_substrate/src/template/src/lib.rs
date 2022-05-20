
use pallet_hello;
use pallet_balances;

// Aggregate all the pallet event types
pub struct Event {}

impl<T: pallet_hello::Config> From<pallet_hello::Event<T>> for Event {
    fn from(_: pallet_hello::Event<T>) -> Self {
        Self {}
    }
}

impl<T: pallet_balances::Config> From<pallet_balances::Event<T>> for Event {
    fn from(_: pallet_balances::Event<T>) -> Self {
        Self {}
    }
}

impl pallet_hello::Config for Runtime {
    type Event = Event;
}

impl pallet_balances::Config for Runtime {
    type Event = Event;
    type Balances = u32;
    type AccountId = u32;
}

// Our goal is make "construct_runtime" macros like real-Substrate code
/* construct_runtime!(
    pub enum Runtime 
    where
    {
        Hello: pallet_hello
        Balances: pallet_balances
    }
)
*/
pub enum Runtime {}