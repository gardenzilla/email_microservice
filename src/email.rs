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

use crate::prelude::EmailError::*;
use crate::prelude::*;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use std::env;

pub fn check_email(address: &str) -> EmailResult<()> {
    if !address.contains('@') {
        return Err(WrongEmailAddress(
            "Nem megfelelő email formátum. Hiányzó karakter: @.".into(),
        ));
    }
    if !address.contains('.') {
        return Err(WrongEmailAddress(
            "Nem megfelelő email formátum. Hiányzó karakter: (pont).".into(),
        ));
    }
    if address.len() < 4 {
        return Err(WrongEmailAddress(
            "Nem megfelelő email formátum. Túl rövid.".into(),
        ));
    }
    Ok(())
}

impl crate::Email {
    pub fn try_send(&self) -> EmailResult<()> {
        // Validate TO email address
        check_email(&*self.to)?;
        // Check subject and body
        if self.subject.is_empty() || self.body.is_empty() {
            return Err(InternalError("Empty subject or body.".into()));
        }
        // Lets build it up
        let email: lettre_email::Email = lettre_email::Email::builder()
            .to(self.to.as_str())
            .from(env::var("SMTP_FROM_EMAIL")?)
            .subject(self.subject.as_str())
            .text(self.body.as_str())
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

impl From<lettre_email::error::Error> for EmailError {
    fn from(error: lettre_email::error::Error) -> Self {
        InternalError(format!("Email internal error: {}", error))
    }
}

impl From<env::VarError> for EmailError {
    fn from(error: env::VarError) -> Self {
        InternalError(format!("Email internal error: {}", error))
    }
}

impl From<lettre::smtp::error::Error> for EmailError {
    fn from(error: lettre::smtp::error::Error) -> Self {
        InternalError(format!("Email internal error: {}", error))
    }
}
