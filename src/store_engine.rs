use anyhow::Result;
use redb::ReadableTable;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path, time::SystemTime};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
    pub server: String,
    pub nickname: String,
    pub strategy: String,
    pub date: SystemTime,
}

const USER_TABLE: redb::TableDefinition<&str, &[u8]> = redb::TableDefinition::new("userdata");

pub struct StoreEngine {
    db: redb::Database,
    users: HashMap<Uuid, UserInfo>,
}

impl StoreEngine {
    pub fn create<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let db = redb::Database::create(path)?;
        let users = Self::get_users_from_db(&db)?;
        Ok(Self { db, users })
    }

    pub fn get_user(&self, uuid: &uuid::Uuid) -> Option<UserInfo> {
        match self.users.get(&uuid) {
            Some(user) => Some(user.to_owned()),
            None => None,
        }
    }

    pub fn get_user_from_db(db: &redb::Database, uuid: &uuid::Uuid) -> Result<UserInfo> {
        let read_context = db.begin_read()?;
        let user = {
            let table = read_context.open_table(USER_TABLE)?;
            let bytes = table
                .get(uuid.to_string().as_str())?
                .ok_or_else(|| anyhow::anyhow!("Failed to get value"))?;
            let user: UserInfo = bincode::deserialize(bytes.value())?;
            user
        };

        Ok(user)
    }

    pub fn get_users(&self) -> std::collections::hash_map::Iter<Uuid, UserInfo> {
        self.users.iter()
    }

    pub fn get_users_from_db(db: &redb::Database) -> Result<HashMap<Uuid, UserInfo>> {
        let read_context = db.begin_read()?;
        let map = {
            let table = read_context.open_table(USER_TABLE)?;
            let map = table
                .iter()?
                .map(|(k, v)| {
                    let uuid = Uuid::parse_str(k.value()).expect("redb failed to work");
                    let user: UserInfo =
                        bincode::deserialize(v.value()).expect("redb failed to work");
                    (uuid, user)
                })
                .collect();
            map
        };
        Ok(map)
    }

    pub fn insert(&mut self, uuid: &uuid::Uuid, user: &UserInfo) -> Result<uuid::Uuid> {
        let write_context = self.db.begin_write()?;
        {
            let mut table = write_context.open_table(USER_TABLE)?;
            let key = uuid.to_string();
            let value = bincode::serialize(user)?;
            table.insert(key.as_str(), value.as_slice())?;
        }
        write_context.commit()?;
        self.users.insert(uuid.to_owned(), user.to_owned());
        Ok(uuid.to_owned())
    }

    pub fn insert_many<'a, I>(&mut self, items: I) -> Result<()>
    where
        I: IntoIterator<Item = (&'a Uuid, &'a UserInfo)>,
    {
        let mut new_userdata = HashMap::new();
        let write_context = self.db.begin_write()?;
        {
            let mut table = write_context.open_table(USER_TABLE)?;
            for (uuid, user) in items {
                let key = uuid.to_string();
                let value = bincode::serialize(user)?;
                table.insert(key.as_str(), value.as_slice())?;
                new_userdata.insert(uuid.to_owned(), user.to_owned());
            }
        }
        write_context.commit()?;
        self.users.extend(new_userdata);
        Ok(())
    }
}
