#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod paymentchannel {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Paymentchannel {
        /// Stores a single `bool` value on the storage.
        sender: AccountId,
        recipient: AccountId,
        expiration: Option<Timestamp>,
        withdraw: Balance,
        close_duration: Timestamp,
    }
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        CallerIsNotSender,
        CallerIsNotRecipient,
        AmountIsLessThanWithdraw,
        TransferFailed,
        NotYetExpired,
        IvalidSignature,
    }
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(event)]
    pub struct SenderCloseStarted {
      
        expiration: Timestamp,
       close_duration: Timestamp,
    }
    impl Paymentchannel {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(recipient: AccountId, close_duration: Timestamp) -> Self {
            Self { sender: Self::env().caller(), 
                recipient: recipient, 
                expiration: None, 
                withdraw: 0, 
                close_duration: close_duration 
            }
        }
        #[ink(message)]
        pub fn close(&mut self, amount: Balance, signature: [u8; 65]) -> Result<()>{
            self.close_inner(amount, signature)?;
            self.env().terminate_contract(self.sender);
        }
        fn close_inner(&mut self, amount: Balance, signature: [u8; 65]) -> Result<()>{
            if self.env().caller() != self.recipient{
                return Err(Error::CallerIsNotRecipient);
            }
            if amount < self.withdraw{
                return Err(Error::AmountIsLessThanWithdraw);
            }
            if self.is_signature_valid(amount, signature){
                return Err(Error::IvalidSignature)
            }
            self.env().transfer(self.recipient, amount - self.withdraw).map_err(|_| Error::TransferFailed)?;
            Ok(())
        }
        #[ink(message)]
        pub fn start_sender_close(&mut self) -> Result<()>{
            if self.env().caller() != self.sender{
                return Err(Error::CallerIsNotSender);
            }
            let now = self.env().block_timestamp();
            let expiration = now + self.close_duration;

            self.env().emit_event(SenderCloseStarted {
                expiration,
                close_duration: self.close_duration,
            });
            self.expiration = Some(expiration);
            Ok(())
        }

        #[ink(message)]
        pub fn claim_timeout(&mut self) -> Result<()>{
            match self.expiration {
                Some(expiration) => {
                   let now = self.env().block_timestamp();
                   if now < expiration {
                       return Err(Error::NotYetExpired);
                   }
                   self.env().terminate_contract(self.sender);
                }
                None => Err(Error::NotYetExpired),
            }
        }
        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance,signature: [u8; 65]) -> Result<()>{
            if self.env().caller() != self.sender{
                return Err(Error::CallerIsNotSender);
            }
            if self.expiration.is_none(){
                return Err(Error::NotYetExpired);
            }
            if self.is_signature_valid(self.withdraw, signature){
                return Err(Error::IvalidSignature)
            }
            let amount_to_withdraw = amount - self.withdraw;
            self.withdraw += amount_to_withdraw;
            self.env().transfer(self.recipient, amount_to_withdraw).map_err(|_| Error::TransferFailed)?;
            Ok(())
        }
        #[ink(message)]
        pub fn get_sender(&self) -> AccountId{
            self.sender
        }
        #[ink(message)]
        pub fn get_recipient(&self) -> AccountId{
            self.recipient
        }
        #[ink(message)]
        pub fn get_expiration(&self) -> Option<Timestamp>{
            self.expiration
        }
        #[ink(message)]
        pub fn get_withdraw(&self) -> Balance{
            self.withdraw
        }
        #[ink(message)]
        pub fn get_close_duration(&self) -> Timestamp{
            self.close_duration
        }
        #[ink(message)]
        pub fn get_balance(&self) -> Balance{
            self.env().balance()
        }
     
    }
    #[ink(impl)]
    impl Paymentchannel {
        fn is_signature_valid(&self, amount: Balance, signature: [u8; 65]) -> bool{
           let encodable = (self.env().account_id(), amount);
           let mut message = <ink_env::hash::Sha2x256 as ink_env::hash::HashOutput>::Type::default();
           ink_env::hash_encoded::<ink_env::hash::Sha2x256, _>(&encodable, &mut message);

           let mut pub_key = [0; 33];
           ink_env::ecdsa_recover(&signature, &message, &mut pub_key).expect("reover failed");

           let mut signature_account_id = [0; 32];
           <ink_env::hash::Blake2x256 as ink_env::hash::CryptoHash>::hash(&pub_key, &mut signature_account_id);

           self.recipient == signature_account_id.into()
            
        }
    }
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        // Imports all the definitions from the outer scope so we can use them here.
        // use super::*;

        // /// Imports `ink_lang` so we can use `#[ink::test]`.
        // use ink_lang as ink;

        // /// We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let paymentchannel = Paymentchannel::default();
        //     assert_eq!(paymentchannel.get(), false);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut paymentchannel = Paymentchannel::new(false);
        //     assert_eq!(paymentchannel.get(), false);
        //     paymentchannel.flip();
        //     assert_eq!(paymentchannel.get(), true);
        // }
    }
}
