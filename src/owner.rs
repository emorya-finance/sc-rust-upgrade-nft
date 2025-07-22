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
    #[endpoint(setLevel)]
    fn set_level(&self, address: ManagedAddress, new_level: u64, category: u64) {
        let nft;
        if category == 1 {
            nft = self.nft_from_address(&address).get();
        } else {
            nft = self.nft_retrieve_from_address(&address).get();
        }

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
    #[endpoint(blockUser)]
    fn block_user(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        for address in addresses.into_iter() {
            self.blocked_users(&address).set(true);
        }
    }

    #[only_owner]
    #[endpoint(unBlockUser)]
    fn unblock_user(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        for address in addresses.into_iter() {
            self.blocked_users(&address).set(false);
        }
    }

    // Deprecated - to be removed
    // #[endpoint(updateStorage)]
    // fn update_storage(&self, addresses: MultiValueEncoded<ManagedAddress>) {
    //     for user in addresses {
    //         let nft = self.nft_from_address(&user).get();
    //         self.nft_from_address(&user).clear();
    //         self.nft_retrieve_from_address(&user).set(UserNft {
    //             identifier: nft.identifier,
    //             nonce: nft.nonce,
    //         });
    //     }
    // }

    #[only_owner]
    #[endpoint(addAllowedAddresses)]
    fn add_allowed_addresses(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        for address in addresses.into_iter() {
            self.allowed_addresses().insert(address);
        }
    }

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

    // NEEDS UPDATE
    // #[only_owner]
    // #[endpoint(forceNftClaim)]
    // fn force_claim(&self, user: ManagedAddress) {
    //     self.require_not_paused();

    //     require!(
    //         !self.nft_from_address(&user).is_empty(),
    //         "User does not have an NFT deposited. Try depositing first."
    //     );
    //     let nft = self.nft_from_address(&user).get();
    //     require!(
    //         self.nft_owner_address(&nft.identifier, nft.nonce).get() == user,
    //         "User is not the owner of the NFT."
    //     );

    //     // let current_epoch = self.blockchain().get_block_epoch();
    //     // let unbounding_period = self.unbonding_period().get();
    //     // let user_retrieve_epoch = self.user_retrieve_epoch(&user).get();

    //     self.tx()
    //         .to(&user)
    //         .single_esdt(&nft.identifier, nft.nonce, &BigUint::from(1u8))
    //         .transfer();

    //     self.nft_owner_address(&nft.identifier, nft.nonce).clear();
    //     self.nft_from_address(&user).clear();
    //     self.user_retrieve_epoch(&user).clear();
    // }
}
