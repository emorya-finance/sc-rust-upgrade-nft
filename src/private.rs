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
            nft_identifier == self.get_nft_identifier()
                || nft_identifier == self.get_nft_identifier_investors(),
            "Invalid EMR NFT identifier."
        );
    }

    fn require_non_blocked_user(&self, user: &ManagedAddress) {
        require!(
            !self.blocked_users(user).get(),
            "You are blocked from actioning the NFT"
        );
    }
}
