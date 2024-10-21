use tui_realm_stdlib::Paragraph;
use tuirealm::props::{Alignment, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent};

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
