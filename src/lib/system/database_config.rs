pub struct DatabaseConfig<'a> {
    db_name: &'a str,
    username: &'a str,
    password: &'a str,
    host: &'a str,
    port: u16,
}

impl<'a> DatabaseConfig<'a> {
    pub fn new(
        db_name: &'a str,
        username: &'a str,
        password: &'a str,
        host: &'a str,
        port: u16,
    ) -> Self {
        Self {
            db_name,
            username,
            password,
            host,
            port,
        }
    }
    pub fn db_name(&self) -> &str {
        self.db_name
    }
    pub fn username(&self) -> &str {
        self.username
    }
    pub fn password(&self) -> &str {
        self.password
    }
    pub fn host(&self) -> &str {
        self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
}
