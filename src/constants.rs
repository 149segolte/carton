use clap::Parser;
use hcloud::models::{server::Status, Server};
use tuirealm::Component;

use crate::components::{
    container::Header,
    input::TextInput,
    label::TextLabel,
    paragraph::{Preview, ServerListDisconnected},
    table::ServerListConnected,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub(crate) auth: AuthPlatform,
    #[arg(short, long)]
    pub(crate) token: String,
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    Nop,
    Launch,
    AppClose,
    Connected,
    Disconnected,
    ChangeFocus,
    UpdateState(UserEvent),
    Input(Id, String),
    UpdateProviderStatus,
    FetchServers,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Header,
    TextInput1,
    TextInput2,
    TextInput3,
    Preview,
    Label,
    ServerList,
    Phantom,
}

pub enum Components {
    Header(Header),
    TextInput(TextInput),
    Preview(Preview),
    TextLabel(TextLabel),
    ServerListConnected(ServerListConnected),
    ServerListDisconnected(ServerListDisconnected),
}

impl Components {
    pub fn unwrap(self) -> Box<dyn Component<Msg, UserEventIter>> {
        match self {
            Components::Header(c) => Box::new(c),
            Components::TextInput(c) => Box::new(c),
            Components::Preview(c) => Box::new(c),
            Components::TextLabel(c) => Box::new(c),
            Components::ServerListConnected(c) => Box::new(c),
            Components::ServerListDisconnected(c) => Box::new(c),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Default)]
pub enum AuthPlatform {
    #[default]
    Google,
    Amazon,
    Hetzner,
}

impl std::str::FromStr for AuthPlatform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "google" => Ok(AuthPlatform::Google),
            "amazon" => Ok(AuthPlatform::Amazon),
            "hetzner" => Ok(AuthPlatform::Hetzner),
            _ => Err("Invalid auth platform".to_string()),
        }
    }
}

impl std::fmt::Display for AuthPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthPlatform::Google => write!(f, "Google"),
            AuthPlatform::Amazon => write!(f, "Amazon"),
            AuthPlatform::Hetzner => write!(f, "Hetzner"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Auth {
    pub auth: AuthPlatform,
    pub token: String,
}

impl Auth {
    pub fn new(auth: AuthPlatform, token: String) -> Self {
        Self { auth, token }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Config {
    pub auth: Auth,
}

impl Config {
    pub fn new(args: Args) -> Self {
        Self {
            auth: Auth::new(args.auth, args.token),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd)]
pub struct ProviderStatus {
    pub name: String,
    pub status: String,
    pub servers: usize,
    pub primary_ips: usize,
    pub firewalls: usize,
    pub load_balancers: usize,
}

impl Default for ProviderStatus {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            status: "Disconnected".to_string(),
            servers: 0,
            primary_ips: 0,
            firewalls: 0,
            load_balancers: 0,
        }
    }
}

impl ProviderStatus {
    pub fn new(name: AuthPlatform) -> Self {
        Self {
            name: name.to_string(),
            status: "Connected".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServerStatus {
    pub name: String,
    pub status: String,
    pub ip: String,
}

#[derive(Debug, Clone)]
pub enum ServerHandle {
    Hetzner(Server),
}

impl ServerHandle {
    pub fn to_status(&self) -> ServerStatus {
        match self {
            ServerHandle::Hetzner(server) => ServerStatus {
                name: server.name.clone(),
                status: if server.status == Status::Running {
                    "Online".to_string()
                } else {
                    "Offline".to_string()
                },
                ip: if let Some(ipv4) = server.public_net.ipv4.as_ref() {
                    ipv4.ip.to_string()
                } else {
                    "Private".to_string()
                },
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ServerListStatus {
    pub servers: Vec<ServerHandle>,
}

impl ServerListStatus {
    pub fn new(servers: Vec<ServerHandle>) -> Self {
        Self { servers }
    }
}

#[derive(Debug, Clone)]
pub enum UserEvent {
    ProviderStatus(ProviderStatus),
    ServerListStatus(ServerListStatus),
    Error(String),
    Empty,
}

impl PartialEq for UserEvent {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (UserEvent::ProviderStatus(_), UserEvent::ProviderStatus(_))
                | (
                    UserEvent::ServerListStatus(_),
                    UserEvent::ServerListStatus(_)
                )
                | (UserEvent::Error(_), UserEvent::Error(_))
        )
    }
}

impl Eq for UserEvent {}

impl PartialOrd for UserEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Eq, Default)]
pub struct UserEventIter {
    pub events: Vec<UserEvent>,
}

impl PartialEq for UserEventIter {
    fn eq(&self, other: &Self) -> bool {
        other
            .events
            .iter()
            .any(|e| self.events.iter().any(|f| e == f))
    }
}

impl UserEventIter {
    pub fn new(events: Vec<UserEvent>) -> Self {
        Self { events }
    }
}
