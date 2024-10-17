multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getIsScPaused)]
    #[storage_mapper("isScPaused")]
    fn is_sc_paused(&self) -> SingleValueMapper<bool>;

    #[view(getAllowedAddresses)]
    #[storage_mapper("allowedAddresses")]
    fn allowed_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;
}
