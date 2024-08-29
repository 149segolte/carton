use std::time::Duration;

use tuirealm::event::NoUserEvent;
use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{Application, AttrValue, Attribute, EventListenerCfg, Update};

use crate::components::input::TextInput;
use crate::components::label::TextLabel;
use crate::components::paragraph::{Header, Preview};
use crate::{Id, Msg};

pub struct Model {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
        }
    }
}

impl Model {
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(8), // Header
                            Constraint::Fill(1),   // UI
                            Constraint::Length(1), // Label
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                self.app.view(&Id::Header, f, chunks[0]);
                self.app.view(&Id::Label, f, chunks[2]);

                let ui_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[1]);
                self.app.view(&Id::Preview, f, ui_chunks[1]);

                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(ui_chunks[0]);
                self.app.view(&Id::TextInput1, f, input_chunks[0]);
                self.app.view(&Id::TextInput2, f, input_chunks[1]);
                self.app.view(&Id::TextInput3, f, input_chunks[2]);
            })
            .is_ok());
    }

    fn init_app() -> Application<Id, Msg, NoUserEvent> {
        // Setup application
        // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
        // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
        // which we will use to update the clock

        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        // Mount header
        assert!(app
            .mount(Id::Header, Box::new(Header::default()), Vec::default())
            .is_ok());

        // Mount UI
        assert!(app
            .mount(Id::Preview, Box::new(Preview::default()), Vec::default())
            .is_ok());
        assert!(app
            .mount(
                Id::TextInput1,
                Box::new(TextInput::new(Id::TextInput1, Id::TextInput2)),
                Vec::default()
            )
            .is_ok());
        assert!(app
            .mount(
                Id::TextInput2,
                Box::new(TextInput::new(Id::TextInput2, Id::TextInput3)),
                Vec::default()
            )
            .is_ok());
        assert!(app
            .mount(
                Id::TextInput3,
                Box::new(TextInput::new(Id::TextInput3, Id::Header)),
                Vec::default()
            )
            .is_ok());

        // Mount Message label
        assert!(app
            .mount(Id::Label, Box::new(TextLabel::default()), Vec::default(),)
            .is_ok());

        // Active Header
        assert!(app.active(&Id::Header).is_ok());
        app
    }

    pub fn terminate(&mut self) {
        let _ = self.terminal.leave_alternate_screen();
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.clear_screen();
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Set redraw
            self.redraw = true;
            // Match message
            match msg {
                Msg::AppClose => {
                    self.quit = true; // Terminate
                    None
                }
                Msg::Focus(id) => {
                    assert!(self.app.active(&id).is_ok());
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String(format!("Focus changed to: {:?}", id))
                        )
                        .is_ok());
                    None
                }
            }
        } else {
            None
        }
    }
}
