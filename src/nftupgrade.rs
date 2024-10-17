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
            "You are not allowed to allocate EMRS."
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

    /// Upgrade an NFT to the same level but with more data in attributes.
    #[payable("*")]
    #[endpoint(upgradeNft)]
    fn upgrade_nft(&self) {
        self.require_not_paused();

        let user = self.blockchain().get_caller();

        let (emr_nft_token, emr_nft_nonce, _) = self.call_value().single_esdt().into_tuple();
        // self.require_valid_emr_nft(emr_nft_token.clone());

        require!(
            user == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&user),
            "You are not to upgrade an NFT."
        );

        let level = self.get_nft_attributes_level_before_upgrade(
            self.blockchain().get_sc_address(),
            emr_nft_token.clone(),
            emr_nft_nonce,
        );

        let uri_json = self.get_nft_uri_json(
            self.blockchain().get_sc_address(),
            emr_nft_token.clone(),
            emr_nft_nonce,
        );

        // prepare NFT attributes | Format is metadata:IPFS_CID/NFT_NONCE.json;tags:TAGS;level:LEVEL
        let mut new_attributes = ManagedBuffer::new();
        new_attributes = new_attributes
            .clone()
            .concat(sc_format!("metadata:{};", uri_json));

        new_attributes = new_attributes.clone().concat(sc_format!("tags:{};", TAGS));
        new_attributes = new_attributes.clone().concat(sc_format!("level:{}", level));

        // Update NFT attributes
        self.send()
            .nft_update_attributes(&emr_nft_token, emr_nft_nonce, &new_attributes);

        // Transfer NFT back to caller
        self.tx()
            .to(&user)
            .single_esdt(&emr_nft_token, emr_nft_nonce, &BigUint::from(1u8))
            .transfer();
    }

    /// Increase the level of an NFT by 1.
    #[payable("*")]
    #[endpoint(increaseLevel)]
    fn increase_level(&self) {
        self.require_not_paused();

        let user = self.blockchain().get_caller();

        let (emr_nft_token, emr_nft_nonce, _) = self.call_value().single_esdt().into_tuple();
        // self.require_valid_emr_nft(emr_nft_token.clone());

        require!(
            user == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&user),
            "You are not allowed to allocate EMRS."
        );

        let level = self.get_nft_attributes_level_after_upgrade(
            self.blockchain().get_sc_address(),
            emr_nft_token.clone(),
            emr_nft_nonce,
        );

        let level = level.ascii_to_u64().unwrap();

        let new_level = level + 1;

        let uri_json = self.get_nft_uri_json(
            self.blockchain().get_sc_address(),
            emr_nft_token.clone(),
            emr_nft_nonce,
        );

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
            .nft_update_attributes(&emr_nft_token, emr_nft_nonce, &new_attributes);

        // Transfer NFT back to caller
        self.tx()
            .to(&user)
            .single_esdt(&emr_nft_token, emr_nft_nonce, &BigUint::from(1u8))
            .transfer();
    }

    /// Decrease the level of an NFT by 1.
    #[payable("*")]
    #[endpoint(decreaseLevel)]
    fn decrease_level(&self) {
        self.require_not_paused();

        let user = self.blockchain().get_caller();

        let (emr_nft_token, emr_nft_nonce, _) = self.call_value().single_esdt().into_tuple();
        // self.require_valid_emr_nft(emr_nft_token.clone());

        require!(
            user == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&user),
            "You are not allowed to allocate EMRS."
        );

        let level = self.get_nft_attributes_level_after_upgrade(
            self.blockchain().get_sc_address(),
            emr_nft_token.clone(),
            emr_nft_nonce,
        );

        let level = level.ascii_to_u64().unwrap();

        require!(level > 1, "NFT level cannot be less than 1.");

        let new_level = level - 1;

        let uri_json = self.get_nft_uri_json(
            self.blockchain().get_sc_address(),
            emr_nft_token.clone(),
            emr_nft_nonce,
        );

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
            .nft_update_attributes(&emr_nft_token, emr_nft_nonce, &new_attributes);

        // Transfer NFT back to caller
        self.tx()
            .to(&user)
            .single_esdt(&emr_nft_token, emr_nft_nonce, &BigUint::from(1u8))
            .transfer();
    }
}
