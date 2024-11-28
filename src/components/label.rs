use tui_realm_stdlib::Label;
use tuirealm::command::Cmd;
use tuirealm::props::{Alignment, Color, TextModifiers, TextSpan};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent};

use crate::constants::{Msg, UserEvent, UserEventIter};

#[derive(MockComponent)]
pub struct TextLabel {
    component: Label,
}

impl Default for TextLabel {
    fn default() -> Self {
        Self {
            component: Label::default()
                .text("Waiting for a Msg...")
                .alignment(Alignment::Left)
                .background(Color::Reset)
                .foreground(Color::LightYellow)
                .modifiers(TextModifiers::BOLD),
        }
    }
}

impl Component<Msg, UserEventIter> for TextLabel {
    fn on(&mut self, ev: Event<UserEventIter>) -> Option<Msg> {
        let _ = match ev {
            Event::User(UserEventIter { events }) => {
                for ev in events {
                    if let UserEvent::Error(err) = ev {
                        self.attr(Attribute::Text, AttrValue::Text(TextSpan::new(err)));
                    }
                }
                Cmd::None
            }
            _ => Cmd::None,
        };

        None
    }
}
