use tui_realm_stdlib::Container;
use tuirealm::props::{Alignment, Color};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use crate::Msg;

#[derive(MockComponent)]
pub struct Header {
    component: Container,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            component: Container::default()
                .background(Color::LightBlue)
                .foreground(Color::Reset)
                .title("Carton", Alignment::Left),
        }
    }
}

impl Component<Msg, NoUserEvent> for Header {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
