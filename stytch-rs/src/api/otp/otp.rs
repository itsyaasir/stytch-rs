use crate::{api::base::Base, model::client::Stytch};

use super::sms::sms::Sms;

#[derive(Debug, Clone)]
pub struct Otp<'a> {
    pub client: &'a Stytch,
}

impl<'a> Otp<'a> {
    pub fn new(client: &'a Stytch) -> Self {
        Self { client }
    }

    pub fn sms(&self, phone_number: String) -> Sms {
        Sms {
            phone_number,
            base: Base::new(&self.client),
            client: &self.client,
        }
    }
}