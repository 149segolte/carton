use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use serde_json::json;
use tuirealm::event::NoUserEvent;
use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{
    Application, AttrValue, Attribute, EventListenerCfg, Sub, SubClause, SubEventClause, Update,
};
use uuid::Uuid;

use crate::async_utils::{Runner, Task, TaskNames};
use crate::components::input::TextInput;
use crate::components::label::TextLabel;
use crate::components::paragraph::{Header, HeaderOverview, Preview, PreviewDataTypes};
use crate::constants::{AuthPlatform, Id, Msg};

pub struct Model {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub uniforms: HashMap<String, Rc<RefCell<String>>>,
    pub data: HashMap<PreviewDataTypes, Rc<RefCell<String>>>,
    pub runner: Option<Runner>,
    pub queue: Vec<Uuid>,
    test: String,
    pub terminal: TerminalBridge,
}

impl Default for Model {
    fn default() -> Self {
        let (app, data, uniforms) = Self::init_app();
        Self {
            app,
            quit: false,
            redraw: true,
            uniforms,
            data,
            runner: None,
            queue: Vec::new(),
            test: "".to_string(),
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
        }
    }
}

impl Model {
    pub fn new(auth: AuthPlatform, token: String) -> Self {
        let mut obj = Self::default();
        obj.uniforms
            .insert("auth".to_string(), Rc::new(RefCell::new(auth.to_string())));
        obj.uniforms
            .insert("token".to_string(), Rc::new(RefCell::new(token.clone())));

        let runner = Runner::new();
        obj.runner = Some(runner);

        let id = obj.runner.clone().unwrap().add_async_task(
            true,
            Task::new(
                TaskNames::ProviderOverview,
                json!({
                    "auth": auth.to_string(),
                    "token": token,
                }),
            ),
        );
        obj.queue.push(id);
        let id = obj.runner.clone().unwrap().add_async_task(
            true,
            Task::new(
                TaskNames::FetchServers,
                json!({
                    "auth": auth.to_string(),
                    "token": token,
                }),
            ),
        );
        obj.test = id.to_string();
        obj.queue.push(id);

        obj
    }

    pub fn post(self, update: bool, input: Task) -> Uuid {
        self.runner.clone().unwrap().add_async_task(update, input)
    }

    pub fn fetch(&mut self, id: Uuid) -> Option<(bool, Task)> {
        self.runner.clone().unwrap().get_async_task(id)
    }

    pub fn handle_tasks(&mut self) -> bool {
        let mut i = 0;
        let mut flag = false;
        while i < self.queue.len() {
            let id = self.queue[i];
            if let Some((update, task)) = self.fetch(id) {
                flag |= update;
                match task.name {
                    TaskNames::ProviderOverview => {
                        let data = HeaderOverview::new(task.data);
                        let mut x = self
                            .uniforms
                            .get_mut("header_overview")
                            .unwrap()
                            .borrow_mut();
                        x.clear();
                        x.push_str(serde_json::to_string(&data).unwrap().as_str());
                    }
                    _ => {
                        self.uniforms
                            .insert(id.to_string(), Rc::new(RefCell::new(task.data.to_string())));
                    }
                }
                self.queue.remove(i);
            } else {
                i += 1;
            }
        }
        flag
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
                            Constraint::Length(3), // Label
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
        HashMap<String, Rc<RefCell<String>>>,
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
        let mut uniforms = HashMap::default();
        let overview = HeaderOverview::default();
        uniforms.insert(
            "header_overview".to_string(),
            Rc::new(RefCell::new(serde_json::to_string(&overview).unwrap())),
        );

        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        // Mount header
        assert!(app
            .mount(
                Id::Header,
                Box::new(Header::new(
                    uniforms.get_mut("header_overview").unwrap().clone()
                )),
                Vec::default()
            )
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
        (app, data, uniforms)
    }

    pub fn terminate(&mut self) {
        let _ = self.terminal.leave_alternate_screen();
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.clear_screen();
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if self.handle_tasks() {
            self.redraw = true;
        }
        let res = if let Some(msg) = msg {
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
            }
        } else {
            None
        };
        assert!(self
            .app
            .attr(
                &Id::Label,
                Attribute::Text,
                AttrValue::String(
                    format!("Debug: {:?}", self.uniforms.get(&self.test))
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(128)
                        .map(|chunk| chunk.iter().collect::<String>())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            )
            .is_ok());
        res
    }
}
