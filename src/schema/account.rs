/// Data structure for registering new account
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewAccount {
    /// Unique account id
    pub id: String,
    /// Account password
    pub password: String,
}

impl NewAccount {

    /// Gives the key to use for db sets and gets
    pub fn get_key(&self) -> String {
        format!("account:{}", self.id)
    }
}
