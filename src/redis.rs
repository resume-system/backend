use lazy_static::lazy_static;
use rand::Rng;
use redis::Commands;
use serde::{Deserialize, Serialize};
use crate::CONFIG;
use crate::Result;

lazy_static! {
    pub static ref REDIS: redis::Client = {
        redis::Client::open(CONFIG.redis.url.as_str())
            .expect("Failed to open redis client.")
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Captcha {
    pub text: String,
    #[serde(with = "crate::db::naive_datetime_format")]
    pub expiration: chrono::NaiveDateTime,
}

impl Default for Captcha {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        // 6
        let text = rng.gen_range(100_000..100_000_1);
        let text = format!("{:06}", text);

        let cur_time = chrono::Local::now().naive_local();
        let expiration = cur_time + chrono::Duration::seconds(5);

        Self {
            text,
            expiration,
        }
    }
}

impl Captcha {
    fn new_with_text(text: String) -> Self {
        Self {
            text,
            ..Self::default()
        }
    }

    fn from_redis(key: impl AsRef<str>) -> Result<Option<Self>> {
        let mut con = REDIS.get_connection()?;
        let text: Option<String> = con.get(key.as_ref())?;

        Ok(
            text.map(|e| serde_json::from_str::<Captcha>(&e)).transpose()?
        )
    }

    fn save_redis(&self, key: impl AsRef<str>) -> Result<()> {
        let mut con = REDIS.get_connection()?;

        Ok(
            con.set(key.as_ref(), serde_json::to_string(self)?)?
        )
    }

    pub fn get_captcha(key: impl AsRef<str>) -> Result<Option<Captcha>> {
        Self::from_redis(key)
    }

    pub fn new_captcha(id: impl AsRef<str>) -> Result<Captcha> {
        let mut con = REDIS.get_connection()?;
        let flag: bool = con.exists(id.as_ref())?;

        let captcha = match flag {
            true => {
                let mut captcha = Self::from_redis(id.as_ref())?.unwrap();
                let new_captcha =
                    Self::new_with_text(std::mem::take(&mut captcha.text));

                new_captcha.save_redis(id)?;
                new_captcha
            }
            false => {
                let captcha = Self::default();

                captcha.save_redis(id)?;
                captcha
            }
        };

        Ok(captcha)
    }
}