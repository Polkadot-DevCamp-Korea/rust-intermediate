
pub use pallet::*;

pub mod pallet {

    use frame_support::pallet_prelude::*;
    
    pub trait Config: Sized {
        type Event: From<Event<Self>>;
    }

    pub enum Event<T: Config> {
        Dummy(PhantomData<T>)
    }
}