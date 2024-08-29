use tui_realm_stdlib::Paragraph;
use tuirealm::command::Cmd;
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, Color};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use crate::{Id, Msg};

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
                .title(" Carton ", Alignment::Left),
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
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::Focus(Id::TextInput1)),
            _ => Cmd::None,
        };

        self.perform(cmd);
        None
    }
}

#[derive(MockComponent)]
pub struct Preview {
    component: Paragraph,
}

impl Default for Preview {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .background(Color::Reset)
                .foreground(Color::Reset)
                .title(" Preview ", Alignment::Left),
        }
    }
}

impl Component<Msg, NoUserEvent> for Preview {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };

        self.perform(cmd);
        None
    }
}
