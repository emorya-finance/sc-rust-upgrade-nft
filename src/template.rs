#![no_std]

mod proxy;

multiversx_sc::imports!();

/// Empty Contract
#[multiversx_sc::contract]
pub trait Template {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
