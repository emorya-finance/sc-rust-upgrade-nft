multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait OwnerModule:
    crate::storage::StorageModule + crate::private::PrivateModule + crate::views::ViewsModule
{
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
    #[endpoint(addAllowedAddresses)]
    fn add_allowed_addresses(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        for address in addresses.into_iter() {
            self.allowed_addresses().insert(address);
        }
    }

    // TODO Add a function to remove allowed addresses
    #[only_owner]
    #[endpoint(removeAllowedAddresses)]
    fn remove_allowed_address(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        for address in addresses.into_iter() {
            self.allowed_addresses().swap_remove(&address);
        }
    }

    #[only_owner]
    #[endpoint(setUnbondingPeriod)]
    fn set_unbonding_period(&self, period: u64) {
        self.unbonding_period().set(period);
    }

    #[only_owner]
    #[endpoint(forceNftClaim)]
    fn force_claim(&self, user: ManagedAddress) {
        self.require_not_paused();

        require!(
            !self.nft_from_address(&user).is_empty(),
            "User does not have an NFT deposited. Try depositing first."
        );
        let nft = self.nft_from_address(&user).get();
        require!(
            self.nft_owner_address(&nft.identifier, nft.nonce).get() == user,
            "User is not the owner of the NFT."
        );

        self.tx()
            .to(&user)
            .single_esdt(&nft.identifier, nft.nonce, &BigUint::from(1u8))
            .transfer();

        self.nft_owner_address(&nft.identifier, nft.nonce).clear();
        self.nft_from_address(&user).clear();
        self.user_retrieve_epoch(&user).clear();
    }
}
