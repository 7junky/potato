use std::collections::HashMap;

#[derive(Debug)]
pub(super) struct PathAndQuery {
    pub(super) path: String,
    pub(super) query: HashMap<String, String>,
}

impl PathAndQuery {
    pub fn from_target(target: &str) -> Self {
        let mut query = HashMap::new();

        let (path, raw_query) = match target.split_once("?") {
            Some(params) => params,
            None => (target, ""),
        };

        for q in raw_query.rsplit("&") {
            let (key, value) = match q.split_once("=") {
                Some(kv) => kv,
                None => continue,
            };

            query.insert(key.into(), value.into());
        }

        Self {
            path: path.to_owned(),
            query,
        }
    }
}
