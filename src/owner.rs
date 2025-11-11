use crate::constants::TAGS;

use multiversx_sc::imports::*;

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

    #[endpoint(setLevel)]
    fn set_level(&self, address: ManagedAddress, new_level: u64, category: u64) {
        let caller = self.blockchain().get_caller();

        require!(
            caller == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&caller),
            "You are not allowed to use this function."
        );

        let nft = if category == 1 {
            self.nft_from_address(&address).get()
        } else {
            self.nft_retrieve_from_address(&address).get()
        };

        let uri_json = self.get_nft_uri_json(nft.identifier.clone(), nft.nonce);
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

    #[endpoint(blockUser)]
    fn block_user(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        let caller = self.blockchain().get_caller();

        require!(
            caller == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&caller),
            "You are not allowed to use this function."
        );

        for address in addresses.into_iter() {
            self.blocked_users(&address).set(true);
        }
    }

    #[endpoint(unBlockUser)]
    fn unblock_user(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        let caller = self.blockchain().get_caller();

        require!(
            caller == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&caller),
            "You are not allowed to use this function."
        );

        for address in addresses.into_iter() {
            self.blocked_users(&address).set(false);
        }
    }

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

    #[only_owner]
    #[endpoint(reclaimNft)]
    fn reclaim_nft(&self, address: ManagedAddress) {
        require!(
            !self.nft_from_address(&address).is_empty(),
            "The user has no NFT deposited!"
        );
        let nft = self.nft_from_address(&address).get();
        let owner_address = self.blockchain().get_owner_address();

        self.tx()
            .to(&owner_address)
            .single_esdt(&nft.identifier, nft.nonce, &BigUint::from(1u8))
            .transfer();

        self.nft_from_address(&address).clear();
    }

    #[only_owner]
    #[endpoint(forceNftClaim)]
    fn force_claim(&self, token: TokenIdentifier, nonce: u64, address: ManagedAddress) {
        let user = self.nft_owner_address(&token, nonce).get();
        require!(
            address == user,
            "The provided address does not match the NFT owner address"
        );

        self.tx()
            .to(&user)
            .single_esdt(&token, nonce, &BigUint::from(1u8))
            .transfer();

        self.nft_owner_address(&token, nonce).clear();

        if !self.nft_from_address(&user).is_empty() {
            let nft_from_address = self.nft_from_address(&user).get();
            if nft_from_address.identifier == token && nft_from_address.nonce == nonce {
                self.nft_from_address(&user).clear();
            }
        }

        if !self.nft_retrieve_from_address(&user).is_empty() {
            let nft_retrieve_from_address = self.nft_retrieve_from_address(&user).get();
            if nft_retrieve_from_address.identifier == token
                && nft_retrieve_from_address.nonce == nonce
            {
                self.nft_retrieve_from_address(&user).clear();
                self.user_retrieve_epoch(&user).clear();
            }
        }
    }
}
