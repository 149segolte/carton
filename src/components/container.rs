use tui_realm_stdlib::Container;
use tuirealm::command::Cmd;
use tuirealm::props::{Alignment, Color, Layout, PropPayload, PropValue, TextSpan};
use tuirealm::tui::layout::{Constraint, Direction};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent};

use crate::components::span::TextBox;
use crate::constants::{Msg, ProviderStatus, UserEvent, UserEventIter};

#[derive(MockComponent)]
pub struct Header {
    component: Container,
}

impl Default for Header {
    fn default() -> Self {
        let mut obj = Self {
            component: Container::default()
                .background(Color::Reset)
                .foreground(Color::Reset)
                .title(" Carton ", Alignment::Left)
                .children(vec![
                    Box::new(TextBox::default()),
                    Box::new(TextBox::default()),
                    Box::new(TextBox::default()),
                ])
                .layout(
                    Layout::default()
                        .direction(Direction::Vertical)
                        .margin(2)
                        .constraints(
                            [
                                Constraint::Length(2),
                                Constraint::Length(2),
                                Constraint::Length(2),
                            ]
                            .as_ref(),
                        ),
                ),
        };
        obj.update_status(ProviderStatus::default());
        obj
    }
}

impl Header {
    pub fn update_status(&mut self, status: ProviderStatus) {
        let mut children = self.component.children.iter_mut();
        for i in 0..3 {
            let data = match i {
                0 => vec![
                    TextSpan::new(" Provider: "),
                    TextSpan::new(status.name.clone()).bold(),
                    TextSpan::new(", Status: "),
                    if status.status == "Connected" {
                        TextSpan::new(status.status.clone()).fg(Color::LightGreen)
                    } else {
                        TextSpan::new(status.status.clone()).fg(Color::LightYellow)
                    },
                ],
                1 => vec![
                    TextSpan::new(" Servers: "),
                    if status.servers > 0 {
                        TextSpan::new(status.servers.to_string()).fg(Color::LightGreen)
                    } else {
                        TextSpan::new(status.servers.to_string()).fg(Color::LightYellow)
                    },
                    TextSpan::new(" | Primary IPs: "),
                    if status.primary_ips > 0 {
                        TextSpan::new(status.primary_ips.to_string()).fg(Color::LightGreen)
                    } else {
                        TextSpan::new(status.primary_ips.to_string()).fg(Color::LightYellow)
                    },
                    TextSpan::new(" | Firewalls: "),
                    if status.firewalls > 0 {
                        TextSpan::new(status.firewalls.to_string()).fg(Color::LightGreen)
                    } else {
                        TextSpan::new(status.firewalls.to_string()).fg(Color::LightYellow)
                    },
                    TextSpan::new(" | Load Balancers: "),
                    if status.load_balancers > 0 {
                        TextSpan::new(status.load_balancers.to_string()).fg(Color::LightGreen)
                    } else {
                        TextSpan::new(status.load_balancers.to_string()).fg(Color::LightYellow)
                    },
                ],
                2 => vec![TextSpan::new("Press ESC to exit.")],
                _ => vec![],
            };
            if let Some(textbox) = children.next() {
                textbox.attr(
                    Attribute::Text,
                    AttrValue::Payload(PropPayload::Vec(
                        data.iter()
                            .map(|x| PropValue::TextSpan(x.clone()))
                            .collect(),
                    )),
                );
            }
        }
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

        if let Some(AttrValue::Flag(true)) = self.query(Attribute::Custom("launch")) {
            self.attr(Attribute::Custom("launch"), AttrValue::Flag(false));
            return Some(Msg::Launch);
        }

        self.perform(cmd);
        None
    }
}