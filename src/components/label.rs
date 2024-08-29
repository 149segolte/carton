use tui_realm_stdlib::Label;
use tuirealm::props::{Alignment, Color, TextModifiers};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use crate::Msg;

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

impl Component<Msg, NoUserEvent> for TextLabel {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
