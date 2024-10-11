use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use hcloud::apis::configuration::Configuration;
use hcloud::apis::servers_api;
use tuirealm::event::NoUserEvent;
use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{
    Application, AttrValue, Attribute, EventListenerCfg, Sub, SubClause, SubEventClause, Update,
};

use crate::components::input::TextInput;
use crate::components::label::TextLabel;
use crate::components::paragraph::{Header, Preview, PreviewDataTypes};
use crate::constants::{AuthPlatform, Id, Msg};

pub struct Model {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub uniforms: HashMap<String, String>,
    pub data: HashMap<PreviewDataTypes, Rc<RefCell<String>>>,
    pub terminal: TerminalBridge,
}

impl Default for Model {
    fn default() -> Self {
        let (app, data) = Self::init_app();
        Self {
            app,
            quit: false,
            redraw: true,
            uniforms: HashMap::new(),
            data,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
        }
    }
}

impl Model {
    pub fn new(auth: AuthPlatform, token: String) -> Self {
        let mut obj = Self::default();
        obj.uniforms.insert("auth".to_string(), auth.to_string());
        obj.uniforms.insert("token".to_string(), token.clone());

        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(token);

        let servers = futures::executor::block_on(servers_api::list_servers(
            &configuration,
            Default::default(),
        ))
        .unwrap()
        .servers;

        obj.data.insert(
            PreviewDataTypes::Servers,
            Rc::new(RefCell::new(format!("{:?}", servers))),
        );

        obj
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

    fn init_app() -> (
        Application<Id, Msg, NoUserEvent>,
        HashMap<PreviewDataTypes, Rc<RefCell<String>>>,
    ) {
        let mut data = HashMap::default();
        data.insert(
            PreviewDataTypes::Servers,
            Rc::new(RefCell::new("".to_string())),
        );
        data.insert(
            PreviewDataTypes::Name,
            Rc::new(RefCell::new("test-instance".to_string())),
        );
        data.insert(
            PreviewDataTypes::Region,
            Rc::new(RefCell::new("us-west-1".to_string())),
        );
        data.insert(
            PreviewDataTypes::Image,
            Rc::new(RefCell::new("ubuntu-20.04".to_string())),
        );

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
            .mount(
                Id::Preview,
                Box::new(Preview::new(
                    data.get_mut(&PreviewDataTypes::Servers).unwrap().clone(),
                    data.get_mut(&PreviewDataTypes::Name).unwrap().clone(),
                    data.get_mut(&PreviewDataTypes::Region).unwrap().clone(),
                    data.get_mut(&PreviewDataTypes::Image).unwrap().clone(),
                )),
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
        (app, data)
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
            let res = match msg {
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
                Msg::Input(id, input) => {
                    match id {
                        Id::TextInput1 => {
                            let mut x = self
                                .data
                                .get_mut(&PreviewDataTypes::Name)
                                .unwrap()
                                .borrow_mut();
                            x.clear();
                            x.push_str(input.as_str());
                        }
                        Id::TextInput2 => {
                            let mut x = self
                                .data
                                .get_mut(&PreviewDataTypes::Region)
                                .unwrap()
                                .borrow_mut();
                            x.clear();
                            x.push_str(input.as_str());
                        }
                        Id::TextInput3 => {
                            let mut x = self
                                .data
                                .get_mut(&PreviewDataTypes::Image)
                                .unwrap()
                                .borrow_mut();
                            x.clear();
                            x.push_str(input.as_str());
                        }
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
            };
            assert!(self
                .app
                .attr(
                    &Id::Label,
                    Attribute::Text,
                    AttrValue::String(format!(
                        "Debug: {} {}",
                        self.uniforms["auth"], self.uniforms["token"]
                    ))
                )
                .is_ok());
            return res;
        } else {
            None
        }
    }
}
