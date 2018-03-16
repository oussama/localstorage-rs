use stdweb::*;
use stdweb::unstable::TryInto;

use std::io::Error;

use stdweb::web::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use serde_json;

pub struct LocalStorage<T>
where
    T: Serialize + DeserializeOwned,
{
    val: Option<T>,
    name: String,
    storage: Storage,
}


impl<T> LocalStorage<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new<S: Into<String>>(name: S) -> Result<LocalStorage<T>, Error> {
        let named = name.into();
        let mut storage = window().local_storage();

        Ok(LocalStorage {
            val: None,// storage.get(&named).map(|val| serde_json::from_str(&val).ok()),
            name: named,
            storage,
        })
    }

    pub fn set(&mut self, val: T) -> Result<(), Error> {
        let json:String = serde_json::to_string(&val)?;
        self.val = Some(val);
        self.storage.insert(&self.name,&json);
        Ok(())
    }

    pub fn get(&self) -> &Option<T> {
        &self.val
    }

    pub fn clear(&mut self) -> Result<(),Error> {
        self.val = None;
        self.storage.remove(&self.name);
        Ok(())
    }
}
