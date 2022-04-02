use crate::{api::base::Base, Email, model::client::Stytch};
use crate::otp::email::email::OtpEmail;

use super::{sms::sms::Sms, whatsapp::whatsapp::Whatsapp};

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

    pub fn whatsapp(&self, phone_number: String) -> Whatsapp {
        Whatsapp {
            phone_number,
            base: Base::new(&self.client),
            client: &self.client,
        }
    }

    pub fn email(&self,email:String)-> OtpEmail {
        OtpEmail {
            email,
            base:Base::new(&self.client),
            client: &self.client
        }
    }
}
