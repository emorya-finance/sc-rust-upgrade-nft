use multiversx_sc::{derive_imports::*, imports::*};

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, ManagedVecItem, Debug, Clone)]
pub struct UserNft<M: ManagedTypeApi> {
    pub identifier: TokenIdentifier<M>,
    pub nonce: u64,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, ManagedVecItem, Debug, Clone)]
pub struct UserRetrieve {
    pub counter: u64,
    pub unlocking: bool,
}

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getIsScPaused)]
    #[storage_mapper("isScPaused")]
    fn is_sc_paused(&self) -> SingleValueMapper<bool>;

    #[view(getAllowedAddresses)]
    #[storage_mapper("allowedAddresses")]
    fn allowed_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getNftOwnerAddress)]
    #[storage_mapper("nftOwnerAddress")]
    fn nft_owner_address(
        &self,
        nft_token: &TokenIdentifier,
        nft_nonce: u64,
    ) -> SingleValueMapper<ManagedAddress>;

    #[view(getNftFromAddress)]
    #[storage_mapper("nftFromAddress")]
    fn nft_from_address(&self, user: &ManagedAddress) -> SingleValueMapper<UserNft<Self::Api>>;

    #[view(getNftRetrieveFromAddress)]
    #[storage_mapper("nftRetrieveFromAddress")]
    fn nft_retrieve_from_address(
        &self,
        user: &ManagedAddress,
    ) -> SingleValueMapper<UserNft<Self::Api>>;

    #[view(getUserRetrieveEpoch)]
    #[storage_mapper("userRetrieveEpoch")]
    fn user_retrieve_epoch(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getUnbondingPeriod)]
    #[storage_mapper("unbondingPeriod")]
    fn unbonding_period(&self) -> SingleValueMapper<u64>;

    #[view(getBlockedUsers)]
    #[storage_mapper("blockedUsers")]
    fn blocked_users(&self, address: &ManagedAddress) -> SingleValueMapper<bool>;
}
