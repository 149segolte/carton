use tui_realm_stdlib::Input;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style};
use tuirealm::{Component, Event, MockComponent};

use crate::constants::{Id, Msg, UserEventIter};

#[derive(MockComponent)]
pub struct TextInput {
    component: Input,
    id: Option<Id>,
    link: Option<Id>,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightYellow),
                )
                .foreground(Color::LightYellow)
                .input_type(InputType::Text)
                .invalid_style(Style::default().fg(Color::Red)),
            id: None,
            link: None,
        }
    }
}

impl TextInput {
    pub fn new(id: Id, link: Id) -> Self {
        Self::default().customize(id).with_link(link)
    }

    pub fn customize(mut self, id: Id) -> Self {
        self.id = Some(id.clone());
        match id {
            Id::TextInput1 => {
                self.component = self.component.title(" Name ", Alignment::Left);
            }
            Id::TextInput2 => {
                self.component = self.component.title(" Region ", Alignment::Left);
            }
            Id::TextInput3 => {
                self.component = self.component.title(" Image ", Alignment::Left);
            }
            _ => {}
        }
        self
    }

    pub fn with_link(mut self, link: Id) -> Self {
        self.link = Some(link);
        self
    }
}

impl Component<Msg, UserEventIter> for TextInput {
    fn on(&mut self, ev: Event<UserEventIter>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                if self.link.is_some() {
                    return Some(Msg::Focus(self.link.clone().unwrap()));
                }
                Cmd::None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => Cmd::Move(Direction::Left),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => Cmd::Move(Direction::Right),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => Cmd::GoTo(Position::Begin),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => Cmd::GoTo(Position::End),
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => Cmd::Cancel,
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => Cmd::Delete,
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Type(ch),
            _ => Cmd::None,
        };

        match self.perform(cmd) {
            CmdResult::Changed(state) => Some(Msg::Input(
                self.id.clone().unwrap(),
                state.unwrap_one().unwrap_string(),
            )),
            _ => None,
        }
    }
}
