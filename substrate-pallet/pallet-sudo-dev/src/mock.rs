
use super::*;
use crate as sudo;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64, Contains, GenesisBuild}
};
use frame_system::limits;
use sp_core::H256;
use sp_io;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup}
};

#[frame_support::pallet]
pub mod logger {
    
    use core::marker::{ PhantomData };

    use frame_support::{
                            pallet_prelude::*, 
                            traits::IsType, 
                            dispatch::DispatchResultWithPostInfo,
                        };
    use frame_system::pallet_prelude::*;
    
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    // enum Call {}
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        /// Only root calls this method.
        #[pallet::weight(*weight)]
        pub fn only_sudo_can_call(
            origin: OriginFor<T>,
            value: i32,
            weight: Weight,
        ) -> DispatchResultWithPostInfo {
            
            ensure_root(origin)?;
            I32Vec::<T>::append(value);
            Self::deposit_event(
                                    Event::AppendI32 { value, weight }
                                );

            Ok(().into())
        }

        #[pallet::weight(*weight)]
        pub fn only_signed_can_call(
            origin: OriginFor<T>,
            value: i32,
            weight: Weight,
        ) -> DispatchResultWithPostInfo {

            let signed = ensure_signed(origin)?;
            I32Vec::<T>::append(value);
            AccountVec::<T>::append(signed.clone());
            Self::deposit_event(
                                    Event::AppendI32AndAccount { signed, value, weight }
                                );

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        
        AppendI32 { value: i32, weight: Weight },
        AppendI32AndAccount { signed: T::AccountId, value: i32, weight: Weight },
    }

    #[pallet::storage]
    #[pallet::getter(fn account_log)]
    pub(super) type AccountVec<T: Config> = StorageValue<
                                                            _,
                                                            Vec<T::AccountId>,
                                                            ValueQuery,                
                                                        >;

    #[pallet::storage]
    #[pallet::getter(fn i32_log)]
    pub(super) type I32Vec<T> = StorageValue<
                                                _,
                                                Vec<i32>,
                                                ValueQuery,
                                            >;

}


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test 
    where 
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Sudo: sudo::{Pallet, Call, Config<T>, Storage, Event<T>},
        Logger: logger::{Pallet, Call, Storage, Event<T>}, 
    }
);

pub struct BlockEverything;
impl Contains<Call> for BlockEverything {
    fn contains(_: &Call) -> bool {
        false
    }
}

impl frame_system::Config for Test {

    type BaseCallFilter = BlockEverything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl logger::Config for Test {

    type Event = Event;
}

impl Config for Test {
    type Event = Event;
    type Call = Call;
}

pub type SudoCall = sudo::Call<Test>;
pub type LoggerCall = logger::Call<Test>;

pub fn test_build(root_key: u64) -> sp_io::TestExternalities {

    let mut test_storage = frame_system
                                        ::GenesisConfig
                                        ::default().build_storage
                                        ::<Test>().unwrap();

    sudo
        ::GenesisConfig::<Test> { key: Some(root_key) }.assimilate_storage(&mut test_storage).unwrap();
    
    test_storage.into()
}
