use std::time::Duration;

use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{
    Application, AttrValue, Attribute, EventListenerCfg, Sub, SubClause, SubEventClause, Update,
};

use crate::app::tasks::{SingletonTasks, Task, TaskHandler, TaskRequest};
use crate::components::input::TextInput;
use crate::components::label::TextLabel;
use crate::components::list::ServerList;
use crate::components::paragraph::{Header, Preview};
use crate::constants::{Args, Config, Id, Msg, ProviderStatus, UserEvent, UserEventIter};

pub struct Model {
    pub app: Application<Id, Msg, UserEventIter>,
    pub quit: bool,
    pub redraw: bool,
    pub data: Config,
    pub tasks: TaskHandler,
    pub terminal: TerminalBridge,
}

impl Default for Model {
    fn default() -> Self {
        let (app, task_handler) = Self::init_app();
        Self {
            app,
            quit: false,
            redraw: true,
            data: Config::default(),
            tasks: task_handler,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
        }
    }
}

impl Model {
    pub fn new(args: Args) -> Self {
        Self {
            data: Config::new(args),
            ..Self::default()
        }
    }

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
                            Constraint::Length(8),  // Header
                            Constraint::Length(12), // List
                            Constraint::Fill(1),    // UI
                            Constraint::Length(3),  // Label
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                self.app.view(&Id::Header, f, chunks[0]);
                self.app.view(&Id::ServerList, f, chunks[1]);
                self.app.view(&Id::Label, f, chunks[3]);

                let ui_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[2]);
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

    fn init_app() -> (Application<Id, Msg, UserEventIter>, TaskHandler) {
        let task_handler = TaskHandler::new();

        let mut app: Application<Id, Msg, UserEventIter> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1))
                .port(Box::new(task_handler.clone()), Duration::from_millis(100)),
        );

        // Mount header
        assert!(app
            .mount(
                Id::Header,
                Box::new(Header::default()),
                vec![Sub::new(
                    SubEventClause::User(UserEventIter::new(vec![UserEvent::ProviderStatus(
                        ProviderStatus::default()
                    )])),
                    SubClause::Always
                )]
            )
            .is_ok());

        // Mount server list
        assert!(app
            .mount(
                Id::ServerList,
                Box::new(ServerList::default()),
                Vec::default()
            )
            .is_ok());

        // Mount UI
        assert!(app
            .mount(
                Id::Preview,
                Box::new(Preview::new()),
                vec![Sub::new(SubEventClause::Tick, SubClause::Always)]
            )
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
        assert!(app
            .attr(
                &Id::Header,
                Attribute::Custom("launch"),
                AttrValue::Flag(true)
            )
            .is_ok());

        (app, task_handler)
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
                Msg::Launch => Some(Msg::UpdateProviderStatus),
                Msg::Nop => None,
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
                Msg::Input(id, input) => {
                    match id {
                        Id::TextInput1 => {}
                        Id::TextInput2 => {}
                        Id::TextInput3 => {}
                        _ => {}
                    }

                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String(format!("Input from {:?}: {:?}", id, input))
                        )
                        .is_ok());

                    None
                }
                Msg::UpdateProviderStatus => {
                    self.tasks.clone().add_task(Task::new(TaskRequest::Single(
                        SingletonTasks::ProviderStatus(self.data.auth.clone()),
                    )));

                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String("Provider status update issued".to_string())
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
