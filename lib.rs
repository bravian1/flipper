#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Flipper {
        message: String,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_message: String) -> Self {
            Self { message: init_message }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(String::new())
        }

        #[ink(message)]
        pub fn set_message(&mut self, new_message: String) {
            self.message = new_message;
        }

        #[ink(message)]
        pub fn get_message(&self) -> String {
            self.message.clone()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get_message(), String::new());
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new("Hello".to_string());
            assert_eq!(flipper.get_message(), "Hello");
            flipper.set_message("World".to_string());
            assert_eq!(flipper.get_message(), "World");
        }
    }


    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = FlipperRef::default();

            // When
            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Flipper>();

            // Then
            let get = call_builder.get_message();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert_eq!(get_result.return_value(), String::new());

            Ok(())
        }

        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = FlipperRef::new("Hello".to_string());
            let contract = client
                .instantiate("flipper", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Flipper>();

            let get = call_builder.get_message();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert_eq!(get_result.return_value(), "Hello");

            // When
            let set_msg = call_builder.set_message("World".to_string());
            let _set_result = client
                .call(&ink_e2e::bob(), &set_msg)
                .submit()
                .await
                .expect("set_message failed");

            // Then
            let get = call_builder.get_message();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert_eq!(get_result.return_value(), "World");

            Ok(())
        }
    }
}

