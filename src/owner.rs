use crate::constants::TAGS;

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
    #[endpoint(upgradeInvestorsNft)]
    fn upgrade_investors_nft(&self, token_identifier: TokenIdentifier, token_nonce: u64) {
        // Upgrade function needs revise
    }

    fn set_manual_level(
        &self,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
        new_level: u64,
    ) {
        let uri_json = self.get_nft_uri_json(token_identifier.clone(), token_nonce);

        // prepare NFT attributes | Format is metadata:IPFS_CID/NFT_NONCE.json;tags:TAGS;level:LEVEL
        let mut new_attributes = ManagedBuffer::new();
        new_attributes = new_attributes
            .clone()
            .concat(sc_format!("metadata:{};", uri_json));

        new_attributes = new_attributes.clone().concat(sc_format!("tags:{};", TAGS));
        new_attributes = new_attributes
            .clone()
            .concat(sc_format!("level:{}", new_level));

        self.send()
            .nft_update_attributes(&token_identifier, token_nonce, &new_attributes);
    }

    #[only_owner]
    #[endpoint(setLevel)]
    fn set_level(&self, address: ManagedAddress, new_level: u64) {
        let nft = self.nft_from_address(&address).get();

        let uri_json = self.get_nft_uri_json(nft.identifier.clone(), nft.nonce);

        // prepare NFT attributes | Format is metadata:IPFS_CID/NFT_NONCE.json;tags:TAGS;level:LEVEL
        let mut new_attributes = ManagedBuffer::new();
        new_attributes = new_attributes
            .clone()
            .concat(sc_format!("metadata:{};", uri_json));

        new_attributes = new_attributes.clone().concat(sc_format!("tags:{};", TAGS));
        new_attributes = new_attributes
            .clone()
            .concat(sc_format!("level:{}", new_level));

        self.send()
            .nft_update_attributes(&nft.identifier, nft.nonce, &new_attributes);
    }

    #[only_owner]
    #[endpoint(downgradeLevel)]
    fn downgrade_nft_level(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        for address in addresses {
            let nft = self.nft_from_address(&address).get();
            let new_level = 1u64;

            let uri_json = self.get_nft_uri_json(nft.identifier.clone(), nft.nonce);

            // prepare NFT attributes | Format is metadata:IPFS_CID/NFT_NONCE.json;tags:TAGS;level:LEVEL
            let mut new_attributes = ManagedBuffer::new();
            new_attributes = new_attributes
                .clone()
                .concat(sc_format!("metadata:{};", uri_json));

            new_attributes = new_attributes.clone().concat(sc_format!("tags:{};", TAGS));
            new_attributes = new_attributes
                .clone()
                .concat(sc_format!("level:{}", new_level));

            self.blocked_user(&address).clear();

            self.send()
                .nft_update_attributes(&nft.identifier, nft.nonce, &new_attributes);
        }
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
    #[endpoint(addBlockUser)]
    fn add_block_user(&self, users: MultiValueEncoded<ManagedAddress>) {
        for user in users {
            self.blocked_user(&user).set(true);
        }
    }

    #[only_owner]
    #[endpoint(removeBlockUser)]
    fn remove_block_user(&self, users: MultiValueEncoded<ManagedAddress>) {
        for user in users {
            self.blocked_user(&user).set(false);
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

        // let current_epoch = self.blockchain().get_block_epoch();
        // let unbounding_period = self.unbonding_period().get();
        // let user_retrieve_epoch = self.user_retrieve_epoch(&user).get();

        self.tx()
            .to(&user)
            .single_esdt(&nft.identifier, nft.nonce, &BigUint::from(1u8))
            .transfer();

        self.nft_owner_address(&nft.identifier, nft.nonce).clear();
        self.nft_from_address(&user).clear();
        self.user_retrieve_epoch(&user).clear();
    }
}
