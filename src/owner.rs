multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait OwnerModule:
    crate::storage::StorageModule + crate::private::PrivateModule + crate::views::ViewsModule
{
    #[only_owner]
    #[endpoint(pauseSc)]
    fn pause_sc(&self) {
        self.is_sc_paused().set(true);
    }

    #[only_owner]
    #[endpoint(resumeSc)]
    fn resume_sc(&self) {
        self.is_sc_paused().set(false);
    }

    #[only_owner]
    #[endpoint(addAllowedAddresses)]
    fn add_allowed_addresses(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        for address in addresses.into_iter() {
            self.allowed_addresses().insert(address);
        }
    }
}
