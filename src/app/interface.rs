use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{Application, AttrValue, Attribute, Sub, SubClause, SubEventClause};

use crate::components::input::TextInput;
use crate::components::label::TextLabel;
use crate::components::list::ServerList;
use crate::components::paragraph::{Header, Preview};
use crate::components::phantom::PhantomHandler;
use crate::constants::{Id, Msg, ProviderStatus, UserEvent, UserEventIter};

#[derive(Debug, Clone, Default)]
pub enum Interface {
    #[default]
    Status,
}

impl Interface {
    pub fn init(&self, app: &mut Application<Id, Msg, UserEventIter>) {
        match self {
            Interface::Status => {
                app.umount_all();
                // Mount handler
                PhantomHandler::mount(app);

                // Mount header
                assert!(app
                    .mount(
                        Id::Header,
                        Box::new(Header::default()),
                        vec![Sub::new(
                            SubEventClause::User(UserEventIter::new(vec![
                                UserEvent::ProviderStatus(ProviderStatus::default())
                            ])),
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

                // Mount Inputs
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
                        Box::new(TextInput::new(Id::TextInput1)),
                        Vec::default()
                    )
                    .is_ok());
                assert!(app
                    .mount(
                        Id::TextInput2,
                        Box::new(TextInput::new(Id::TextInput2)),
                        Vec::default()
                    )
                    .is_ok());
                assert!(app
                    .mount(
                        Id::TextInput3,
                        Box::new(TextInput::new(Id::TextInput3)),
                        Vec::default()
                    )
                    .is_ok());

                // Mount Message label
                assert!(app
                    .mount(Id::Label, Box::new(TextLabel::default()), Vec::default(),)
                    .is_ok());

                // Activate header
                assert!(app.active(&Id::Header).is_ok());
                assert!(app
                    .attr(
                        &Id::Header,
                        Attribute::Custom("launch"),
                        AttrValue::Flag(true)
                    )
                    .is_ok());
            }
        }
    }

    pub fn view(
        &self,
        app: &mut Application<Id, Msg, UserEventIter>,
        terminal: &mut TerminalBridge,
    ) {
        match self {
            Interface::Status => {
                assert!(terminal
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
                        app.view(&Id::Header, f, chunks[0]);
                        app.view(&Id::ServerList, f, chunks[1]);
                        app.view(&Id::Label, f, chunks[3]);

                        let ui_chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .margin(0)
                            .constraints(
                                [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
                            )
                            .split(chunks[2]);
                        app.view(&Id::Preview, f, ui_chunks[1]);

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
                        app.view(&Id::TextInput1, f, input_chunks[0]);
                        app.view(&Id::TextInput2, f, input_chunks[1]);
                        app.view(&Id::TextInput3, f, input_chunks[2]);
                    })
                    .is_ok());
            }
        }
    }

    pub fn change_focus(&self, app: &mut Application<Id, Msg, UserEventIter>) {
        match self {
            Interface::Status => {
                if let Some(current_active) = app.focus() {
                    match current_active {
                        Id::Header => {
                            assert!(app.active(&Id::ServerList).is_ok());
                        }
                        Id::ServerList => {
                            assert!(app.active(&Id::TextInput1).is_ok());
                        }
                        Id::TextInput1 => {
                            assert!(app.active(&Id::TextInput2).is_ok());
                        }
                        Id::TextInput2 => {
                            assert!(app.active(&Id::TextInput3).is_ok());
                        }
                        Id::TextInput3 => {
                            assert!(app.active(&Id::Header).is_ok());
                        }
                        _ => {
                            assert!(app.active(&Id::Header).is_ok());
                        }
                    }
                }
            }
        }
    }
}
