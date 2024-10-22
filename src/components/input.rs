use std::ops::Index;

use tui_realm_stdlib::Input;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent};

use crate::constants::{InputId, Msg, UserEventIter};

#[derive(MockComponent)]
pub struct TextInput {
    component: Input,
    id: Option<InputId>,
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
        }
    }
}

impl TextInput {
    pub fn new(id: InputId, title: &str) -> Self {
        Self {
            id: Some(id),
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightYellow),
                )
                .title(title, Alignment::Left)
                .foreground(Color::LightYellow)
                .input_type(InputType::Text)
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Custom(s) => {
                let chunks = s.split(":").take(2).collect::<Vec<_>>();
                let action = chunks.index(0);
                let target = chunks.index(1);
                if *action == "focus" {
                    if let Some(id) = self.id.as_ref() {
                        if id.to_string() == *target {
                            self.component.attr(Attribute::Focus, AttrValue::Flag(true));
                        } else {
                            self.component
                                .attr(Attribute::Focus, AttrValue::Flag(false));
                        }
                    }
                }
                CmdResult::Custom(s)
            }
            _ => self.component.perform(cmd),
        }
    }
}

impl Component<Msg, UserEventIter> for TextInput {
    fn on(&mut self, ev: Event<UserEventIter>) -> Option<Msg> {
        let cmd = match ev {
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
