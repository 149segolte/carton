use clap::Parser;

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
    AppClose,
    Focus(Id),
    Input(Id, String),
    UpdateProviderStatus,
    Launch,
    Nop,
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

#[derive(Debug, Clone, PartialOrd, Eq)]
pub enum UserEvent {
    ProviderStatus(ProviderStatus),
    ServerList(Vec<String>),
    Error(String),
}

impl PartialEq for UserEvent {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (UserEvent::ProviderStatus(_), UserEvent::ProviderStatus(_))
                | (UserEvent::ServerList(_), UserEvent::ServerList(_))
                | (UserEvent::Error(_), UserEvent::Error(_))
        )
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
