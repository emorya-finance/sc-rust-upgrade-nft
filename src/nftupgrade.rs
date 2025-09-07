#![no_std]

use constants::TAGS;
use managedbufferutils::ManagedBufferUtils;
use multiversx_sc::imports::*;
use storage::UserNft;

pub mod constants;
pub mod managedbufferutils;
pub mod owner;
pub mod private;
pub mod storage;
pub mod views;

#[multiversx_sc::contract]
pub trait NftUpgrade:
    crate::storage::StorageModule
    + crate::private::PrivateModule
    + crate::owner::OwnerModule
    + crate::views::ViewsModule
{
    // === Deployment & Upgrade ===

    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    /// Only for testing purposes in Devnet.
    /// Initialize a Test NFT with level 1 in attributes, plus some more info to match current EMR NFTs.
    /// This will make an NFT similar to the current EMR NFTs.
    #[payable("*")]
    #[endpoint(initialize)]
    fn initialize(&self) {
        let caller = self.blockchain().get_caller();

        let (nft_identifier, nft_nonce, _) = self.call_value().single_esdt().clone().into_tuple();

        require!(
            caller == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&caller),
            "You are not allowed to upgrade NFTs."
        );

        let mut new_attributes = ManagedBuffer::new();
        new_attributes = new_attributes
            .clone()
            .concat(sc_format!("level:{},activity_days:0,calories_per_day:0", 1));

        self.send()
            .nft_update_attributes(&nft_identifier, nft_nonce, &new_attributes);

        self.tx()
            .to(&caller)
            .single_esdt(&nft_identifier, nft_nonce, &BigUint::from(1u8))
            .transfer();
    }

    // === Public Endpoints ===

    /// Allows a user to deposit an NFT into the contract to be able to "lock" it and enjoy the benefits.
    /// Only 1 NFT per user can be active at a time.
    #[payable("*")]
    #[endpoint(depositNft)]
    fn deposit_nft(&self) {
        self.require_not_paused();
        let user = self.blockchain().get_caller();
        self.require_non_blocked_user(&user);

        let (emr_nft_token, token_nonce, amount) =
            self.call_value().single_esdt().clone().into_tuple();
        self.require_valid_emr_nft(emr_nft_token.clone());
        require!(
            amount == BigUint::from(1u8),
            "You can only deposit one NFT at a time."
        );
        require!(
            self.nft_from_address(&user).is_empty(),
            "You already have an active NFT deposited."
        );
        require!(
            self.nft_owner_address(&emr_nft_token, token_nonce)
                .is_empty(),
            "This NFT is already in retrieval process."
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

            let mut new_attributes = ManagedBuffer::new();
            new_attributes = new_attributes
                .clone()
                .concat(sc_format!("metadata:{};", uri_json));

            new_attributes = new_attributes.clone().concat(sc_format!("tags:{};", TAGS));
            new_attributes = new_attributes.clone().concat(sc_format!("level:{}", level));

            self.send()
                .nft_update_attributes(&emr_nft_token, token_nonce, &new_attributes);
        }
    }

    /// Allows a user to iniate the retrieval of an NFT.
    /// This will start the unbonding period.
    /// After the unbonding period, the user can claim their NFT.
    /// While the NFT is in retrieval, it cannot be used for any benefits but the user can deposit a new NFT as active.
    #[endpoint(retrieveNft)]
    fn retrieve_nft(&self) {
        self.require_not_paused();

        let user = self.blockchain().get_caller();
        self.require_non_blocked_user(&user);

        require!(
            self.user_retrieve_epoch(&user).is_empty(),
            "You already started the retrieval process."
        );
        require!(
            !self.nft_from_address(&user).is_empty(),
            "You do not have an NFT deposited."
        );

        let nft = self.nft_from_address(&user).get();
        require!(
            self.nft_owner_address(&nft.identifier, nft.nonce).get() == user,
            "You are not the owner of the NFT."
        );

        let current_epoch = self.blockchain().get_block_epoch();

        // Storage
        self.nft_from_address(&user).clear();
        self.nft_retrieve_from_address(&user).set(UserNft {
            identifier: nft.identifier,
            nonce: nft.nonce,
        });
        self.user_retrieve_epoch(&user).set(current_epoch);
    }

    /// Allows a user to claim their NFT after the unbonding period.
    /// This will transfer the NFT back to the user.
    /// The user must have initiated the retrieval process first.
    /// If the unbonding period is not over, the user cannot claim the NFT.
    #[endpoint(claimNft)]
    fn claim_nft(&self) {
        self.require_not_paused();

        let user = self.blockchain().get_caller();
        self.require_non_blocked_user(&user);

        require!(
            !self.user_retrieve_epoch(&user).is_empty(),
            "First, retrieve the NFT and wait for the unbonding period to end."
        );
        require!(
            !self.nft_retrieve_from_address(&user).is_empty(),
            "You do not have an NFT deposited. Try depositing first."
        );
        let nft = self.nft_retrieve_from_address(&user).get();
        require!(
            self.nft_owner_address(&nft.identifier, nft.nonce).get() == user,
            "You are not the owner of the NFT."
        );

        let current_epoch = self.blockchain().get_block_epoch();
        let unbounding_period = self.unbonding_period().get();
        let user_retrieve_epoch = self.user_retrieve_epoch(&user).get();
        let unclaimed_epochs = current_epoch - user_retrieve_epoch;

        if unclaimed_epochs <= unbounding_period {
            sc_panic!("You have to wait until the unbounding period is over.");
        } else {
            self.tx()
                .to(&user)
                .single_esdt(&nft.identifier, nft.nonce, &BigUint::from(1u8))
                .transfer();

            self.nft_owner_address(&nft.identifier, nft.nonce).clear();
            self.nft_retrieve_from_address(&user).clear();
            self.user_retrieve_epoch(&user).clear();
        }
    }

    /// Upgrade an NFT to the same level but with the new attributes.
    /// This will update the NFT attributes to match the current EMR NFTs.
    #[payable("*")]
    #[endpoint(upgradeNft)]
    fn upgrade_nft(&self) {
        self.require_not_paused();

        let caller = self.blockchain().get_caller();

        let (emr_nft_token, token_nonce, amount) =
            self.call_value().single_esdt().clone().into_tuple();

        self.require_valid_emr_nft(emr_nft_token.clone());

        require!(
            amount == BigUint::from(1u8),
            "You can only upgrade one NFT at a time."
        );

        let level =
            self.get_nft_attributes_level_before_upgrade(emr_nft_token.clone(), token_nonce);

        let uri_json = self.get_nft_uri_json(emr_nft_token.clone(), token_nonce);

        let mut new_attributes = ManagedBuffer::new();
        new_attributes = new_attributes
            .clone()
            .concat(sc_format!("metadata:{};", uri_json));

        new_attributes = new_attributes.clone().concat(sc_format!("tags:{};", TAGS));
        new_attributes = new_attributes.clone().concat(sc_format!("level:{}", level));

        self.send()
            .nft_update_attributes(&emr_nft_token.clone(), token_nonce, &new_attributes);

        self.tx()
            .to(&caller)
            .single_esdt(&emr_nft_token, token_nonce, &BigUint::from(1u8))
            .transfer();
    }

    // === Admin Endpoints ===

    /// Increase the level of an NFT by 1.
    /// This can only be done by the owner or an allowed address.
    #[endpoint(increaseLevel)]
    fn increase_level(&self, user: ManagedAddress) {
        self.require_not_paused();
        require!(
            !self.nft_from_address(&user).is_empty(),
            "The user has no NFT deposited!"
        );

        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&caller),
            "You are not allowed to modify NFT level."
        );

        let nft = self.nft_from_address(&user).get();
        let level = self.get_nft_attributes_level_after_upgrade(nft.identifier.clone(), nft.nonce);
        let level = level.ascii_to_u64().unwrap();
        let new_level = level + 1;
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

    /// Decrease the level of an NFT by 1.
    /// This can only be done by the owner or an allowed address.
    #[endpoint(decreaseLevel)]
    fn decrease_level(&self, user: ManagedAddress) {
        self.require_not_paused();
        require!(
            !self.nft_from_address(&user).is_empty(),
            "The user has no NFT deposited!"
        );

        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address()
                || self.allowed_addresses().contains(&caller),
            "You are not allowed to modify NFT level."
        );

        let nft = self.nft_from_address(&user).get();
        let level = self.get_nft_attributes_level_after_upgrade(nft.identifier.clone(), nft.nonce);
        let level = level.ascii_to_u64().unwrap();
        require!(level > 1, "NFT level cannot be less than 1.");
        let new_level = level - 1;
        let uri_json = self.get_nft_uri_json(nft.identifier.clone(), nft.nonce);

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
