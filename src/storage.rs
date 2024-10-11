multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getEmrNftIdentifier)]
    #[storage_mapper("emrNftIdentifier")]
    fn emr_nft_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getIsScPaused)]
    #[storage_mapper("isScPaused")]
    fn is_sc_paused(&self) -> SingleValueMapper<bool>;
}
