use serde::Deserialize;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Usernames {
    users: Vec<String>,
    forbidden_users: Vec<String>,
}

impl Usernames {
    /// Check if nick matches any forbidden names.
    ///
    /// ```
    /// use marbles_tool::types;
    /// let u = types::Usernames::new().forbidden_contains("Bot");
    /// assert_eq!(u, true);
    /// ```
    pub fn forbidden_contains(&self, string: &str) -> bool {
        self.forbidden_users.contains(&string.to_owned())
    }

    /// Check if we already have nick.
    fn contains(&self, string: &str) -> bool {
        self.users.contains(&string.to_owned())
    }

    pub fn new() -> Self {
        let forbidden_users = [
            "Bot",
            "Emotes",
            "tng69",
            "voy70",
            "gpt71",
            "guhpt72",
            "wikiboy73",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();
        Self {
            users: vec![],
            forbidden_users: forbidden_users,
        }
    }

    pub fn add_user(&mut self, nick: String, tx: &mpsc::UnboundedSender<String>) {
        if self.forbidden_contains(&nick) {
            return;
        }

        if !self.contains(&nick) {
            self.users.push(nick.to_owned());
            println!("{}", nick);
            let _ = tx.send(nick.to_owned());
        }
    }

    pub fn as_vec(&self) -> Vec<String> {
        self.users.clone()
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Sub {
    tier: u64,
    source: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Watch {
    id: String,
    platform: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WSMsg {
    id: u64,
    nick: String,
    roles: Vec<String>,
    features: Vec<String>,
    #[serde(rename = "createdDate")]
    created_date: String,
    watching: Option<Watch>,
    subscription: Option<Sub>,
    timestamp: u64,
    data: String,
}

impl WSMsg {
    pub fn get_name_data(&self) -> (String, String) {
        (self.nick.clone(), self.data.clone())
    }
}
