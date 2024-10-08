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
        new_attributes = new_attributes
            .clone()
            .concat(sc_format!("level:{};activity_days:0;calories_per_day:0", 1));

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
    #[endpoint(upgradeNft)]
    fn upgrade_nft(&self) {
        self.require_not_paused();

        let user = self.blockchain().get_caller();

        let payment = self.call_value().all_esdt_transfers();

        require!(payment.len() == 1, "Only one NFT should be sent.");

        let mut emr_nft_payment = None;

        let emr_nft = self.emr_nft().get();

        for data in payment.iter() {
            if data.token_identifier == emr_nft {
                emr_nft_payment = Some(data);
            }
        }
        let emorya_nft_payment = emr_nft_payment.expect("Emorya NFT payment not found.");

        let mut nft_attributes_buffer = self.blockchain().get_token_attributes::<NftAttributes>(
            &emorya_nft_payment.token_identifier,
            emorya_nft_payment.token_nonce,
        );
        nft_attributes_buffer.level += 1;
        let mut encoded_attributes = ManagedBuffer::new();
        nft_attributes_buffer
            .top_encode(&mut encoded_attributes)
            .unwrap();

        let mut args = ManagedArgBuffer::new();
        args.push_arg(&user);
        self.send_raw()
            .transfer_esdt_execute(
                &user,
                &emorya_nft_payment.token_identifier,
                &BigUint::from(emorya_nft_payment.token_nonce),
                self.blockchain().get_gas_left(),
                &encoded_attributes,
                &args,
            )
            .expect("Failed to transfer the updated NFT.");
    }

    #[endpoint(increaseLevel)]
    fn increase_level(&self) {}

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
}
