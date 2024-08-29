use tuirealm::application::PollStrategy;
use tuirealm::Update;

mod app;
mod components;
mod utils;

use crate::app::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    DigitCounterChanged(isize),
    DigitCounterBlur,
    LetterCounterChanged(isize),
    LetterCounterBlur,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Header,
    DigitCounter,
    LetterCounter,
    Label,
}

fn main() {
    let mut model = Model::default();
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();

    while !model.quit {
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                let _ = model.terminal.leave_alternate_screen();
                let _ = model.terminal.disable_raw_mode();
                let _ = model.terminal.clear_screen();
                panic!("Application error: {}", err);
            }
            Ok(messages) if messages.len() > 0 => {
                // NOTE: redraw if at least one msg has been processed
                model.redraw = true;
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }

        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }

    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
}
