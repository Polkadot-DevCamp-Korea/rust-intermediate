## Pallet Balances

Deep dive into Balances pallet


## Skeleton Code

> Schemes of Balance Pallet withouut any types/logic

1. Dispatch Calls

````
1. Dispatch Calls
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

## Reference
[Balances Pallet](https://github.com/paritytech/substrate/tree/master/frame/balances)
