use std::cell::RefCell;
use std::rc::Rc;

use tui_realm_stdlib::Paragraph;
use tuirealm::command::Cmd;
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, Color, PropPayload, PropValue, TextSpan};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent};

use crate::constants::{Id, Msg};

#[derive(MockComponent)]
pub struct Header {
    component: Paragraph,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .background(Color::Reset)
                .foreground(Color::Reset)
                .title(" Carton ", Alignment::Left)
                .text(&[
                    TextSpan::new(""),
                    TextSpan::new(" GCP Status: Disconnected"),
                    TextSpan::new(" AWS Status: Disconnected"),
                    TextSpan::new(" Hetzner Status: Disconnected"),
                    TextSpan::new(""),
                    TextSpan::new("Press ESC to exit"),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for Header {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
