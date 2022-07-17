#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{traits::StaticLookup, DispatchResult};
use sp_std::prelude::*;

use frame_support::{traits::UnfilteredDispatchable, weights::GetDispatchInfo};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    #[pallet::config]
    pub trait Config: frame_system::Config {
        
        type Event: From<Event<Self>> + IsType<Self as frame_system::Config>::Event;
        type Call: Parameter + UnfilteredDispatchable<Origin = Self::Origin> + GetDispatchInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        // 소윤
        pub fn sudo() -> DispatchResultWithPostInfo {}

        // 경원
        pub fn sudo_unchecked_weight(
            origin: OriginFor<T>,
            call: Box<<T as Config>::Call>,
            //specifying weight
            _weight: Weight,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
			ensure!(Self::key().map_or(false, |k| sender == k), Error::<T>::RequireSudo);

            let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
			Self::deposit_event(Event::Sudid { sudo_result: res.map(|_| ()).map_err(|e| e.error) });
			Ok(Pays::No.into())
        }

        // 명하
        #[pallet::weight(0)] // limits storage resource
        pub set_key(
            origin: OriginFor<T>,
            new: <T::LookUp as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {

            // check origin
            let sender = ensure_signed(origin)?;

            // get current key, compare to sender
            ensure!(Self::key().map_or(false, |k| sender == k), Error::<T>::RequireSudo);
            
            // get new key
            let new = T::Lookup::lookup(new)?;

            // emit change event
            Self::deposit_event(Event::KeyChanged {old_sudoer: Key::<T>::get()});
            
            // update key
            Key::<T>::put(&new);

            // return ok without fee
            Ok(Pays::No.into());
        }

        // 현택
        pub fn sudo_as() -> DispatchResultWithPostInfo {}
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {

        Sudid { sudo_result: DispatchResult},
        KeyChanged { old_sudoer: Option<T::AccountId>}
        SudoAsDone { sudo_result: DispatchResult},
    }

    #[pallet::error]
    pub enum Error<T> {

        RequireSudo,
    }

    #[pallet::storage]
    #[pallet::getter(fn key)]
    pub(super) type Key<T: Config> = StorageValue<
                                                    _, 
                                                    T::AccountId, 
                                                    OptionQuery
                                    >

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        
        pub key: Option<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { key: None }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        
        fn build(&self) {
            if let Some(ref key) = self.key {
                Key::<T>::put(key);
            }
        }
    }
}