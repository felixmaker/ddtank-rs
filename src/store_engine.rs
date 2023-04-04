use anyhow::Result;
use redb::ReadableTable;
use std::{collections::HashMap, path::Path};
use uuid::Uuid;

use crate::userinfo::UserInfo;

const USER_TABLE: redb::TableDefinition<&[u8; 16], &[u8]> = redb::TableDefinition::new("userdata");

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
        let users = Self::get_users_from_db(&db).unwrap_or(HashMap::new());
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
                .get(uuid.as_bytes())?
                .ok_or_else(|| anyhow::anyhow!("Failed to get value"))?;
            let user: UserInfo = bincode::deserialize(bytes.value())?;
            user
        };

        Ok(user)
    }

    pub fn get_all_users(&self) -> &std::collections::HashMap<Uuid, UserInfo> {
        &self.users
    }

    pub fn users(&self) -> std::collections::hash_map::Iter<Uuid, UserInfo> {
        self.users.iter()
    }

    pub fn get_users_from_db(db: &redb::Database) -> Result<HashMap<Uuid, UserInfo>> {
        let read_context = db.begin_read()?;
        let map = {
            let table = read_context.open_table(USER_TABLE)?;
            let map = table
                .iter()?
                .map(|(k, v)| {
                    let uuid = Uuid::from_bytes(k.value().to_owned());
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
            let key = uuid.as_bytes();
            let value = bincode::serialize(user)?;
            table.insert(key, value.as_slice())?;
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
                let key = uuid.as_bytes();
                let value = bincode::serialize(user)?;
                table.insert(key, value.as_slice())?;
                new_userdata.insert(uuid.to_owned(), user.to_owned());
            }
        }
        write_context.commit()?;
        self.users.extend(new_userdata);
        Ok(())
    }

    pub fn remove(&mut self, uuid: &uuid::Uuid) -> Result<uuid::Uuid> {
        let write_context = self.db.begin_write()?;
        {
            let mut table = write_context.open_table(USER_TABLE)?;
            let key = uuid.as_bytes();
            table.remove(key)?;
        }
        write_context.commit()?;
        self.users.remove(uuid);
        Ok(uuid.to_owned())
    }

    pub fn remove_many<'a, I>(&'a mut self, uuids: I) -> Result<()>
    where
        I: IntoIterator<Item = &'a Uuid>,
    {
        let mut users = self.users.clone();
        let write_context = self.db.begin_write()?;
        {
            let mut table = write_context.open_table(USER_TABLE)?;
            for uuid in uuids {
                table.remove(uuid.as_bytes())?;
                users.remove(uuid);
            }
        }
        write_context.commit()?;
        self.users = users;
        Ok(())
    }
}
