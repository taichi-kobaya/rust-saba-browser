use alloc::string::String;
use alloc::string::ToString;

#[derive(Debug, Clone, PartialEq)]

pub struct Url {
    url:        String,
    host:       String,
    port:       String,
    path:       String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }
}