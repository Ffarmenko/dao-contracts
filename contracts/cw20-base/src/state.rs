use cosmwasm_std::{ReadonlyStorage, Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub use cw20::MetaResponse as Meta;

const META_KEY: &[u8] = b"meta";
const PREFIX_BALANCE: &[u8] = b"balance";

// meta is the token definition as well as the total_supply
pub fn meta<S: Storage>(storage: &mut S) -> Singleton<S, Meta> {
    singleton(storage, META_KEY)
}

pub fn meta_read<S: ReadonlyStorage>(storage: &S) -> ReadonlySingleton<S, Meta> {
    singleton_read(storage, META_KEY)
}

/// balances are state of the erc20 tokens
pub fn balances<S: Storage>(storage: &mut S) -> Bucket<S, Uint128> {
    bucket(PREFIX_BALANCE, storage)
}

pub fn balances_read<S: ReadonlyStorage>(storage: &S) -> ReadonlyBucket<S, Uint128> {
    bucket_read(PREFIX_BALANCE, storage)
}
