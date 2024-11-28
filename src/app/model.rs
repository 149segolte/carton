use std::time::Duration;

use tuirealm::terminal::TerminalBridge;
use tuirealm::{Application, AttrValue, Attribute, EventListenerCfg, Update};

use crate::app::interface::Interface;
use crate::app::tasks::{Task, TaskHandler, Tasks};
use crate::constants::{Args, Config, Id, InterfaceMsg, Msg, ServerHandle, State, UserEventIter};

pub struct Model {
    pub app: Application<Id, Msg, UserEventIter>,
    pub quit: bool,
    pub redraw: bool,
    pub tasks: TaskHandler,
    pub interface: Interface,
    pub terminal: TerminalBridge,
}

impl Model {
    pub fn new(args: Args) -> Self {
        let config = Config::new(args);
        let task_handler = TaskHandler::new(config.clone());
        let interface = Interface::default();
        let mut terminal = TerminalBridge::new().expect("Cannot initialize terminal");

        let mut app: Application<Id, Msg, UserEventIter> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_millis(250))
                .port(Box::new(task_handler.clone()), Duration::from_millis(100)),
        );
        interface.init(&mut app, &mut terminal);

        Self {
            app,
            quit: false,
            redraw: true,
            tasks: task_handler,
            interface,
            terminal,
        }
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
                Msg::Nop(count) => {
                    if count > 0 {
                        Some(Msg::Nop(count - 1))
                    } else {
                        None
                    }
                }
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

                    // Update UI
                    self.interface
                        .perform(&mut self.app, InterfaceMsg::Connected)
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

                    // Update UI
                    self.interface
                        .perform(&mut self.app, InterfaceMsg::Disconnected)
                }
                Msg::ChangeFocus() => {
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String("Focus changed".to_string())
                        )
                        .is_ok());

                    // Update UI
                    self.interface.change_focus(&mut self.app)
                }
                Msg::UpdateState(state) => {
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String(format!("State update: {:?}", state))
                        )
                        .is_ok());

                    match state {
                        State::SelectedServer(server) => {
                            match server {
                                ServerHandle::Create => {
                                    // Update interface
                                    if self.interface != Interface::Create {
                                        self.interface = Interface::Create;
                                        self.interface.init(&mut self.app, &mut self.terminal);
                                    }
                                    None
                                }
                                _ => {
                                    // Update UI
                                    if self.interface != Interface::Status {
                                        self.interface = Interface::Status;
                                        self.interface.init(&mut self.app, &mut self.terminal);
                                    }
                                    self.interface.perform(
                                        &mut self.app,
                                        InterfaceMsg::SelectedServer(server),
                                    )
                                }
                            }
                        }
                        State::Empty => None,
                    }
                }
                Msg::Input(id, input) => {
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
                Msg::Submit => {
                    let name = self
                        .app
                        .query(&Id::CreateServer1, Attribute::Custom("state"));
                    assert!(name.is_ok());
                    let name = name.unwrap().unwrap().unwrap_string();
                    let srv_type = self
                        .app
                        .query(&Id::CreateServer2, Attribute::Custom("state"));
                    assert!(srv_type.is_ok());
                    let srv_type = srv_type.unwrap().unwrap().unwrap_string();
                    let image = self
                        .app
                        .query(&Id::CreateServer3, Attribute::Custom("state"));
                    assert!(image.is_ok());
                    let image = image.unwrap().unwrap().unwrap_string();

                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String(format!(
                                "Creating server: {}, {}, {}",
                                name, srv_type, image
                            ))
                        )
                        .is_ok());

                    if name.trim().is_empty() || !name.is_ascii() {
                        // Update label
                        assert!(self
                            .app
                            .attr(
                                &Id::Label,
                                Attribute::Text,
                                AttrValue::String("Name invalid".to_string())
                            )
                            .is_ok());
                    }

                    if srv_type.trim().is_empty() || !srv_type.is_ascii() {
                        // Update label
                        assert!(self
                            .app
                            .attr(
                                &Id::Label,
                                Attribute::Text,
                                AttrValue::String("Type invalid".to_string())
                            )
                            .is_ok());
                    }

                    if image.trim().is_empty() || !image.is_ascii() {
                        // Update label
                        assert!(self
                            .app
                            .attr(
                                &Id::Label,
                                Attribute::Text,
                                AttrValue::String("Image invalid".to_string())
                            )
                            .is_ok());
                    }

                    // Trigger task
                    self.tasks
                        .clone()
                        .add_task(Task::new(Tasks::CreateServer(name, srv_type, image)));

                    None
                }
                Msg::UpdateProviderStatus => {
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String("Provider status update issued".to_string())
                        )
                        .is_ok());

                    // Trigger task
                    self.tasks
                        .clone()
                        .add_task(Task::new(Tasks::ProviderStatus));

                    None
                }
                Msg::FetchServers => {
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String("Fetching servers".to_string())
                        )
                        .is_ok());

                    // Trigger task
                    self.tasks.clone().add_task(Task::new(Tasks::FetchServers));

                    None
                }
                Msg::Info(msg) => {
                    // Update label
                    assert!(self
                        .app
                        .attr(&Id::Label, Attribute::Text, AttrValue::String(msg))
                        .is_ok());

                    None
                }
            }
        } else {
            None
        }
    }
}
