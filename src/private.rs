multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait PrivateModule: crate::storage::StorageModule + crate::views::ViewsModule {
    fn require_not_paused(&self) {
        require!(
            !self.is_sc_paused().get(),
            "This smart contract is currently paused. All actions are haulted."
        );
    }

    fn require_valid_emr_nft(&self, nft_identifier: TokenIdentifier) {
        require!(
            nft_identifier == self.get_nft_identifier(),
            "Invalid EMR NFT identifier."
        );
    }
}
