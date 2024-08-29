use tui_realm_stdlib::Input;
use tuirealm::command::Cmd;
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, Color, InputType, Style};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use crate::{Id, Msg};

#[derive(MockComponent)]
pub struct TextInput {
    component: Input,
    link: Option<Id>,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            component: Input::default()
                .background(Color::Reset)
                .foreground(Color::Reset)
                .title(" Name ", Alignment::Left)
                .input_type(InputType::Text)
                .input_len(64)
                .placeholder("Test Server", Style::default()),
            link: None,
        }
    }
}

impl TextInput {
    pub fn new(link: Id) -> Self {
        Self::default().with_link(link)
    }

    pub fn with_link(mut self, link: Id) -> Self {
        self.link = Some(link);
        self
    }
}

impl Component<Msg, NoUserEvent> for TextInput {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => {
                if self.link.is_some() {
                    return Some(Msg::Focus(self.link.clone().unwrap()));
                }
                Cmd::None
            }
            _ => Cmd::None,
        };

        self.perform(cmd);
        None
    }
}
