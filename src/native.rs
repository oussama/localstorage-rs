use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::Error;
use std::io::prelude::*;
//use errors::*;
use std::fs::*;

use serde_json;

pub struct LocalStorage<T>
where
    T: Serialize + DeserializeOwned,
{
    val: Option<T>,
    name: String,
    file: File,
}

use std::thread;

impl<T> LocalStorage<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new<S: Into<String>>(name: S) -> Result<LocalStorage<T>, Error> {
        let named = name.into();
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .append(false)
            .open("/data/ls-".to_string() + &named)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(LocalStorage {
            val: serde_json::from_str(&contents).ok(),
            name: named,
            file,
        })
    }

    pub fn set(&mut self, val: T) -> Result<(), Error> {
        let json = serde_json::to_string(&val)?;
        self.val = Some(val);
        self.file.set_len(0)?;
        self.file.write_all(json.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }

    pub fn get(&self) -> &Option<T> {
        &self.val
    }

    pub fn get_mut(&mut self) -> &Option<T> {
        &mut self.val
    }

    pub fn clear(&mut self) -> Result<(),Error> {
        self.val = None;
        self.file.set_len(0)?;
        Ok(())
    }
}
