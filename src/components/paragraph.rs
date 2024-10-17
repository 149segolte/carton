use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tui_realm_stdlib::Paragraph;
use tuirealm::command::Cmd;
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, Color, PropPayload, PropValue, TextSpan};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent};

use crate::constants::{Id, Msg};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct HeaderOverview {
    name: String,
    status: String,
    servers: usize,
    primary_ips: usize,
    firewalls: usize,
    load_balancers: usize,
}

impl Default for HeaderOverview {
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

impl HeaderOverview {
    pub fn new(data: serde_json::Value) -> Self {
        let mut obj = Self::default();
        if let Some(name) = data.get("name") {
            obj.name = name.as_str().unwrap().to_string();
        }
        if let Some(status) = data.get("status") {
            obj.status = status.as_str().unwrap().to_string();
        }
        if let Some(servers) = data.get("servers") {
            obj.servers = servers.as_u64().unwrap() as usize;
        }
        if let Some(primary_ips) = data.get("primary_ips") {
            obj.primary_ips = primary_ips.as_u64().unwrap() as usize;
        }
        if let Some(firewalls) = data.get("firewalls") {
            obj.firewalls = firewalls.as_u64().unwrap() as usize;
        }
        if let Some(load_balancers) = data.get("load_balancers") {
            obj.load_balancers = load_balancers.as_u64().unwrap() as usize;
        }
        obj
    }
}

#[derive(MockComponent)]
pub struct Header {
    component: Paragraph,
    overview: Option<Rc<RefCell<String>>>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .background(Color::Reset)
                .foreground(Color::Reset)
                .title(" Carton ", Alignment::Left),
            overview: None,
        }
    }
}

impl Header {
    pub fn new(overview: Rc<RefCell<String>>) -> Self {
        let mut obj = Self::default();
        obj.overview = Some(overview);
        obj.update_overview();
        obj
    }

    pub fn update_overview(&mut self) {
        let data = self.overview.as_ref().unwrap().as_ref().borrow();
        let overview = serde_json::from_str::<HeaderOverview>(&data).unwrap();

        self.component.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                vec![
                    TextSpan::new(""),
                    TextSpan::new(format!(
                        " Provider: {}, Status: {}",
                        overview.name, overview.status
                    )),
                    TextSpan::new(""),
                    TextSpan::new(format!(
                        " Servers: {} | Primary IPs: {} | Firewalls: {} | Load Balancers: {}",
                        overview.servers,
                        overview.primary_ips,
                        overview.firewalls,
                        overview.load_balancers
                    )),
                    TextSpan::new(""),
                    TextSpan::new("Press ESC to exit."),
                ]
                .iter()
                .cloned()
                .map(PropValue::TextSpan)
                .collect(),
            )),
        );
    }
}

impl Component<Msg, NoUserEvent> for Header {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.update_overview();

        let cmd = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                return Some(Msg::Focus(Id::TextInput1))
            }
            _ => Cmd::None,
        };

        self.perform(cmd);
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewDataTypes {
    Servers,
    Name,
    Region,
    Image,
}

#[derive(MockComponent)]
pub struct Preview {
    component: Paragraph,
    servers: Option<Rc<RefCell<String>>>,
    name: Option<Rc<RefCell<String>>>,
    region: Option<Rc<RefCell<String>>>,
    image: Option<Rc<RefCell<String>>>,
}

impl Default for Preview {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .background(Color::Reset)
                .foreground(Color::Reset)
                .title(" Preview ", Alignment::Left),
            servers: None,
            name: None,
            region: None,
            image: None,
        }
    }
}

impl Preview {
    pub fn new(
        servers: Rc<RefCell<String>>,
        name: Rc<RefCell<String>>,
        region: Rc<RefCell<String>>,
        image: Rc<RefCell<String>>,
    ) -> Self {
        let mut s = Self::default();
        s.servers = Some(servers);
        s.name = Some(name);
        s.region = Some(region);
        s.image = Some(image);
        s.update_text();
        s
    }

    pub fn update_text(&mut self) {
        self.component.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                [
                    TextSpan::new(format!(
                        "servers: {}",
                        self.servers.as_ref().unwrap().as_ref().borrow()
                    )),
                    TextSpan::new(format!(
                        "name: {}",
                        self.name.as_ref().unwrap().as_ref().borrow()
                    )),
                    TextSpan::new(format!(
                        "region: /zone/{}",
                        self.region.as_ref().unwrap().as_ref().borrow()
                    )),
                    TextSpan::new(format!(
                        "image: /images/{}",
                        self.image.as_ref().unwrap().as_ref().borrow()
                    )),
                ]
                .iter()
                .cloned()
                .map(PropValue::TextSpan)
                .collect(),
            )),
        );
    }
}

impl Component<Msg, NoUserEvent> for Preview {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        self.update_text();

        let cmd = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };

        self.perform(cmd);
        None
    }
}
