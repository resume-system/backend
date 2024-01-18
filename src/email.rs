use actix_web::error::UrlencodedError::ContentType;
use lazy_static::lazy_static;
use lettre::message::MessageBuilder;
use lettre::{SmtpTransport, Transport};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::PoolConfig;
use native_tls::TlsConnector;
use crate::{CONFIG, Result};

lazy_static! {
    pub static ref EMAIL: Email = {
        Email {
            creds: Credentials::new(
                CONFIG.email.from.clone(),
                CONFIG.email.password.clone(),
            ),
            from: CONFIG.email.from.clone(),
            server: CONFIG.email.server.clone(),
            port: CONFIG.email.port.clone(),
        }
    };
}

pub struct Email {
    creds: Credentials,
    from: String,
    server: String,
    port: u16,
}

impl Email {
    pub fn send(&self, target: impl AsRef<str>, content: impl AsRef<str>) -> Result<()> {
        let mailer = SmtpTransport::relay(self.server.as_str())?
            .credentials(self.creds.clone())
            .port(self.port)
            .authentication(vec![Mechanism::Plain])
            .pool_config(PoolConfig::new().max_size(20))
            .build();

        let msg = MessageBuilder::new()
            .from(self.from.clone().parse()?)
            .to(target.as_ref().clone().parse()?)
            .subject("验证码")
            .header(lettre::message::header::ContentType::TEXT_PLAIN)
            .body(String::from(content.as_ref().to_string()))?;

        mailer.send(&msg)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::email::EMAIL;

    #[test]
    fn test_send() {
        EMAIL.send("1049617132@qq.com", "111??").unwrap();
    }
}