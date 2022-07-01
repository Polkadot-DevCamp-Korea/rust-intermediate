
#[frame_support::pallet]
pub mod pallet {

    pub trait Config<I: 'static = ()>: frame_system::Config {
        type Balance
        type DustRemoval
        type Event
        type ExistentialDeposit: Get<Self::Balance>;
        type AccountStore
        type WeightInfo
        type MaxLocks
        type MaxReserves
        type ReserveIdentifier
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T, I = ()>(PhantomData<T, I>)

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        
        // 시완
        #[pallet::weight]
        pub fn transfer(origin, dest) {}

        // 명하
        #[pallet::weight] 
        pub fn set_balance(origin, who, new_free, new_reserved) {}

        // 현택
        #[pallet::weight]
        pub fn force_transfer(origin, source, dest, value) {}
        
        // 경원
        #[pallet::weight]
        pub fn transfer_keep_alive(origin, dest, value) {}

        // 혜민 
        #[pallet::weight]
        pub fn transfer_all(origin, dest, keep_alive) {}

        // 소윤
        #[pallet::weight]
        pub fn force_unreserve(
            origin: OriginFor<T>, 
            who: <T::LookUp as StaticLookUp>::Source, 
            amount: T::Balance,
        ) -> DispatchResult {
            ensure_root(origin)?; // only sudo can call
            let who = T::LookUp::lookup(who)?;
            let _leftover = <Self as ReservableCurrency<_>>::unreserve(&who, amount)
            
            Ok(())
        } 

        // impl<T: Config> StaticLookup for Pallet<T> {
        //     type Source = MultiAddress<T::AccountId, T::AccountIndex>;
        //     type Target = T::AccountId;
        
        //     fn lookup(a: Self::Source) -> Result<Self::Target, LookupError> {
        //         Self::lookup_address(a).ok_or(LookupError)
        //     }
        
        //     fn unlookup(a: Self::Target) -> Self::Source {
        //         MultiAddress::Id(a)
        //     }
        // }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        Endowed,
        DustLost,
        Transfer,
        BalacneSet,
        Reserved,
        Unreserved,
        ReserveRepatraited,
        Deposit,
        Withdraw,
        Slashed,
    }

    #[pallet::error]
    pub enum Error<T, I = ()> {
        VestingBalance,
        LiquidityRestrictions,
        InsufficientBalance,
        ExistentialDeposit,
        KeepAlive,
        ExistingVestingSchedule,
        DeadAccount,
        TooManyReserved
    }

    #[pallet::storage]
    #[pallet::getter(fn total_issuance)]
    pub type TotalIssuance<T: Config<I>, I: 'static = ()> = StorageValue<>; 
    
    #[pallet::storage]
    pub type Account<T: Config<I>, I: 'static = ()> = StorageMap<>;
 
    #[pallet::storage]
    #[pallet::getter(fn locks)]
    pub type Locks<T: Config<I>, I: 'static = ()> = StorageMap<>;
    
    #[pallet::storage]
    #[pallet:getter(fn reserves)]
    pub type Reserves<T: Config<I>, I: 'static = ()> = StorageMap<>;

    #[pallet::storage]
    pub(super) type StorageVersion<T: Config<I>, I: 'static = ()> = StorageValue<>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
        pub balances
    }

    #[cfg(feature = "std")]
    impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
        fn default() -> Self {
            Self { balances: Default::default()}
        }
    }

    #[pallet::genesis_build]
    impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
        fn build(&self) {}
    }

    #[cfg(feature = "std")]
    impl<T: Config<I>, I: 'static> GenesisConfig<T, I> {
        
        pub fn build_storage(&self) {}

        pub fn assimilate_storage(&self, storage)
    }

    #[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    pub enum Reasons {
        Fee = 0,
        Misc = 1,
        All = 2,
    }

    impl From<WithdrawReasons> for Reasons {
        fn from(r; WithdrawReasons) -> Reasons {}
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    pub struct BalanceLock<Balance> {
        pub id,
        pub amount,
        pub reasons
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    pub struct ReserveData<ReserveIdentifier, Balance> {
        pub id,
        pub amount,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    pub struct AccountData<Balance> {
        pub free: Balance,
        pub reserved: Balance,
        pub misc_frozen: Balance,
        pub fee_frozen: Balance,
    }

    impl<Balance: Saturating + Copy + Ord> AccountData<Balance> {
        fn usable(&self, reasons: Reasons) -> Balance {
            // free - frozen = usable
            self.free.saturating_sub(self.frozen(reasons))
        }
        fn frozen(&self, reasons: Reasons) -> Balance {
            match reasons {
                Reasons::All => self.misc_frozen.max(self.fee_frozen), // Take max(misc_frozen, fee_frozen)
                Reasons::Misc => self.misc_frozen,
                Reasons::Fee => self.fee_frozen
            }
        }
        fn total(&self) -> Balance {
            // free + reserved = total
            self.free.saturating_add(self.reserved)
        }
    }

    #[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    enum Releases {
        v1_0_0,
        v2_0_0,
    }

    impl Default for Releases {
        fn default() -> Self {
            Releases::v1_0_0
        }
    }

    pub struct DustCleaner<T: Config<I>, I: 'static = ()>(
        Option<(T::AccountId, NegativeImbalance<T,I>)>,
    );

    impl<T: Config<I>, I: 'static> Drop for DustCleaner<T, I> {
        fn drop(&mut self) {}
    }

    // internal/external function
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        
        // 현택
        pub fn free_balance(who) -> T::Balance {}

        // 명하
        pub fn usable_balance(who) -> T::Balance {}

        // 소윤 
        pub fn usable_balance_for_fees(who: impl sp_std::borrow::Borrow<T::AccountId>) -> T::Balance {
            // free - fee_frozen = usable_balance_for_fee
            self.account(who.borrow()).usable(Reasons::Fee)
        }

        // 시완
        pub fn reserved_balance(who) -> T::Balance {}

        // 경원
        fn account(who) -> AccountData<T::Balance> {}

        // 소윤 
        fn post_mutation(
            _who: &T::AccountId, // reference 
            new: AccountData<T::Balance>
        ) -> (Option<AccountData<T::Balance>>, Option<NegativeImbalance<T, I>>) {
            // Concept 
            // Post action for newly created account
            // returns tuple (account-data, negative-imbalance e.g slashing/dust)
            
            // type ExistentialDeposit: Get<Self::Balance>;

            let total = new.total(); // Free + Reserved
            if total < T::ExistentialDeposit::get() {
                if total.is_zero() {
                    (None, None)
                } else {
                    (None, Some(NegativeImbalance::new(total)))
                }
            } else {
                (Some(new), None)
            }
        }

        // 혜민
        fn deposit_consequence(_who, amount, account, mint) -> DepositConsequence {}

        // 시완 
        fn withdraw_consequence(who, amount, account) -> WithdrawConsequnce<T::Balance> {}

        // 명하
        pub fn mutate_account<R>(who, f) -> Result<R, DispatchError> {}

        //현택
        fn try_mutate_account<R, E: From<DispatchError>> (who, f) -> Result<R, E> {}dddddddddfffffffffff

        // 소윤
        fn try_mutate_account_with_dust<R, E: From<DispatchError>>(
            who: &T::AccountId, 
            f: impl FnOnce(&mut AccountData<T::Balance>, bool) -> Result<R, E>
        ) -> Result<(R, DustCleaner<T, I>), E> {

            // try_mutate_exists => Some / None
            // maybe_account => AccountData
            // account => AccountData
            let result = T::AccountStore.try_mutate_exists(who, |maybe_account| {
                let is_new = maybe_account.is_none(); // if account stored in AccountStore, False else True
                let mut account = maybe_account.take().unwrap_or_default() // Default = Endowed account(pre-funded account). Most account would have no endowment
                f(&mut account, is_new).map(move |result| {
                    let maybe_endowed = if is_new { Some(account.free) } else None;
                    let maybe_account_maybe_dust = Self::post_mutation(who, account); // If account is not dust, it would return account itself
                    *maybe_account = maybe_account_maybe_dust.0; // Change the value inside maybe_account after 'post_mutation' => None / account
                    (maybe_endowed, maybe_account_maybe_dust.1, result)
                })
            })

            result.map(|maybe_endowed, maybe_dust, result| {
                if let Some(endowed) = maybe_endowed {
                    Self::deposit_event(Event::Endowed {account: who.clone(), free_balance: endowed})
                }
                let dust_cleaner = DustCleaner(maybe_dust.map(|dust| (who.clone(), dust)))
                (result, dust_cleaner)
            })
        }

        // 혜민 
        fn update_locks(who, locks) {}

        // 경원
        fn do_transfer_reserved(slashed, beneficiary, value, best_effort, status) -> Result<T::Balance, DispatchError> {}
    }

    impl<T: Config<I>, I: 'static> fungible::Mutate<T::AccountId> for Pallet<T, I> {
        
        fn mint_into(who, amount) {}

        fn burn_from(who, amount) {}
    }

    impl<T: Config<I>, I: 'static> fungible::Transfer<T::AccountId> for Pallet<T, I> {

        fn transfer(source, dest, amount, keep_alive) {}
    }

    impl<T: Config<I>, I: 'static> fungible::Unbalanced<T::AccountId> for Pallet<T, I> {

        fn set_balance(who, amount) {}

        fn set_total_issuance(amount) {}
    }

    impl<T: Config<I>, I: 'static> fungible::InspectHold<T::AccountId> for Pallet<T, I> {

        fn balance_on_hold(who) {}

        fn can_hold(who, amount) {}
    }

    impl<T: Config<I>, I: 'static> fungible::MutateHold<T::AccountId> for Pallet<T, I> {

        fn hold(who, amount) {}

        fn release(who, amount, best_effor) {}

        fn transfer_held(source ,dest, amount, best_effort, on_hold) {}
    }

    mod imbalances {}

    impl<T: Config<I>, I: 'static> Currency<T::AccountId> for Pallet<T, I> 
    where T::Balacne: MaybeSerialzeDeseralize + Debug,
    {
        fn total_balance(who) {}

        fn can_slash(who, value) {}

        fn total_issuance() {}

        fn minimum_balance() {}

        fn burn(mut amount) {}

        fn issue(mut amount) {}

        fn free_balance(who) {}

        fn ensure_can_withdraw(who, amount, reasons, new_balance) {}

        fn transfer(transactor, dest, value, existence_requirement) {}

        fn slash(who, value) {} 

        fn deposit_into_existing(who, value) {}

        fn deposit_creating(who, value) {}

        fn withdraw(who, value, reasons, liveness) {}

        fn make_free_balance_be(who, value) {}
    }

    impl<T: Config<I>, I: 'static> ReservableCurrency<T::AccountId> for Pallet<T, I> {

        fn can_reserve(who, value) {}

        fn reserved_balance(who) {}

        fn reserve(who, value) {}

        fn unreserve(who ,value) {}

        fn slash_reserved(who, value) {}

        fn repatriate_reserved(slashed, beneficiary, value, status) {}
    }

    impl<T: Config<I>, I: 'static> NamedReservableCurrency<T::AccountId> for Pallet<T, I> {

        fn reserved_balance_named(id, who) {}

        fn reserve_named(id, who, value) {}

        fn unreserve_named(id, who, value) {}

        fn slash_reserved_named(id, who, value) {}

        fn repatriate_reserved_named(id, slashed, beneficiary, value, status) {}
    }

    impl<T: Config<T>, I: 'static> LockableCurrency<T::AccountId> for Pallet<T, I> {

        fn set_lock(id, who, amount, reasons) {}

        fn extend_lock(id, who, amount, reasons) {}

        fn remove_lock(id, who) {}
    }
}
