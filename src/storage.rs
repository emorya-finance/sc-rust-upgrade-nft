multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getEmrNft)]
    #[storage_mapper("emrNft")]
    fn emr_nft(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getIsScPaused)]
    #[storage_mapper("isScPaused")]
    fn is_sc_paused(&self) -> SingleValueMapper<bool>;
}
