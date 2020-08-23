// Copyright (C) 2020 Peter Mezei
//
// This file is part of Gardenzilla.
//
// Gardenzilla is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Gardenzilla is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Gardenzilla.  If not, see <http://www.gnu.org/licenses/>.

use crate::check::*;
use crate::prelude::EmailServiceError::*;
use crate::prelude::*;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use std::env;

pub trait Email<'a> {
    fn send(&self) -> ServiceResult<()>;
    fn is_dummy(&mut self) -> &mut Self;
}

pub struct EmailData<'a> {
    to: &'a str,
    subject: &'a str,
    body: &'a str,
    is_dummy: bool,
}

pub fn new<'a>(to: &'a str, subject: &'a str, body: &'a str) -> impl Email<'a> {
    EmailData {
        to,
        subject,
        body,
        is_dummy: false,
    }
}

impl<'a> Email<'a> for EmailData<'a> {
    // Use it for test email
    // You can test everything but the SMTP connection.
    fn is_dummy(&mut self) -> &mut Self {
        self.is_dummy = true;
        self
    }
    // Try send email. Return ServiceResult<()>
    // TODO: Implement some kind of email POOL to manage
    // connection fail and email stmp temp EmailServiceErrors.
    fn send(&self) -> ServiceResult<()> {
        // Validate TO email address
        check_email(self.to)?;
        // Check subject and body
        if self.subject.is_empty() || self.body.is_empty() {
            return Err(InternalError("Empty subject or body.".into()));
        }
        if self.is_dummy {
            return Ok(());
        }
        // Lets build it up
        let email: lettre_email::Email = lettre_email::Email::builder()
            .to(self.to)
            .from(env::var("SMTP_FROM_EMAIL")?)
            .subject(self.subject)
            .text(self.body)
            .build()?;

        // Open a remote connection to SMTP server
        SmtpClient::new_simple(&env::var("SMTP_SERVER_DOMAIN")?)?
            .credentials(Credentials::new(
                env::var("SMTP_USERNAME")?,
                env::var("SMTP_PASSWORD")?,
            ))
            .transport()
            .send(email.into())?;

        Ok(())
    }
}

impl From<lettre_email::error::Error> for EmailServiceError {
    fn from(error: lettre_email::error::Error) -> Self {
        InternalError(format!("{}", error))
    }
}

impl From<env::VarError> for EmailServiceError {
    fn from(error: env::VarError) -> Self {
        InternalError(format!("{}", error))
    }
}

impl From<lettre::smtp::error::Error> for EmailServiceError {
    fn from(error: lettre::smtp::error::Error) -> Self {
        InternalError(format!("{}", error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_send_email() {
        new("mezeipetister@gmail.com", "Subject", "Body")
            .is_dummy()
            .send()
            .unwrap();
    }
}
