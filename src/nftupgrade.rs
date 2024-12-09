#![no_std]

multiversx_sc::imports!();

pub mod constants;
pub mod managedbufferutils;
pub mod owner;
pub mod private;
pub mod storage;
pub mod views;

use constants::TAGS;
use managedbufferutils::ManagedBufferUtils;
use storage::UserNft;

#[multiversx_sc::contract]
pub trait NftUpgrade:
    crate::storage::StorageModule
    + crate::private::PrivateModule
    + crate::owner::OwnerModule
    + crate::views::ViewsModule
{
    // ===================== Deployment & Upgrade =====================

    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // ===================== For Devnet initialization =====================

    /// Initialize a Test NFT with level 1 in attributes, plus some more info to match current EMR NFTs.
    /// This will make an NFT similar to the current EMR NFTs.
    #[payable("*")]
    #[endpoint(initialize)]
    fn initialize(&self) {
        let caller = self.blockchain().get_caller();

        let (nft_identifier, nft_nonce, _) = self.call_value().single_esdt().into_tuple();

        require!(
            caller == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&caller),
            "You are not allowed to upgrade NFTs."
        );

        // prepare NFT attributes | I skip the IPFS CID and tags for now but you will need them in upgradeNft
        let mut new_attributes = ManagedBuffer::new();
        new_attributes = new_attributes
            .clone()
            .concat(sc_format!("level:{};activity_days:0;calories_per_day:0", 1));

        // Update NFT attributes
        self.send()
            .nft_update_attributes(&nft_identifier, nft_nonce, &new_attributes);

        // Transfer NFT back to caller
        self.tx()
            .to(&caller)
            .single_esdt(&nft_identifier, nft_nonce, &BigUint::from(1u8))
            .transfer();
    }

    // ===================== Endpoints =====================

    // Deposit the NFT and save the actual owner address in the storage
    #[payable("*")]
    #[endpoint(depositNft)]
    fn deposit_nft(&self) {
        self.require_not_paused();
        let user = self.blockchain().get_caller();

        let (emr_nft_token, token_nonce, amount) = self.call_value().single_esdt().into_tuple();
        self.require_valid_emr_nft(emr_nft_token.clone());
        require!(
            amount == BigUint::from(1u8),
            "You can only deposit one NFT at a time."
        );

        require!(
            self.nft_from_address(&user).is_empty(),
            "You already deposited one NFT."
        );
        require!(
            self.nft_owner_address(&emr_nft_token, token_nonce)
                .is_empty(),
            "This NFT is already deposited."
        );

        // Storage
        self.nft_owner_address(&emr_nft_token, token_nonce)
            .set(user.clone());
        self.nft_from_address(&user).set(UserNft {
            identifier: emr_nft_token.clone(),
            nonce: token_nonce,
        });

        let attributes = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &emr_nft_token,
                token_nonce,
            )
            .attributes;

        if attributes.copy_slice(0, 6).unwrap() == b"level:" {
            let level =
                self.get_nft_attributes_level_before_upgrade(emr_nft_token.clone(), token_nonce);

            let uri_json = self.get_nft_uri_json(emr_nft_token.clone(), token_nonce);

            // prepare NFT attributes | Format is metadata:IPFS_CID/NFT_NONCE.json;tags:TAGS;level:LEVEL
            let mut new_attributes = ManagedBuffer::new();
            new_attributes = new_attributes
                .clone()
                .concat(sc_format!("metadata:{};", uri_json));

            new_attributes = new_attributes.clone().concat(sc_format!("tags:{};", TAGS));
            new_attributes = new_attributes.clone().concat(sc_format!("level:{}", level));

            // Update NFT attributes
            self.send()
                .nft_update_attributes(&emr_nft_token.clone(), token_nonce, &new_attributes);
        }
    }

    #[endpoint(retrieveNft)]
    fn retrieve_nft(&self) {
        self.require_not_paused();

        let user = self.blockchain().get_caller();

        require!(
            !self.nft_from_address(&user).is_empty(),
            "You do not have an NFT deposited. Try depositing first."
        );
        let nft = self.nft_from_address(&user).get();
        require!(
            self.nft_owner_address(&nft.identifier, nft.nonce).get() == user,
            "You are not the owner of the NFT."
        );

        self.tx()
            .to(&user)
            .single_esdt(&nft.identifier, nft.nonce, &BigUint::from(1u8))
            .transfer();

        // Storage
        self.nft_owner_address(&nft.identifier, nft.nonce).clear();
        self.nft_from_address(&user).clear();
    }

    /// Upgrade an NFT to the same level but with more data in attributes.
    #[payable("*")]
    #[endpoint(upgradeNft)]
    fn upgrade_nft(&self) {
        self.require_not_paused();

        let caller = self.blockchain().get_caller();

        let (emr_nft_token, token_nonce, amount) = self.call_value().single_esdt().into_tuple();

        self.require_valid_emr_nft(emr_nft_token.clone());

        require!(
            amount == BigUint::from(1u8),
            "You can only upgrade one NFT at a time."
        );

        let level =
            self.get_nft_attributes_level_before_upgrade(emr_nft_token.clone(), token_nonce);

        let uri_json = self.get_nft_uri_json(emr_nft_token.clone(), token_nonce);

        // prepare NFT attributes | Format is metadata:IPFS_CID/NFT_NONCE.json;tags:TAGS;level:LEVEL
        let mut new_attributes = ManagedBuffer::new();
        new_attributes = new_attributes
            .clone()
            .concat(sc_format!("metadata:{};", uri_json));

        new_attributes = new_attributes.clone().concat(sc_format!("tags:{};", TAGS));
        new_attributes = new_attributes.clone().concat(sc_format!("level:{}", level));

        // Update NFT attributes
        self.send()
            .nft_update_attributes(&emr_nft_token.clone(), token_nonce, &new_attributes);

        self.tx()
            .to(&caller)
            .single_esdt(&emr_nft_token, token_nonce, &BigUint::from(1u8))
            .transfer();
    }

    #[payable("*")]
    #[endpoint(increaseLevel)]
    fn increase_level(&self, user: ManagedAddress) {
        self.require_not_paused();

        require!(
            !self.nft_from_address(&user).is_empty(),
            "The user has no NFT deposited!"
        );
        let nft = self.nft_from_address(&user).get();

        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&caller),
            "You are not allowed to modify NFT level."
        );

        let level = self.get_nft_attributes_level_after_upgrade(nft.identifier.clone(), nft.nonce);

        let level = level.ascii_to_u64().unwrap();

        let new_level = level + 1;

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

        // Update NFT attributes
        self.send()
            .nft_update_attributes(&nft.identifier, nft.nonce, &new_attributes);
    }

    #[payable("*")]
    #[endpoint(decreaseLevel)]
    fn decrease_level(&self, user: ManagedAddress) {
        self.require_not_paused();

        require!(
            !self.nft_from_address(&user).is_empty(),
            "The user has no NFT deposited!"
        );
        let nft = self.nft_from_address(&user).get();

        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&caller),
            "You are not allowed to modify NFT level."
        );

        let level = self.get_nft_attributes_level_after_upgrade(nft.identifier.clone(), nft.nonce);

        let level = level.ascii_to_u64().unwrap();

        require!(level > 1, "NFT level cannot be less than 1.");

        let new_level = level - 1;

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

        // Update NFT attributes
        self.send()
            .nft_update_attributes(&nft.identifier, nft.nonce, &new_attributes);
    }
}
