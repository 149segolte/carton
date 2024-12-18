use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{Application, AttrValue, Attribute, Sub, SubClause, SubEventClause};

use crate::components::container::{Header, Preview};
use crate::components::input::TextInput;
use crate::components::label::TextLabel;
use crate::components::paragraph::ServerListDisconnected;
use crate::components::phantom::PhantomHandler;
use crate::components::table::ServerListConnected;
use crate::constants::{
    Components, Id, InputId, InterfaceMsg, Msg, ProviderStatus, ServerHandle, ServerListStatus,
    UserEvent, UserEventIter,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Interface {
    #[default]
    Status,
    Create,
}

impl Interface {
    fn mount(&self, app: &mut Application<Id, Msg, UserEventIter>, id: Id, component: Components) {
        let subs = match component {
            Components::Header(_) => vec![
                Sub::new(
                    SubEventClause::User(UserEventIter::new(vec![UserEvent::ProviderStatus(
                        ProviderStatus::default(),
                    )])),
                    SubClause::Always,
                ),
                Sub::new(
                    SubEventClause::User(UserEventIter::new(vec![UserEvent::Refresh])),
                    SubClause::Always,
                ),
            ],
            Components::ServerListConnected(_) => {
                vec![Sub::new(
                    SubEventClause::User(UserEventIter::new(vec![UserEvent::ServerListStatus(
                        ServerListStatus::default(),
                    )])),
                    SubClause::Always,
                )]
            }
            Components::TextLabel(_) => vec![Sub::new(
                SubEventClause::User(UserEventIter::new(vec![UserEvent::Error(String::new())])),
                SubClause::Always,
            )],
            _ => Vec::default(),
        };
        assert!(app.mount(id.clone(), component.unwrap(), subs).is_ok());
    }

    pub fn init(
        &self,
        app: &mut Application<Id, Msg, UserEventIter>,
        terminal: &mut TerminalBridge,
    ) {
        match self {
            Interface::Status => {
                app.umount_all();
                self.view(app, terminal);
                // Mount handler
                PhantomHandler::mount(app);

                self.mount(app, Id::Header, Components::Header(Header::default()));
                self.mount(
                    app,
                    Id::ServerList,
                    Components::ServerListDisconnected(ServerListDisconnected::default()),
                );
                self.mount(
                    app,
                    Id::Preview,
                    Components::ServerPreview(Preview::default()),
                );
                self.mount(app, Id::Label, Components::TextLabel(TextLabel::default()));

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
            Interface::Create => {
                app.umount_all();
                self.view(app, terminal);
                // Mount handler
                PhantomHandler::mount(app);

                self.mount(app, Id::Header, Components::Header(Header::default()));
                self.mount(
                    app,
                    Id::ServerList,
                    Components::ServerListDisconnected(ServerListDisconnected::default()),
                );

                self.mount(
                    app,
                    Id::CreateServer1,
                    Components::TextInput(TextInput::new(InputId::CreateServerName, " Name ")),
                );
                self.mount(
                    app,
                    Id::CreateServer2,
                    Components::TextInput(TextInput::new(InputId::CreateServerType, " Type ")),
                );
                self.mount(
                    app,
                    Id::CreateServer3,
                    Components::TextInput(TextInput::new(InputId::CreateServerImage, " Image ")),
                );

                self.mount(app, Id::Label, Components::TextLabel(TextLabel::default()));

                // Activate CreateServer1
                assert!(app.active(&Id::CreateServer1).is_ok());
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
                        app.view(&Id::Preview, f, chunks[2]);
                        app.view(&Id::Label, f, chunks[3]);
                    })
                    .is_ok());
            }
            Interface::Create => {
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

                        let sub_chunks = Layout::default()
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
                            .split(chunks[2]);

                        app.view(&Id::CreateServer1, f, sub_chunks[0]);
                        app.view(&Id::CreateServer2, f, sub_chunks[1]);
                        app.view(&Id::CreateServer3, f, sub_chunks[2]);

                        app.view(&Id::Label, f, chunks[3]);
                    })
                    .is_ok())
            }
        }
    }

    pub fn change_focus(&self, app: &mut Application<Id, Msg, UserEventIter>) -> Option<Msg> {
        match self {
            Interface::Status => {
                if let Some(current_active) = app.focus() {
                    match current_active {
                        Id::Header => {
                            assert!(app.active(&Id::ServerList).is_ok());
                            None
                        }
                        Id::ServerList => {
                            assert!(app.active(&Id::Preview).is_ok());
                            None
                        }
                        Id::Preview => {
                            assert!(app.active(&Id::Header).is_ok());
                            None
                        }
                        _ => {
                            assert!(app.active(&Id::Header).is_ok());
                            None
                        }
                    }
                } else {
                    None
                }
            }
            Interface::Create => {
                if let Some(current_active) = app.focus() {
                    match current_active {
                        Id::Header => {
                            assert!(app.active(&Id::ServerList).is_ok());
                            None
                        }
                        Id::ServerList => {
                            assert!(app.active(&Id::CreateServer1).is_ok());
                            None
                        }
                        Id::CreateServer1 => {
                            assert!(app.active(&Id::CreateServer2).is_ok());
                            None
                        }
                        Id::CreateServer2 => {
                            assert!(app.active(&Id::CreateServer3).is_ok());
                            None
                        }
                        Id::CreateServer3 => {
                            assert!(app.active(&Id::Header).is_ok());
                            None
                        }
                        _ => {
                            assert!(app.active(&Id::Header).is_ok());
                            None
                        }
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn perform(
        &self,
        app: &mut Application<Id, Msg, UserEventIter>,
        msg: InterfaceMsg,
    ) -> Option<Msg> {
        match msg {
            InterfaceMsg::Connected => {
                if app.mounted(&Id::ServerList) {
                    assert!(app.umount(&Id::ServerList).is_ok());
                }

                self.mount(
                    app,
                    Id::ServerList,
                    Components::ServerListConnected(ServerListConnected::default()),
                );

                Some(Msg::FetchServers)
            }
            InterfaceMsg::Disconnected => {
                if app.mounted(&Id::ServerList) {
                    assert!(app.umount(&Id::ServerList).is_ok());
                }

                self.mount(
                    app,
                    Id::ServerList,
                    Components::ServerListDisconnected(ServerListDisconnected::default()),
                );

                None
            }
            InterfaceMsg::SelectedServer(server) => {
                match server {
                    ServerHandle::Create => unreachable!(),
                    other => {
                        if app.mounted(&Id::Preview) {
                            assert!(app.umount(&Id::Preview).is_ok());
                        }

                        self.mount(
                            app,
                            Id::Preview,
                            Components::ServerPreview(Preview::new(other.to_preview().unwrap())),
                        );
                    }
                }

                None
            }
        }
    }
}
