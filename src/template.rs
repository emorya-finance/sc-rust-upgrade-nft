#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod proxy;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Clone)]
pub enum State {
    State1,
    State2,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, ManagedVecItem)]
pub struct Structure<M: ManagedTypeApi> {
    pub param1: u64,
    pub param2: EgldOrEsdtTokenIdentifier<M>,
}

type MultiValueType<M> = MultiValue3<u64, ManagedAddress<M>, ManagedVec<M, BigUint<M>>>;

#[multiversx_sc::contract]
pub trait Template {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(endpointExample)]
    fn endpoint_example(&self) {}

    #[payable("*")]
    #[only_owner]
    #[endpoint(ownerPayableEndpointExample)]
    fn owner_payable_endpoint_example(&self) {}

    #[view(viewExample)]
    fn view_example(&self) {}

    #[view(getExampleMapper)]
    #[storage_mapper("exampleMapper")]
    fn get_example_mapper(&self) -> SingleValueMapper<u64>;

    #[event("example_event")]
    fn example(&self, #[indexed] id: &u64);

    #[proxy]
    fn example_proxy(&self, to: ManagedAddress) -> example_proxy::Proxy<Self::Api>;
}

pub mod example_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait ExampleProxy {}
}
