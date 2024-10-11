multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait OwnerModule: crate::storage::StorageModule + crate::private::PrivateModule {
    #[only_owner]
    #[endpoint(pauseSc)]
    fn pause_sc(&self) {
        self.is_sc_paused().set(true);
    }

    #[only_owner]
    #[endpoint(resumeSc)]
    fn resume_sc(&self) {
        self.is_sc_paused().set(false);
    }

    #[only_owner]
    #[endpoint(setEmrNftIdentifier)]
    fn set_emr_nft_identifier(&self, identifier: TokenIdentifier) {
        self.emr_nft_identifier().set(identifier);
    }
}
