use std::cell::RefCell;
use std::rc::Rc;

use tui_realm_stdlib::Paragraph;
use tuirealm::command::Cmd;
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, Color, PropPayload, PropValue, TextSpan};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent};

use crate::constants::{Msg, ProviderStatus, UserEvent, UserEventIter};

#[derive(MockComponent)]
pub struct Header {
    component: Paragraph,
}

impl Default for Header {
    fn default() -> Self {
        let mut obj = Self {
            component: Paragraph::default()
                .background(Color::Reset)
                .foreground(Color::Reset)
                .title(" Carton ", Alignment::Left),
        };
        obj.update_status(ProviderStatus::default());
        obj
    }
}

impl Header {
    pub fn update_status(&mut self, status: ProviderStatus) {
        self.component.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                vec![
                    TextSpan::new(""),
                    TextSpan::new(format!(
                        " Provider: {}, Status: {}",
                        status.name, status.status
                    )),
                    TextSpan::new(""),
                    TextSpan::new(format!(
                        " Servers: {} | Primary IPs: {} | Firewalls: {} | Load Balancers: {}",
                        status.servers, status.primary_ips, status.firewalls, status.load_balancers
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

impl Component<Msg, UserEventIter> for Header {
    fn on(&mut self, ev: Event<UserEventIter>) -> Option<Msg> {
        let cmd = match ev {
            Event::User(UserEventIter { events }) => {
                let mut msg = Msg::Nop;
                for ev in events {
                    if let UserEvent::ProviderStatus(status) = ev {
                        msg = if status.status == "Connected" {
                            Msg::Connected
                        } else {
                            Msg::Disconnected
                        };
                        self.update_status(status);
                    }
                }
                return Some(msg);
            }
            _ => Cmd::None,
        };

        if self
            .query(Attribute::Custom("launch"))
            .unwrap_or(AttrValue::Flag(false))
            .unwrap_flag()
        {
            self.attr(Attribute::Custom("launch"), AttrValue::Flag(false));
            return Some(Msg::Launch);
        }

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
    pub fn new() -> Self {
        let mut s = Self::default();
        s.servers = Some(Rc::new(RefCell::new("0".to_string())));
        s.name = Some(Rc::new(RefCell::new("Unknown".to_string())));
        s.region = Some(Rc::new(RefCell::new("Unknown".to_string())));
        s.image = Some(Rc::new(RefCell::new("Unknown".to_string())));
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

impl Component<Msg, UserEventIter> for Preview {
    fn on(&mut self, ev: Event<UserEventIter>) -> Option<Msg> {
        self.update_text();

        let cmd = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };

        self.perform(cmd);
        None
    }
}
