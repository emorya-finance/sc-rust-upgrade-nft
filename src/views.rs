use crate::{
    constants::{NFT_IDENTIFIER, NFT_IDENTIFIER_INVESTORS, TAGS},
    managedbufferutils::ManagedBufferUtils,
    storage::UserRetrieve,
};

type NftInfo<M> = MultiValue3<TokenIdentifier<M>, u64, u64>;
type UserInfo<M> = MultiValue4<NftInfo<M>, NftInfo<M>, u64, bool>;
type CustomNftInfo<M> = MultiValue3<ManagedAddress<M>, bool, u64>;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ViewsModule: crate::storage::StorageModule {
    #[view(getTags)]
    fn get_tags(&self) -> ManagedBuffer {
        ManagedBuffer::new_from_bytes(TAGS)
    }

    #[view(getNftIdentifier)]
    fn get_nft_identifier(&self) -> TokenIdentifier {
        TokenIdentifier::from_esdt_bytes(NFT_IDENTIFIER)
    }

    #[view(getNftIdentifierInvestors)]
    fn get_nft_identifier_investors(&self) -> TokenIdentifier {
        TokenIdentifier::from_esdt_bytes(NFT_IDENTIFIER_INVESTORS)
    }

    #[view(getNftAttributes)]
    fn get_nft_attributes(
        &self,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
    ) -> ManagedBuffer {
        self.blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &token_identifier,
                token_nonce,
            )
            .attributes
    }

    #[view(getNftUris)]
    fn get_nft_uris(
        &self,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
    ) -> ManagedVec<ManagedBuffer> {
        self.blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &token_identifier,
                token_nonce,
            )
            .uris
    }

    #[view(getNftUriJson)]
    fn get_nft_uri_json(
        &self,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
    ) -> ManagedBuffer {
        let uris = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &token_identifier,
                token_nonce,
            )
            .uris;

        let link = uris.get(1).clone_value();
        link.copy_slice(8, link.len() - 8).unwrap()
    }

    #[view(getNftAttributesLevelBeforeUpgrade)]
    fn get_nft_attributes_level_before_upgrade(
        &self,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
    ) -> ManagedBuffer {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &token_identifier,
                token_nonce,
            )
            .attributes;

        if attributes.copy_slice(0, 6).unwrap() != b"level:" {
            sc_panic!("Attributes do not start as expected.");
        }

        let mut semicolon_index = 7;
        let mut semicolon = attributes.copy_slice(semicolon_index, 1).unwrap();
        while semicolon != b"," {
            semicolon_index += 1;
            semicolon = attributes.copy_slice(semicolon_index, 1).unwrap();
        }
        // 0 1 2 3 4 5 6 7
        // l e v e l : 5 ;
        attributes.copy_slice(6, semicolon_index - 6).unwrap()
    }

    #[view(getNftAttributesLevelAfterUpgrade)]
    fn get_nft_attributes_level_after_upgrade(
        &self,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
    ) -> ManagedBuffer {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &token_identifier,
                token_nonce,
            )
            .attributes;

        let uri_json = self.get_nft_uri_json(token_identifier, token_nonce);

        let mut starting_attributes = ManagedBuffer::new();
        starting_attributes = starting_attributes
            .clone()
            .concat(sc_format!("metadata:{}", uri_json));

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

    #[view(getNftLevel)]
    fn get_nft_level(&self, token_identifier: TokenIdentifier, token_nonce: u64) -> ManagedBuffer {
        let attributes = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &token_identifier,
                token_nonce,
            )
            .attributes;

        if attributes.copy_slice(0, 6).unwrap() == b"level:" {
            self.get_nft_attributes_level_before_upgrade(token_identifier, token_nonce)
        } else {
            self.get_nft_attributes_level_after_upgrade(token_identifier, token_nonce)
        }
    }

    #[view(getNftInfoBeforeUpgrade)]
    fn get_nft_from_address_before(&self, user: ManagedAddress) -> NftInfo<Self::Api> {
        let nft_token = self.nft_from_address(&user).get();

        let level = self
            .get_nft_attributes_level_before_upgrade(nft_token.identifier.clone(), nft_token.nonce)
            .ascii_to_u64()
            .unwrap_or(1);

        NftInfo::from((nft_token.identifier, nft_token.nonce, level))
    }

    #[view(getNftInfoAfterUpgrade)]
    fn get_nft_from_address(&self, user: ManagedAddress) -> NftInfo<Self::Api> {
        if self.nft_from_address(&user).is_empty() {
            return NftInfo::from((TokenIdentifier::from_esdt_bytes(b""), 0, 0));
        }

        let nft_token = self.nft_from_address(&user).get();

        let level = self
            .get_nft_attributes_level_after_upgrade(nft_token.identifier.clone(), nft_token.nonce)
            .ascii_to_u64()
            .unwrap_or(1);

        NftInfo::from((nft_token.identifier, nft_token.nonce, level))
    }

    #[view(getNftInRetrieveByAddress)]
    fn get_nft_retrieve_from_address(&self, user: ManagedAddress) -> NftInfo<Self::Api> {
        if self.nft_retrieve_from_address(&user).is_empty() {
            return NftInfo::from((TokenIdentifier::from_esdt_bytes(b""), 0, 0));
        }
        let nft = self.nft_retrieve_from_address(&user).get();

        let level = self
            .get_nft_attributes_level_after_upgrade(nft.identifier.clone(), nft.nonce)
            .ascii_to_u64()
            .unwrap_or(1);

        NftInfo::from((nft.identifier, nft.nonce, level))
    }

    #[view(getNftNonce)]
    fn get_nft_nonce(&self, user: ManagedAddress) -> u64 {
        let nft_token = self.nft_from_address(&user).get();
        nft_token.nonce
    }

    #[view(getNftLevelByAddress)]
    fn get_nft_level_by_address(&self, user: ManagedAddress) -> u64 {
        self.get_nft_from_address(user).into_tuple().2
    }

    /// Boolean is a number -> 01 True , {empty}/"" False
    #[view(getRemainingUnbondingTime)]
    fn get_remaining_unbonding_time(&self, user: ManagedAddress) -> UserRetrieve {
        if self.unbonding_period().get()
            >= (self.blockchain().get_block_epoch() - self.user_retrieve_epoch(&user).get())
        {
            UserRetrieve {
                counter: self.unbonding_period().get()
                    - (self.blockchain().get_block_epoch() - self.user_retrieve_epoch(&user).get())
                    + 1,
                unlocking: !self.user_retrieve_epoch(&user).is_empty(),
            }
        } else {
            UserRetrieve {
                counter: 0,
                unlocking: !self.user_retrieve_epoch(&user).is_empty(),
            }
        }
    }

    /// Returns:
    /// - User Active NFT (Identifier, Nonce , Level)
    /// - User in Retrieve NFT (Identifier, Nonce, Level)
    /// - Unbounding Time
    /// - Can Claim
    #[view(getUserInfo)]
    fn ger_user_info(&self, user: ManagedAddress) -> UserInfo<Self::Api> {
        let user_retrieve_info = self.get_remaining_unbonding_time(user.clone());
        UserInfo::from((
            NftInfo::from(self.get_nft_from_address(user.clone())),
            NftInfo::from(self.get_nft_retrieve_from_address(user)),
            user_retrieve_info.counter,
            user_retrieve_info.unlocking,
        ))
    }

    /// Returns:
    /// - User Address
    /// - Is in Retrieve
    /// - Unbounding Time
    ///
    /// Takes as input the token identifier and nonce of the NFT.
    #[view(getCustomNftInfo)]
    fn get_custom_nft_info(
        &self,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
    ) -> CustomNftInfo<Self::Api> {
        let user = self.nft_owner_address(&token_identifier, token_nonce).get();
        let nft_in_retrieve = self.nft_retrieve_from_address(&user.clone()).get();

        if token_identifier == nft_in_retrieve.identifier && token_nonce == nft_in_retrieve.nonce {
            let nft_in_rtetrieve = self.get_remaining_unbonding_time(user.clone());
            return CustomNftInfo::from((
                user.clone(),
                nft_in_rtetrieve.unlocking,
                nft_in_rtetrieve.counter,
            ));
        } else {
            CustomNftInfo::from((user.clone(), false, 0))
        }
    }
}
