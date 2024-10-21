use tui_realm_stdlib::Span;
use tuirealm::props::{Alignment, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent};

use crate::constants::{Msg, UserEventIter};

#[derive(MockComponent)]
pub struct TextBox {
    component: Span,
}

impl Default for TextBox {
    fn default() -> Self {
        Self {
            component: Span::default()
                .alignment(Alignment::Left)
                .background(Color::Reset)
                .foreground(Color::Reset),
        }
    }
}

impl TextBox {
    pub fn new(spans: &[TextSpan]) -> Self {
        Self {
            component: Span::default()
                .alignment(Alignment::Left)
                .background(Color::Reset)
                .foreground(Color::Reset)
                .spans(spans),
        }
    }
}

impl Component<Msg, UserEventIter> for TextBox {
    fn on(&mut self, _: Event<UserEventIter>) -> Option<Msg> {
        None
    }
}
