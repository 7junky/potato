use chrono::{DateTime, Utc};

pub struct Cookie<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub expires: Option<DateTime<Utc>>,
    pub secure: bool,
    pub http_only: bool,
}

impl<'a> Cookie<'a> {
    pub(super) fn to_string(&self) -> String {
        let mut cookie = format!("{}={}", self.key, self.value);

        if let Some(expires) = self.expires {
            let expires = expires.to_rfc2822();
            cookie.push_str(&format!("; Expires={}", expires));
        };

        if self.secure {
            cookie.push_str("; Secure")
        };

        if self.http_only {
            cookie.push_str("; HttpOnly")
        }

        cookie
    }
}
