multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait PrivateModule: crate::storage::StorageModule {
    fn require_not_paused(&self) {
        require!(
            !self.is_sc_paused().get(),
            "This smart contract is currently paused. All actions are haulted."
        );
    }
}
