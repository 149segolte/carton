use clap::Parser;

#[derive(Debug, PartialEq, Clone)]
pub enum AuthPlatform {
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
