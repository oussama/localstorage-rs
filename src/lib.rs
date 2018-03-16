extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

#[cfg(target_arch = "wasm32")]
mod stdw;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(test)]
mod tests {



    #[cfg(not(target_arch = "wasm32"))]
    use native::LocalStorage;
    #[cfg(target_arch = "wasm32")]
    use stdw;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Token(pub String);

    #[test]
    fn it_works() {
        {
            let mut ls = LocalStorage::<Token>::new("token").unwrap();
            ls.clear();
        }
        {
            let mut ls = LocalStorage::<Token>::new("token").unwrap();
            assert!(ls.get().is_none());
            ls.set(Token("Hello".to_string()));
            if let &Some(ref val) = ls.get() {
                assert_eq!(val.0, "Hello".to_string());
            } else {
                panic!("1. val not found");
            }
        }
        {
            let mut ls = LocalStorage::<Token>::new("token").unwrap();
            if let &Some(ref val) = ls.get() {
                assert_eq!(val.0, "Hello".to_string());
            } else {
                panic!("2. val not found");
            }
        }
    }
}
