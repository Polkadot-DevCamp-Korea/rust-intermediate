
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
        #[pallet::weight(
            T::WeightInfo::set_balance_creating()
            .max(T::WeightInfo::set_balance_killing())
        )] 
        pub fn set_balance(
            origin: OriginFor<T>, 
            who: <T::LookUp as StaticLookUp>::Source, 
            #[pallet::compact] new_free: T::Balance, // For memory efficiency,use #[pallet::compact]
            #[pallet::compact] new_reserved: T::Balance, // For memory efficiency, use #[pallet::compact]
        ) -> DispatchResultWithPostInfo {
                // check the origin === root
                ensure_root(origin)?;
                let who = T::LookUp::lookup(who)?;
                let existential_deposit = T::ExistentialDeposit::get(); // The minimum balance required to create or keep an account open.

                // get current target and his balance
                let wipeout: bool = new_free + new_reserved < existential_deposit;
                let new_free = if wipeout {Zero::zero()} else {new_free};
                let new_reseved = if wipeout {Zero::zero()} else {new_reserved};

                // calculate new free/reseved balance > existential deposit
                let (old_free, old_reserved) = Self::mutate_account(&who, |account| {
                    let old_free = account.free;
                    let old_reserved = account.reseved;
                    
                    account.free = new_free;
                    account.reserved = new_reserved;

                    (old_free, old_reserved)
                })?;

                // To-Do
                // Change total issuance
                if new_free >  old_free {
                    mem::drop(PositiveImbalance::<T, I>::new(new_free - old_free));
                } else if new_free < old_free {
                    mem::drop(NegativeImbalance::<T, I>::new(old_free - new_free));
                }

                if new_reseved > old_reserved {
                    mem::drop(PositiveImbalance::<T, I>::new(new_reseved - old_reserved));
                } else if new_reseved < old_reserved {
                    mem::drop(NegativeImbalance::<T, I>::new(old_reserved - new_reseved));
                }

                // trigger deposit event
                Self::deposit_event(Event::BalacneSet {who, free:new_free, reseved:new_reserved });
                Ok(().into())
            }

        // 현택
        #[pallet::weight(T::WeightInfo::force_transfer())]
        pub fn force_transfer(
            // prelude of type Origin
            origin: OriginFor<T>,
            // StaticLookup for handle multiple types of account address, convert to accountID
            source: <T::LookUp as StaticLookUp>::Source,
            dest: <T::LookUp as StaticLookUp>::Source,
            // encoding compact values
            #[pallet::compact] value: T::Balance,
        // type Dispatchable + Result function + PostInfomation
        ) -> DispatchResultWithPostInfo {
            
            //only root can call this function
            ensure_root(origin)?;

            let source = T::Lookup::lookup(source)?;
            let dest = T::Lookup::lookup(dest)?;
            // type Currency -> transfer function

            <Self as Currency>::transfer(
                //reference
                &source,
                //reference
                &dest,
                //taking ownership b/c actual value are moving source account to dest?
                value,
                //can kill account <-> KeepAlive
                ExistenceRequirement::AllowDeath,
            )
            //???
            Ok(().into())
        }
        
        // 경원
        #[pallet::weight]
        pub fn transfer_keep_alive(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: T::Balance,
		) -> DispatchResultWithPostInfo {
			// Ensure that the origin `o` represents a signed extrinsic (i.e. transaction)
			// Returns `Ok` with the account that signed the extrinsic or an `Err` otherwise
			let transactor = ensure_signed(origin)?;
			// able to provide any compatible address format
			let dest = T::Lookup::lookup(dest)?;
			//returns Dispatchresult, check if account is still alive
			<Self as Currency<_>>::transfer(&transactor, &dest, value, KeepAlive)?;
			Ok(().into())
		}

        // 혜민...
        // Transfer the entire transferable balance from the caller account

        pub fn transfer_all( 
            origin: OriginFor<T>,
            dest :  <T::Lookup as StaticLookup>::Source,
            keep_alive : bool,
            // true : transfer everything except at least the existential deposit, which will guarantee to keep the sender account alive
            // false : sender account to be killed
        ) -> DispatchResult {

            // Result<(), DispatchError>;
            use fungible::Inspect;
            let transactor = ensure_signed(origin)?;

            let reducible_balance = Self::reducible_balance(&transactor, keep_alive); // 전체 balance 다 불러올거같은 느낌?
            let dest = T::Lookup::lookup(dest)?; // The recipient of the transfer
            let keep_alive = if keep_alive { KeepAlive } else { AllowDeath };
            <Self as Currency<_>>::transfer(&transactor, &dest, reducible_balance, keep_alive)?;

            Ok(())
        }
        // 소윤
        #[pallet::weight]
        pub fn force_unreserve (
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
        //make reference of accountId by Borrow Trait and check amount of free balance
        //just checking, no need to take ownership -> so reference?
        // pub trait Borrow<Borrowed> 
        //    where
        //    Borrowed: ?Sized, 
        //    {
        //    fn borrow(&self) -> &Borrowed;
        //    }
        //why using impl keyword? 
        pub fn free_balance(who: impl sp_std::Borrow:borrow<T::AccountId>) -> T::Balance {
            self.account(who.borrow()).free
        }

        // 명하
        pub fn usable_balance(who: impl sp_std::borrow::Borrow<T::AccountId>) -> T::Balance {
            // usable balance 가져오기
            self.account(whoe.borrow()).usable(Reasons::Misc)

        }

        // 소윤 
        pub fn usable_balance_for_fees(who: impl sp_std::borrow::Borrow<T::AccountId>) -> T::Balance {
            // free - fee_frozen = usable_balance_for_fee
            self.account(who.borrow()).usable(Reasons::Fee)
        }

        // 시완
        pub fn reserved_balance(who) -> T::Balance {}

        // 경원
        fn account(
            who: &T::AccountId
        ) -> AccountData<T::Balance> {
            // get account data, or its default value
            T::AccountStore::get(who)
        }

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

        // 혜민...
	// fn deposit_consequence(_who, amount, account, mint) -> DepositConsequence {}
	    
	fn deposit_consequence(
		
	    _who: &T::AccountId,
	    amount: T::Balance,
	    account: &AccountData<T::Balance>,
	    mint: bool,
	
	) -> DepositConsequence {
	    if amount.is_zero() {
		    return DepositConsequence::Success
	    }
	    if mint && TotalIssuance::<T, I>::get().checked_add(&amount).is_none() {
		    return DepositConsequence::Overflow
	    }
	    let new_total_balance = match account.total().checked_add(&amount) {
		    Some(x) => x,
		    None => return DepositConsequence::Overflow,
	    };
	    if new_total_balance < T::ExistentialDeposit::get() {
		    return DepositConsequence::BelowMinimum
	    }
	    DepositConsequence::Success
	}




        // 시완 
        fn withdraw_consequence(who, amount, account) -> WithdrawConsequnce<T::Balance> {}

        // 명하
        // account의 balance 값을 업데이트. 기존에 있는 계정인지 헤크해야 함
        pub fn mutate_account<R>(who:&T::AccountId, f: impl FnOnce(&mut AccountData<T::Balance> -> R)) -> Result<R, DispatchError> {
            Self::try_mutate_account(who, |a, _| -> Result<R, DispatchError> {
                Ok(f(a))
            })
        }

        // 현택
        fn try_mutate_account<R, E: From<DispatchError>>(
            who: &T::AccountId,
            //pub trait FnOnce<Args> {
            //    type Output;

            //    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
            //    }
            //accept a parameter of function-like type and only need to call it once??
            //closure and capturing reference
            f: impl FnOnce(&mut AccountData<T::Balance>, bool) -> Result<R, E>,
        ) -> Result<R, E> {
            //iteralbe.map(|current_item(each individual item in the iterable)|) function(current_item)<-- applying function to each item of iteralbe>)
            Self::try_mutate_account_with_dust(who, f).map(|(result, dust_cleaner)| {
                drop(dust_cleaner);
                result
            })
        }
    

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
                    let maybe_account_maybe_dust = Self::post_mutation(who, account); // If account is not dust, it would return account-data itself
                    *maybe_account = maybe_account_maybe_dust.0; // (AccountData, Dust). Change the value inside maybe_account after 'post_mutation' => None / account
                    (maybe_endowed, maybe_account_maybe_dust.1, result)
                    
                    // maybe_account_maybe_dust.0 -> Accountdata / None
                    // maybe_account_maybe_dust.1 -> Dust / None
                    // result -> Result<R ,E>. R: Generic / E: Generic
                    // Any result type return from 'f' is acceptable since it is generic
                })
            }) 
            
            // result -> (maybe_endowed, maybe_account_maybe, result)
            result.map(|maybe_endowed, maybe_dust, result| {
                if let Some(endowed) = maybe_endowed { // if account is endowed, emit 'event'
                    Self::deposit_event(Event::Endowed {account: who.clone(), free_balance: endowed})
                }
                let dust_cleaner = DustCleaner(maybe_dust.map(|dust| (who.clone(), dust))) // DustCleaner = (Option<(AccountId, dust)>)
                (result, dust_cleaner)
            })

        }

        // 혜민 
        fn update_locks(who, locks) {}

        // 경원
        fn do_transfer_reserved(
            slashed: &T::AccountId,
            beneficiary: &T::AccountId,
            value: T::Balance,
            best_effort: bool,
            status: Status,
        ) -> Result<T::Balance, DispatchError> {
            //if value is zero, do nothing
            if value.is_zero() {
                return Ok(Zero::zero())
            }
            //if slashed account and beneficiary account are same
            if slashed == beneficiary {
                return match status {
                    //change reserved value to free value
                    Status::Free => Ok(Self::unreserve(slashed, value)),
                    //do nothing
                    Status::Reserved => Ok(value.saturating_sub(Self::reserved_balance(slashed))), 
                }
            }
    
            let ((actual, _maybe_one_dust), _maybe_other_dust) = Self::try_mutate_account_with_dust(
                beneficiary,
                |to_account, is_new| -> Result<(T::Balance, DustCleaner<T, I>), DispatchError> {
                    //check if account to transfer is not dead
                    ensure!(!is_new, Error::<T, I>::DeadAccount);
                    Self::try_mutate_account_with_dust(
                        slashed,
                        |from_account, _| -> Result<T::Balance, DispatchError> {
                            //check if reserved account has enough balance
                            let actual = cmp::min(from_account.reserved, value); 
                            ensure!(best_effort || actual == value, Error::<T, I>::InsufficientBalance); 
                            match status {
                                Status::Free =>
                                    to_account.free = to_account
                                        .free
                                        //safemath
                                        .checked_add(&actual)
                                        .ok_or(ArithmeticError::Overflow)?,
                                Status::Reserved =>
                                    to_account.reserved = to_account
                                        .reserved
                                        //safemath
                                        .checked_add(&actual)
                                        .ok_or(ArithmeticError::Overflow)?,
                            }
                            //after adding value to to_account, subtract value from from_account
                            from_account.reserved -= actual;
                            Ok(actual)
                        },
                    )
                },
            )?;
            //emit event, destination_status is status of destination account 
            Self::deposit_event(Event::ReserveRepatriated {
                from: slashed.clone(),
                to: beneficiary.clone(),
                amount: actual,
                destination_status: status,
            });
            Ok(actual)
        }
    }

    impl<T: Config<I>, I: 'static> fungible::Inspect<T::AccountId> for Pallet<T, I> {

        type Balance = T::Balance;

        fn total_issuance(who: &T::AccountId) -> Self::Balance {
            TotalIssuance<T, I>::get()
        }

        fn minimum_balance() -> Self::Balance {
            ExistentialDeposit<T, I>::get()
        }

        fn balance(who: &T::AccountId) -> Self::Balance {
            Self::account(who).total()
        }

        fn reducible_balance(who: &T::AccountId, keep_alive: bool) -> Self::Balance {}

        fn can_deposit(who: &T::AccountId, amount: Self::Balance, mint: bool) -> DepositConsequence {}

        fn can_withdraw(who: &T::AccountId, amount: Self::Balance) -> WithdrawConsequnce {}
    }

    impl<T: Config<I>, I: 'static> fungible::Mutate<T::AccountId> for Pallet<T, I> {
        
        fn mint_into(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {}

        fn burn_from(who: &T::AccountId, amount: Self::Balance) -> Result<Self::Balance, DispatchError> {}
    }

    impl<T: Config<I>, I: 'static> fungible::Transfer<T::AccountId> for Pallet<T, I> {

        fn transfer(
            source: &T::AccountId, 
            dest: &T::AccountId, 
            amount: T::Balance, 
            keep_alive: bool
        ) -> Result<T::Balance, DispatchError> {}
    }

    impl<T: Config<I>, I: 'static> fungible::Unbalanced<T::AccountId> for Pallet<T, I> {

        fn set_balance(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {}

        fn set_total_issuance(amount: Self::Balance) {}
    }

    impl<T: Config<I>, I: 'static> fungible::InspectHold<T::AccountId> for Pallet<T, I> {

        fn balance_on_hold(who: &T::AccountId) -> T::Balance {}

        fn can_hold(who: &T::AccountId, amount: T::Balance) -> bool {}
    }

    impl<T: Config<I>, I: 'static> fungible::MutateHold<T::AccountId> for Pallet<T, I> {

        fn hold(who: &T::AccountId, amount: T::Balance) -> DispatchResult {}

        fn release(who: &T::AccountId, amount: Self::Balance, best_effor: bool) -> Result<T::Balance, DispatchError> {}

        fn transfer_held(
            source: &T::AccountId,
            dest: &T::AccountId, 
            amount: Self::Balance, 
            best_effort: bool, 
            on_hold: bool
        ) -> Result<Self::Balance, DispatchError> {}
    }

    mod imbalances {}

    impl<T: Config<I>, I: 'static> Currency<T::AccountId> for Pallet<T, I> 
    where T::Balacne: MaybeSerialzeDeseralize + Debug,
    {   
        // Associated Types
        type Balance = T::Balance;
        type PositiveImbalance = PositiveImbalance<T, I>;
        type NegativeImabalance = NegativeImabalance<T, I>;

        fn total_balance(who: &T::AccountId) -> Self::Balance {
            Self::account(who).total()
        }

        fn free_balance(who: &T::AccountId) {
            Self::account(who).free()
        }

        fn can_slash(who, value) -> bool {}

        fn total_issuance() -> Self::Balance {
            <TotalIssuance<T,I>>::get()
        }

        fn minimum_balance() -> Self::Balance {
            <ExistentialDeposit<T, I>>::get()
        }

        // 혜민
        fn burn(mut amount) {}

    // Create new funds into the total issuance, returning a negative imbalance
	// for the amount issued.
	fn issue(mut amount: Self::Balance) -> Self::NegativeImbalance {
		if amount.is_zero() {
			return NegativeImbalance::zero()
		}
		//pub fn mutate<R, F: FnOnce(&mut QueryKind::Query) -> R>(f: F) -> R {
		//	<Self as crate::storage::StorageValue<Value>>::mutate(f)
		//}
		//came from TotalIssuance -> StorageValue -> impl method
		//mutate the value 
		<TotalIssuance<T, I>>::mutate(|issued| {
			//Checked integer addition. Computes self + rhs, returning None if overflow occurred
			//issued = existing issued + amount
			//amount = Balance::max_value - existing issued
			*issued = issued.checked_add(&amount).unwrap_or_else(|| {
				//amount = Balance::max_value - existing issued
				amount = Self::Balance::max_value() - *issued;
				Self::Balance::max_value()
			})
		});
	//NegatvieImbalance 
	//Opaque, move-only struct with private fields that serves as a token denoting 
	//that funds have been destroyed without any equal and opposite accounting.
	/// Create a new negative imbalance from a balance.
	//	pub fn new(amount: T::Balance) -> Self {
	//		NegativeImbalance(amount)
	//	}
		NegativeImbalance::new(amount)
	}

        // 명하
        fn ensure_can_withdraw(
            who: &T::AccountId,
             amount: T::Balance,
              reasons:WithdrawReasons, 
              new_balance: T::Balance,
            ) -> DispatchResult {
                if amount.is_zero() {
                    return Ok(())
                }
                let min_balance = Self::account(who).frozen(reasons.into());
                ensure!(new_balance >= min_balance, Error::<T, I>::LiquidityRestrictions);
                Ok(())
            }

        // 소윤
        fn transfer(
            transactor: &T::AccountId, 
            dest: &T::AccountId, 
            value: Self::Balance, 
            existence_requirement: ExistenceRequirement //enum types {keep alive, allow_death}
        ) -> DispatchResult {
            
            if value.is_zero() || transactor == dest {
                return Ok(())
            }

            Self::try_mutate_account_with_dust(
                dest,
                |to_account, _| -> Result<DustCleaner<T, I>, DispatchError> {
                    Self::try_mutate_account_with_dust(
                        transactor,
                        |from_account, _| -> DispatchResult {
                            
                            from_account.free = from_account  
                                                .free()
                                                .checked_sub(value)
                                                .ok_or(Error::<T,I>::InsufficientBalance)?;

                            to_account.free = to_account   
                                              .free()
                                              .checked_add(value)
                                              .ok_or(Error::<T,I>::Overflow)?;

                            let ed = ExistentialDeposit::get();
                            ensure!(to_account.total >= ed, Error::<T,I>::ExistentialDeposit)
                            
                            // Should be greater than Transfer Fee
                            Self::ensure_can_withdraw(
                                transactor,
                                value,
                                WithdrawReasons::TRANSFER, 
                                from_account.free,
                            ).map_err(|_|, Error::<T,I>::LiquidityRestrictions)

                            let allow_death = existence_requirement == ExistenceRequirement::AllowDeath;
                            let allow_death = allow_death && system::Pallet::<T>::can_dec_provider(transactor);
                            
                            // Account should be dead or greater than ed
                            ensure!(
                                allow_death || from_account.total() >= ed, 
                                Error::<T,I>::KeepAlive
                            );

                            Ok(())
                        },
                    ).map(|_, maybe_dust_cleaner| maybe_dust_cleaner)
                }
            )?;

            Self::deposit_event(Event::Transfer {
			    from: transactor.clone(),
			    to: dest.clone(),
			    amount: value,
		    });

            Ok(())
        }
        
        // 소윤
        fn slash(
            who: &T::AccountId, 
            value: Self::Balance
        ) -> (Self::NegativeImabalance, Self::Balance) {

            if value.is_zero() {
                return (NegativeImablance::zero(), Zero::zero())
            }

            if Self::total_balance(who).is_zero() {
                return (NegativeImabalance::zero(), value)
            }

            for attempt in 0..2 {
                match Self::try_mutate_account(
                    who,
                    // NegativeImbalance: Amount of slashed balance, Balance: Amount of not-slashed balance
                    |account, _| -> Result<(Self::NegativeImbalance, Self::Balance), DispatchError> {
                        let best_value = match attempt {
                            // If it is first try, slash the full value
                            0 => value
                            // Else min(value, account.free + account.reserved - ed)
                            _ => value
                                 .min(account.free + account.reserved)
                                 .saturating_sub(T::ExistentialDeposit::get())
                        }
                    };

                    let free_slash = cmp::min(account.free, best_value);
                    account.free -= free_slash;
                    // if account.free < best_value, it would remain some balance
                    // else, remaining_slash = Zero::zero()
                    let remaining_slash = best_value - free_slash; 
                    if !remaning_slash.is_zero() {

                        // If there is remaining slash, take from reserved balance
                        let reserved_slash = cmp::min(account.reserved, remaining_slash);
                        account.reserved -= reserved_slash;
                        Ok((
                            NegativeImabalance::new(free_slash + reserved_slash), 
                            value - free_slash - reserved_slash
                        ))
                    } else {

                        Ok((NegativeImabalance::new(free_slash), value - free_slash))
                    }
                ) {
                    Ok((imbalance, not_slashed)) => {
                        // Emit slash event
                        Self::deposit_event(Event::Slashed {
                            who: who.clone(),
                            amount: value.saturating_sub(not_slashed)
                        });
                        return (imbalance, not_slashed)
                    }

                    Err(_) => (), // increase attempt
                }

                (Self::NegativeImablance::zero(), value)
            }            
        } 

        // 경원
        fn deposit_into_existing(
            who: &T::AccountId,
            value: Self::Balance,
        ) -> Result<Self::PositiveImbalance, DispatchError> {
            //if value is zero, do nothing
            if value.is_zero() {
                return Ok(PositiveImbalance::zero());
            }
        
            Self::try_mutate_account(
                who,
                |account, is_new| -> Result<Self::PositiveImbalance, DispatchError> {
                    //check if account is not new
                    ensure!(!is_new, Error::<T, I>::DeadAccount);
                    //add value to free balance
                    account.free = account.free.checked_add(&value).ok_or(ArithmeticError::Overflow)?;
                    Self::deposit_event(Event::Deposit { who: who.clone(), amount: value});
                    Ok(PositiveImbalance::new(amount: value))
                },
            )
        }

        // 현택
        /// Deposit some `value` into the free balance of `who`, possibly creating a new account.
        ///
        /// This function is a no-op if:
        /// - the `value` to be deposited is zero; or
        /// - the `value` to be deposited is less than the required ED and the account does not yet
        ///   exist; or
        /// - the deposit would necessitate the account to exist and there are no provider references;
        ///   or
        /// - `value` is so large it would cause the balance of `who` to overflow.
        fn deposit_creating(who: &T::AccountId, value: Self::Balance) -> Self::PositiveImbalance {
            /// - the `value` to be deposited is zero;
            if value.is_zero() {
                return Self::PositiveImbalance::zero()
            }

            Self::try_mutate_account(
                who,
                |account, is_new| -> Result<Self::PositiveImbalance, DispatchError> {
                    let ed = T::ExistentialDeposit::get();
                    /// - the `value` to be deposited is less than the required ED
                    /// - the deposit would necessitate the account to exist
                    ensure!(value >= ed || !is_new, Error::<T, I>::ExistentialDeposit);

                    // defensive only: overflow should never happen,
                    // b/c checked_add i think
                    // however in case it does -> ??, 
                    // then this operation is a no-op.
                    // maybe value is so large cause balance overflow?
                    // depositing value in account.free
                    account.free = match account.free.checked_add(&value) {
                        Some(x) => x,
                        //there are no provider references
                        None => return Ok(Self::PositiveImbalance::zero()),
                    };

                    Self::deposit_event(Event::Deposit { who: who.clone(), amount: value });
                    Ok(PositiveImbalance::new(value))
                },
            )
            .unwrap_or_else(|_| Self::PositiveImbalance::zero())
        }

        // 명하
        fn withdraw(who: &T::AccountId, 
            value:Self::Balacne, 
            reasons: WithdrawReasons, 
            liveness:ExistenceRequirement,
        ) -> result::Result<Self::NegativeImabalance, DispatchError> {
            if value.is_zero() {
                return Ok(NegativeImabalance::zero())
            }

            Self::try_mutate_account(who, 
                |account, _| -> Result<Self::NegativeImabalance, DispatchError> {
                    let new_free_account = account.free.checked_sub(&value).ok_or(Error::<T, I>::InsufficientBalance)?;

                    lef ed = T::ExistentialDeposit::get();
                    let would_be_dead = new_free_account + account.reserved < ed;
                    let would_kill = would_be_dead && account.free + account.reserved >= ed;
                    ensure!(liveness == AllowDeath || !would_kill, Error::<T, I>::KeepAlive);

                    Self::ensure_can_withdraw(who, value, reasons, new_free_account)?;

                    account.free = new_free_account;

                    Self::deposit_event(Event::Withdraw {who: who.clone(), amount: value});
                    Ok(NegativeImabalance::new(value))
                },
            )
        }

        // 경원
        fn make_free_balance_be(
            who = &T::AccountId, 
            value = Self::balance,
        ) -> SignedImbalance<Self::Balance, Self::PositiveImbalance>{
            Self::try_mutate_account(
                who,
                |account,
                 is_new|
                 -> Result<SignedImbalance<Self::Balance, Self::PositiveImbalance>, DispatchError> {
                    let ed = T::ExistentialDeposit::get();
                    //total = value + reserved
                    let total = value.saturating_add(account.reserved);
        
                    // if < ED, return Error -> unwrap_or_else(|DispatchError| SignedImbalance::Positive(Self::PositiveImbalance::zero()))
                    ensure!(total >= ed || !is_new, Error::<T, I>::ExistentialDeposit);
        
                    let imbalance = if account.free <= value {
                        SignedImbalance::Positive(PositiveImbalance::new(value - account.free))
                    } else {
                        SignedImbalance::Negative(NegativeImbalance::new(account.free - value))
                    };
                    account.free = value;
                    Self::deposit_event(Event::BalanceSet {
                        who: who.clone(),
                        free: account.free,
                        reserved: account.reserved,
                    });
                    Ok(imbalance)
                },
            )
            .unwrap_or_else(|_| SignedImbalance::Positive(Self::PositiveImbalance::zero()))
        }
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
