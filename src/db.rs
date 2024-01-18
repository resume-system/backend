use std::sync::Mutex;
use diesel::{Connection, MysqlConnection, RunQueryDsl, table};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::{CONFIG, DBConfig};
use crate::Result;

pub struct DBEngine {
    db: MysqlConnection,
}

lazy_static! {
    pub static ref ENGINE: Mutex<DBEngine> = {
        let db_url = format!("mysql://{}:{}@localhost/{}",
            CONFIG.db.username, CONFIG.db.password, CONFIG.db.db_name);

        Mutex::new(DBEngine {
            db: MysqlConnection::establish(&db_url)
                    .expect(&format!("Error connecting to {}", db_url))
        })
    };
}

table! {
    administrator (administrator_id) {
        administrator_id -> SmallInt,
        username -> Varchar,
        password -> Varchar,
    }
}

table! {
    company (company_id) {
        company_id -> TinyInt,
        company_name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        state -> Char,
    }
}

table! {
    r#match (resume_id, position_id) {
        resume_id -> SmallInt,
        position_id -> SmallInt,
        match_degree -> Double,
    }
}

table! {
    position (position_id, company_id) {
        position_id -> SmallInt,
        title -> Varchar,
        company -> Varchar,
        sex -> Char,
        age_min -> TinyInt,
        age_max -> TinyInt,
        school_lev -> Varchar,
        education -> Char,
        english_lev -> Varchar,
        subject -> Varchar,
        graduation -> Datetime,
        word_experience -> Varchar,
        skill -> Varchar,
        other_clain -> Varchar,
        address -> Varchar,
        salary -> Varchar,
        welfare -> Varchar,
        other_welfare -> Varchar,
        hrname -> Varchar,
        company_id -> TinyInt,
    }
}

table! {
    resume(user_id) {
        user_id -> SmallInt,
        name -> Varchar,
        age -> TinyInt,
        sex -> Char,
        email -> Varchar,
        school -> Varchar,
        education -> Char,
        subject -> Varchar,
        graduation_data -> Date,
        english_lev -> Varchar,
        skill -> Varchar,
        project_experience -> Varchar,
        work_experience -> Varchar,
        position_expect -> Varchar,
        salary_expect -> Varchar,
        address_expect -> Varchar,
    }
}

table! {
    user (user_id) {
        user_id -> SmallInt,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        state -> Char,
    }
}

#[derive(Insertable, Default)]
#[table_name = "administrator"]
pub struct Administrator {
    pub administrator_id: i16,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Default)]
#[table_name = "company"]
pub struct Company {
    pub company_id: i8,
    pub company_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub state: String,
}

#[derive(Insertable)]
#[table_name = "r#match"]
pub struct Match {
    pub resume_id: i16,
    pub position_id: i16,
    pub match_degree: f64,
}

#[derive(Insertable, Deserialize, Serialize, Default)]
#[table_name = "position"]
pub struct Position {
    pub position_id: i16,
    pub title: String,
    pub company: String,
    pub sex: String,
    pub age_min: i8,
    pub age_max: i8,
    pub school_lev: String,
    pub education: String,
    pub english_lev: String,
    pub subject: String,
    #[serde(with = "naive_datetime_format")]
    pub graduation: chrono::NaiveDateTime,
    pub word_experience: String,
    pub skill: String,
    pub other_clain: String,
    pub address: String,
    pub salary: String,
    pub welfare: String,
    pub other_welfare: String,
    pub hrname: String,
    pub company_id: i8,
}

#[derive(Insertable, Serialize, Deserialize, Default)]
#[table_name = "resume"]
pub struct Resume {
    pub user_id: i16,
    pub name: String,
    pub age: i8,
    pub sex: String,
    pub email: String,
    pub school: String,
    pub education: String,
    pub subject: String,
    #[serde(with = "naive_date_format")]
    pub graduation_data: chrono::NaiveDate,
    pub english_lev: String,
    pub skill: String,
    pub project_experience: String,
    pub work_experience: String,
    pub position_expect: String,
    pub salary_expect: String,
    pub address_expect: String,
}

#[derive(Insertable, Serialize, Deserialize, Default)]
#[table_name = "user"]
pub struct User {
    pub user_id: i16,
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub state: String,
}

pub mod naive_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod naive_datetime_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", datetime.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub fn register(info: User) -> Result<()> {
    diesel::insert_into(user::table)
        .values(&info)
        .execute(&mut ENGINE.lock().unwrap().db)?;

    Ok(())
}