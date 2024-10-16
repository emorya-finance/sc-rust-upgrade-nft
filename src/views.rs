use crate::constants::{IPFS_CID, NFT_IDENTIFIER, TAGS};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ViewsModule: crate::storage::StorageModule {
    #[view(getIpfsCid)]
    fn get_ipfs_cid(&self) -> ManagedBuffer {
        ManagedBuffer::new_from_bytes(IPFS_CID)
    }

    #[view(getTags)]
    fn get_tags(&self) -> ManagedBuffer {
        ManagedBuffer::new_from_bytes(TAGS)
    }

    #[view(getNftIdentifier)]
    fn get_nft_identifier(&self) -> TokenIdentifier {
        TokenIdentifier::from_esdt_bytes(NFT_IDENTIFIER)
    }

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

    #[view(getNftAttributesLevelBeforeUpgrade)]
    fn get_nft_attributes_level_before_upgrade(
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
            sc_panic!("Attributes do not start as expected.");
        }

        let mut semicolon_index = 7;
        let mut semicolon = attributes.copy_slice(semicolon_index, 1).unwrap();
        while semicolon != b";" {
            semicolon_index += 1;
            semicolon = attributes.copy_slice(semicolon_index, 1).unwrap();
        }

        attributes.copy_slice(6, semicolon_index - 6).unwrap()
    }

    #[view(getNftAttributesLevelAfterUpgrade)]
    fn get_nft_attributes_level_after_upgrade(
        &self,
        owner: ManagedAddress,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
    ) -> ManagedBuffer {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(&owner, &token_identifier, token_nonce)
            .attributes;

        let mut starting_attributes = ManagedBuffer::new();
        starting_attributes = starting_attributes
            .clone()
            .concat(sc_format!("metadata:{}/", IPFS_CID));

        if attributes.copy_slice(0, starting_attributes.len()).unwrap() != starting_attributes {
            sc_panic!("Attributes do not start as expected.");
        }

        let mut colon_index = attributes.len() - 1;
        let mut colon = attributes.copy_slice(colon_index, 1).unwrap();

        while colon != b":" {
            colon_index -= 1;
            colon = attributes.copy_slice(colon_index, 1).unwrap();
        }

        attributes
            .copy_slice(colon_index + 1, attributes.len() - colon_index - 1)
            .unwrap()
    }
}
