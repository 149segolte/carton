use std::cell::RefCell;
use std::rc::Rc;

use tui_realm_stdlib::Paragraph;
use tuirealm::command::Cmd;
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, Color, PropPayload, PropValue, TextSpan};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent};

use crate::constants::{Msg, UserEventIter};

#[derive(MockComponent)]
pub struct ServerListDisconnected {
    component: Paragraph,
}

impl Default for ServerListDisconnected {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .background(Color::Reset)
                .foreground(Color::LightYellow)
                .title(" Servers List ", Alignment::Center)
                .text(&[
                    TextSpan::new(""),
                    TextSpan::new("Refresh or change provider to connect."),
                ])
                .alignment(Alignment::Center),
        }
    }
}

impl Component<Msg, UserEventIter> for ServerListDisconnected {
    fn on(&mut self, _: Event<UserEventIter>) -> Option<Msg> {
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
