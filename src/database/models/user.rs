use crate::{database::get_now_time, security};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, PgPool, Row};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserModel {
    id: Uuid,
    username: String,

    #[serde(skip_serializing)]
    password: Option<String>,

    profile_image: String,

    online: bool,
    last_online: NaiveDateTime,
    created_at: NaiveDateTime,
}

impl UserModel {
    pub fn default() -> Self {
        let now_time = get_now_time();

        Self {
            id: Uuid::new_v4(),
            username: String::new(),
            password: None,

            profile_image: String::new(),

            online: false,
            last_online: now_time,
            created_at: now_time,
        }
    }

    pub fn from_row(row: &PgRow, with_password: bool) -> Self {
        Self {
            id: row.try_get("id").expect("Cannot parse the user id."),
            username: row.try_get("username").expect("Cannot parse the username."),
            password: if with_password {
                Some(
                    row.try_get("password")
                        .expect("Cannot parse the user password."),
                )
            } else {
                None
            },

            profile_image: row
                .try_get("profile_image")
                .expect("Cannot parse the user profile image."),

            online: row
                .try_get("online")
                .expect("Cannot parse the user online status."),
            last_online: row
                .try_get("last_online")
                .expect("Cannot parse the user last online timestamp."),
            created_at: row
                .try_get("created_at")
                .expect("Cannot parse the user created at timestamp."),
        }
    }

    pub async fn from_id(client: &PgPool, id: &Uuid, with_password: bool) -> Result<Self, Error> {
        let mut query = String::from("SELECT id, username, ");
        if with_password {
            query.push_str("password, ");
        }
        query.push_str("profile_image, online, last_online, created_at FROM users WHERE id = $1");

        let result: PgRow = sqlx::query(&query).bind(id).fetch_one(client).await?;
        Ok(Self::from_row(&result, with_password))
    }

    pub async fn from_username(
        client: &PgPool,
        username: &str,
        with_password: bool,
    ) -> Result<Self, Error> {
        let mut query = String::from("SELECT id, username, ");
        if with_password {
            query.push_str("password, ");
        }
        query.push_str(
            "profile_image, online, last_online, created_at FROM users WHERE username = $1",
        );

        let result: PgRow = sqlx::query(&query).bind(username).fetch_one(client).await?;
        Ok(Self::from_row(&result, with_password))
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = String::from(username);
    }

    pub fn set_password(&mut self, password: &str) -> String {
        let password = security::encrypt_password(password);
        self.password = Some(password.clone());
        password
    }

    pub fn get_profile_image(&self) -> String {
        self.profile_image.clone()
    }

    pub fn is_online(&self) -> bool {
        self.online
    }

    pub fn set_online(&mut self, online: bool) {
        self.online = online;
    }

    pub fn get_last_online(&self) -> NaiveDateTime {
        self.last_online
    }

    pub fn set_last_online(&mut self, last_online: NaiveDateTime) {
        self.last_online = last_online;
    }

    pub fn get_created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn verify_password(&self, password: String) -> bool {
        if self.password.is_none() {
            panic!("The user password cannot be found.");
        }

        security::verify_password(&password, &self.password.clone().unwrap())
    }
}
