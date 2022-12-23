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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::PathAndQuery;

    #[test]
    fn test_from_target_empty() {
        let target = "/";

        let pnq = PathAndQuery::from_target(target);

        assert_eq!(pnq.path, "/");
        assert_eq!(pnq.query, HashMap::default());
    }

    #[test]
    fn test_from_target_path() {
        let target = "/path/to/resource";

        let pnq = PathAndQuery::from_target(target);

        assert_eq!(pnq.path, "/path/to/resource");
        assert_eq!(pnq.query, HashMap::default());
    }

    #[test]
    fn test_from_target_path_and_query() {
        let target = "/path/to/resource?a=1&b=2&c=3";

        let pnq = PathAndQuery::from_target(target);

        assert_eq!(pnq.path, "/path/to/resource");
        assert_eq!(pnq.query.len(), 3);
        assert_eq!(pnq.query.get("a"), Some(&"1".to_owned()));
        assert_eq!(pnq.query.get("b"), Some(&"2".to_owned()));
        assert_eq!(pnq.query.get("c"), Some(&"3".to_owned()));
    }

    #[test]
    #[ignore]
    fn test_from_target_multivalue_query() {
        let target = "/path?a[0]=1&a[1]=2&a[2]=3";

        let pnq = PathAndQuery::from_target(target);

        assert_eq!(pnq.path, "/path");
        assert_eq!(pnq.query.len(), 3);
        assert_eq!(pnq.query.get("a[0]"), Some(&"1".to_owned()));
        assert_eq!(pnq.query.get("a[1]"), Some(&"2".to_owned()));
        assert_eq!(pnq.query.get("a[2]"), Some(&"3".to_owned()));
    }
}
