use tui_realm_stdlib::Phantom;
use tuirealm::{
    command::Cmd,
    event::{Key, KeyEvent, KeyModifiers},
    Application, Component, Event, MockComponent, Sub, SubClause, SubEventClause,
};

use crate::constants::{Id, Msg, UserEventIter};

#[derive(MockComponent, Default)]
pub struct PhantomHandler {
    component: Phantom,
}

impl PhantomHandler {
    pub fn mount(app: &mut Application<Id, Msg, UserEventIter>) {
        assert!(app
            .mount(
                Id::Phantom,
                Box::new(PhantomHandler::default()),
                vec![
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Esc,
                            modifiers: KeyModifiers::NONE
                        }),
                        SubClause::Always
                    ),
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Tab,
                            modifiers: KeyModifiers::NONE
                        }),
                        SubClause::Always
                    )
                ]
            )
            .is_ok());
    }
}

impl Component<Msg, UserEventIter> for PhantomHandler {
    fn on(&mut self, ev: Event<UserEventIter>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                return Some(Msg::ChangeFocus(false))
            }
            _ => Cmd::None,
        };

        self.perform(cmd);
        None
    }
}
