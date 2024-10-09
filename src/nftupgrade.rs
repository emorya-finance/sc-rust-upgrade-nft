#![no_std]

pub mod owner;
pub mod private;
pub mod storage;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
struct NftAttributes {
    level: u32,
}

const IPFS_CID: &[u8] =
    b"bafybeihggf4ao72jrpc7oeyvyrfm6srxbtwybiyans4e37pxxxgrhrjx4y.ipfs.nftstorage.link";
const TAGS: &[u8] = b"tag1,tag2,tag3";

#[multiversx_sc::contract]
pub trait NftUpgrade:
    crate::storage::StorageModule + crate::private::PrivateModule + crate::owner::OwnerModule
{
    // ===================== Deployment & Upgrade =====================

    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // ===================== For Devnet initialization =====================

    /// Initialize a Test NFT with level 1 in attributes, plus some more info to match current EMR NFTs.
    #[payable("*")]
    #[endpoint(initialize)]
    fn initialize(&self) {
        // read caller
        let caller = self.blockchain().get_caller();

        // read NFT transfer
        let (nft_identifier, nft_nonce, _) = self.call_value().single_esdt().into_tuple();

        // prepare NFT attributes | I skip the IPFS CID and tags for now but you will need them in upgradeNft
        let mut new_attributes = ManagedBuffer::new();
        // new_attributes = new_attributes.clone().concat(sc_format!(
        //     "metadata:{}/{}.json",
        //     IPFS_CID,
        //     nft_nonce
        // ));
        // new_attributes = new_attributes.clone().concat(sc_format!(
        //     "tags:{};",
        //     TAGS
        // ));
        new_attributes = new_attributes.clone().concat(sc_format!(
            "level:{};activity_days:0;calories_per_day:0",
            14278
        ));

        // update NFT attributes
        self.send()
            .nft_update_attributes(&nft_identifier, nft_nonce, &new_attributes);

        // transfer NFT back to caller
        self.tx()
            .to(&caller)
            .single_esdt(
                &nft_identifier,
                nft_nonce,
                &BigUint::from(1u8), // NFT amount is always 1
            )
            .transfer();
    }

    // ===================== Endpoints =====================

    /// Upgrade an NFT to the same level but with more data in attributes.
    #[payable("*")]
    #[endpoint(upgradeNft)]
    fn upgrade_nft(&self) {
        self.require_not_paused();

        let user = self.blockchain().get_caller();

        let payment = self.call_value().single_esdt().into_tuple();

        let emr_nft_payment = payment.0;
        let token_nonce = payment.1;

        let _emr_nft = self.emr_nft().get();

        let nft_attributes_buffer = self.get_nft_attributes_level(
            self.blockchain().get_sc_address(),
            emr_nft_payment.clone(),
            token_nonce,
        );

        // so i need to work with string, separate with the quotes

        let next_level = match nft_attributes_buffer.parse_as_u64() {
            Some(level) => level + 1,
            None => 1,
        };

        let nft_attributes_buffer = nft_attributes_buffer.clone().concat(sc_format!(
            "metadata:{}/{}.json",
            IPFS_CID,
            token_nonce
        ));
        let nft_attributes_buffer = nft_attributes_buffer
            .clone()
            .concat(sc_format!("tags:{};", TAGS));

        let nft_attributes_buffer = nft_attributes_buffer
            .clone()
            .concat(sc_format!("level:{}", next_level));

        self.send()
            .nft_update_attributes(&emr_nft_payment, token_nonce, &nft_attributes_buffer);

        // transfer NFT back to caller
        self.tx()
            .to(&user)
            .single_esdt(
                &emr_nft_payment,
                token_nonce,
                &BigUint::from(1u8), // NFT amount is always 1
            )
            .transfer();
    }

    #[payable("*")]
    #[endpoint(increaseLevel)]
    fn increase_level(&self) {
        let user = self.blockchain().get_caller();

        let (nft_identifier, nft_nonce, _) = self.call_value().single_esdt().into_tuple();

        let _emr_nft = self.emr_nft().get();

        let nft_attributes_buffer = self.get_nft_attributes_level(
            self.blockchain().get_sc_address(),
            nft_identifier.clone(),
            nft_nonce,
        );

        let next_level = match nft_attributes_buffer.parse_as_u64() {
            Some(level) => level + 1,
            None => 1,
        };

        let nft_attributes_buffer = nft_attributes_buffer.clone().concat(sc_format!(
            "metadata:{}/{}.json",
            IPFS_CID,
            nft_nonce
        ));

        let nft_attributes_buffer = nft_attributes_buffer
            .clone()
            .concat(sc_format!("tags:{};", TAGS));

        let nft_attributes_buffer = nft_attributes_buffer
            .clone()
            .concat(sc_format!("level:{}", next_level));

        self.send()
            .nft_update_attributes(&nft_identifier, nft_nonce, &nft_attributes_buffer);

        // transfer NFT back to caller
        self.tx()
            .to(&user)
            .single_esdt(
                &nft_identifier,
                nft_nonce,
                &BigUint::from(1u8), // NFT amount is always 1
            )
            .transfer();
    }

    // ===================== Views =====================

    #[view(getNftAttributes)]
    fn get_nft_attributes(
        &self,
        owner: ManagedAddress,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
    ) -> ManagedBuffer {
        self.blockchain()
            .get_esdt_token_data(&owner, &token_identifier, token_nonce)
            .attributes
    }

    #[view(getNftAttributesLevel)]
    fn get_nft_attributes_level(
        &self,
        owner: ManagedAddress,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
    ) -> ManagedBuffer {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(&owner, &token_identifier, token_nonce)
            .attributes;

        if attributes.copy_slice(0, 6).unwrap() != b"level:" {
            sc_panic!("Attributes do not start with level.");
        }

        let mut semicolon_index = 7;
        let mut semicolon = attributes.copy_slice(semicolon_index, 1).unwrap();
        while semicolon != b";" {
            semicolon_index += 1;
            semicolon = attributes.copy_slice(semicolon_index, 1).unwrap();
        }

        attributes.copy_slice(6, semicolon_index - 6).unwrap()
    }
}
