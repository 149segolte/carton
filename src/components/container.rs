use tui_realm_stdlib::Container;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
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
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };

        match self.perform(cmd) {
            _ => None,
        }
    }
}
