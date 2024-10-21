use std::time::Duration;

use tuirealm::terminal::TerminalBridge;
use tuirealm::{Application, AttrValue, Attribute, EventListenerCfg, Update};

use crate::app::interface::Interface;
use crate::app::tasks::{Task, TaskHandler, TaskRequest};
use crate::constants::{Args, Config, Id, Msg, UserEventIter};

pub struct Model {
    pub app: Application<Id, Msg, UserEventIter>,
    pub quit: bool,
    pub redraw: bool,
    pub config: Config,
    pub tasks: TaskHandler,
    pub interface: Interface,
    pub terminal: TerminalBridge,
}

impl Model {
    pub fn new(args: Args) -> Self {
        let (app, config, task_handler, interface) = Self::init_app(args);
        Self {
            app,
            quit: false,
            redraw: true,
            config,
            tasks: task_handler,
            interface,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
        }
    }

    fn init_app(
        args: Args,
    ) -> (
        Application<Id, Msg, UserEventIter>,
        Config,
        TaskHandler,
        Interface,
    ) {
        let config = Config::new(args);
        let task_handler = TaskHandler::new(config.clone());
        let interface = Interface::default();

        let mut app: Application<Id, Msg, UserEventIter> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1))
                .port(Box::new(task_handler.clone()), Duration::from_millis(100)),
        );

        interface.init(&mut app);

        (app, config, task_handler, interface)
    }

    pub fn view(&mut self) {
        self.interface.view(&mut self.app, &mut self.terminal);
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
                Msg::Nop => None,
                Msg::Launch => Some(Msg::UpdateProviderStatus),
                Msg::AppClose => {
                    self.quit = true; // Terminate
                    None
                }
                Msg::Connected => {
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String("Provider connected".to_string())
                        )
                        .is_ok());

                    todo!()
                }
                Msg::Disconnected => {
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String("Provider disconnected".to_string())
                        )
                        .is_ok());

                    todo!()
                }
                Msg::ChangeFocus => {
                    self.interface.change_focus(&mut self.app);
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
                    self.tasks
                        .clone()
                        .add_task(Task::new(TaskRequest::ProviderStatus));

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
