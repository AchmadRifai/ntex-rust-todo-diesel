use std::net::ToSocketAddrs;

pub struct ServerInfo {
    pub host: Option<String>,
    pub port: Option<i16>,
}

impl ServerInfo {
    pub fn to_socket_addr(&self) -> impl ToSocketAddrs {
        (
            self.host
                .clone()
                .unwrap_or_else(|| String::from("127.0.0.1")),
            self.port.unwrap_or_else(|| 8080).try_into().unwrap(),
        )
    }
}

impl std::fmt::Display for ServerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ServerInfo[host={}, port={}]",
            self.host
                .clone()
                .unwrap_or_else(|| String::from("127.0.0.1")),
            self.port.unwrap_or_else(|| 8080)
        )
    }
}

pub fn load_server() -> ServerInfo {
    let host = std::env::var("SERVER_HOST").ok();
    let port = std::env::var("SERVER_PORT")
        .ok()
        .map(|s| s.parse().ok().unwrap_or_else(|| 8080));
    ServerInfo { host, port }
}
