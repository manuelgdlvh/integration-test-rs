pub struct ServerConfig<'a> {
    host: &'a str,
    port: u16,
}

impl<'a> ServerConfig<'a> {
    pub fn new(host: &'a str, port: u16) -> Self {
        Self { host, port }
    }

    pub fn host(&self) -> &str {
        self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
}
