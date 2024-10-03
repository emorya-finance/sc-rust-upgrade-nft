#![no_std]

pub mod nftupgrade;
pub mod owner;
pub mod private;
mod proxy;
pub mod storage;

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SC:
    storage::StorageModule + private::PrivateModule + owner::OwnerModule + nftupgrade::NftUpgradeModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
