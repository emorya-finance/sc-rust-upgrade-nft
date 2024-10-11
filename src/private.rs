multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait PrivateModule: crate::storage::StorageModule {
    fn require_not_paused(&self) {
        require!(
            !self.is_sc_paused().get(),
            "This smart contract is currently paused. All actions are haulted."
        );
    }

    fn require_emr_nft_identifier(&self) {
        require!(
            !self.emr_nft_identifier().is_empty(),
            "EMR NFT identifier is not set. Please set it first."
        );
    }

    fn require_valid_emr_nft(&self, nft_identifier: TokenIdentifier) {
        self.require_emr_nft_identifier();

        require!(
            nft_identifier == self.emr_nft_identifier().get(),
            "Invalid EMR NFT identifier."
        );
    }
}
