## Pallet Balances

Deep dive into Balances pallet


## Skeleton Code

> Schemes of Balance Pallet withouut any types/logic

1. Dispatchable Calls

````
pub fn transfer() 
pub fn set_balance()
pub fn force_transfer()
pub fn transfer_keep_alive()
pub fn transfer_all()
pub fn force_unreserve()
````

2. Storage

````
pub type TotalIssuance = StorageValue
pub type Account = StorageMap
pub type Locks = StorageMap
pub type Reserves = StorageMap
pub type StorageVersion = StorageValue
````

3. Data Struct

````
pub struct BalanceLock
pub struct ReserveDate
pub struct AccountData
````

4. Internal/External Functions

````
pub fn free_balance() -> T::Balance
pub fn usable_balance() -> T::Balance
pub fn usable_balance_for_fees() ->T::Balance
pub fn reserved_balance() -> T::Balance
fn account() -> AccountData
fn post_mutation() -> (Option<AccountData<T::Balance>>, Option<NegativeImbalance<T, I>>)
fn deposit_consequence() -> DepositConsequence
fn withdraw_consequence() -> WithdrawConsequence
pub fn mutate_account<R>() -> Result<R, DispatchError>
fn try_mutate_account<R, E: From<DispatchError>>() -> Result<R,E>
fn try_mutate_account_with_dust<R, E: From<DispatchError>>() -> Result<R, DustCleaner<T, I>, E>
fn update_locks() 
fn do_transfer_reserved() -> Result<T::Balance, DispatchError>
````

5. Trait impls

````
fungible
Currency
Reservable Currency
NamedReservableCurrency
LockableCurrency
````

6. Config(Associated Types)

````
type Balance
type DustRemoval
type Event
type ExistentialDeposit
type AccountStore
type WeightInfo
type MaxLocks
type MaxReserves
type ReserveIdentifier
````

7. Event

````
Endowed,
DustLost,
Transfer,
BalacneSet,
Reserved,
Unreserved,
ReserveRepatraited,
Deposit,
Withdraw,
Slashed
````

8. Error

````
VestingBalance,
LiquidityRestrictions,
InsufficientBalance,
ExistentialDeposit,
KeepAlive,
ExistingVestingSchedule,
DeadAccount,
TooManyReserved
````

## Reference
[Balances Pallet](https://github.com/paritytech/substrate/tree/master/frame/balances)
