use clap::Parser;
use tuirealm::{application::PollStrategy, Update};

mod app;
mod components;
mod constants;

use crate::app::Model;

fn main() {
    let args = constants::Args::parse();

    let mut model = Model::new(args.auth, args.token);
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();

    while !model.quit {
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                model.terminate();
                panic!("Application error: {}", err);
            }
            Ok(messages) if !messages.is_empty() => {
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

    model.terminate();
}
