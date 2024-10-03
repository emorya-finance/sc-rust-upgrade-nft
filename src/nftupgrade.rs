multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
struct NftAttributes {
    level: u32,
}

#[multiversx_sc::module]
pub trait NftUpgradeModule:
    crate::storage::StorageModule + crate::private::PrivateModule + crate::owner::OwnerModule
{
    #[endpoint(upgradeNft)]
    #[only_owner]
    fn upgrade_nft(&self, user: ManagedAddress) {
        self.require_not_paused();
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
        let _ = self
            .send_raw()
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
}
