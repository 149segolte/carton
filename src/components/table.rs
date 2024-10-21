use tui_realm_stdlib::Table;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent};

use crate::constants::{Msg, ServerHandle, ServerListStatus, UserEvent, UserEventIter};

#[derive(MockComponent)]
pub struct ServerListConnected {
    component: Table,
    servers: Vec<ServerHandle>,
}

impl Default for ServerListConnected {
    fn default() -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Yellow),
                )
                .title("Servers List", Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::LightYellow)
                .highlighted_str(">")
                .rewind(true)
                .step(4)
                .row_height(1)
                .headers(&["No", "Name", "Status", "IP"])
                .table(
                    TableBuilder::default()
                        .add_col(TextSpan::new(""))
                        .add_col(TextSpan::new("No servers detected"))
                        .add_col(TextSpan::new(""))
                        .add_col(TextSpan::new(""))
                        .add_row()
                        .add_col(TextSpan::new("+"))
                        .add_col(TextSpan::new("Create a new server"))
                        .add_col(TextSpan::new(""))
                        .add_col(TextSpan::new(""))
                        .add_row()
                        .build(),
                ),
            servers: Vec::default(),
        }
    }
}

impl ServerListConnected {
    fn update_status(&mut self, status: ServerListStatus) {
        self.servers = status.servers;
        let mut table = TableBuilder::default();
        self.servers.iter().enumerate().for_each(|(index, server)| {
            let status = server.to_status();
            table
                .add_col(TextSpan::new(format!("{}", index + 1)))
                .add_col(TextSpan::new(&status.name))
                .add_col(TextSpan::new(&status.status))
                .add_col(TextSpan::new(&status.ip))
                .add_row();
        });
        table
            .add_col(TextSpan::new("+"))
            .add_col(TextSpan::new("Create a new server"))
            .add_col(TextSpan::new(""))
            .add_col(TextSpan::new(""))
            .add_row();
        self.component
            .attr(Attribute::Content, AttrValue::Table(table.build()))
    }
}

impl Component<Msg, UserEventIter> for ServerListConnected {
    fn on(&mut self, ev: Event<UserEventIter>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Cmd::Move(Direction::Down),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => Cmd::Scroll(Direction::Down),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => Cmd::Scroll(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => Cmd::GoTo(Position::Begin),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => Cmd::GoTo(Position::End),
            Event::User(UserEventIter { events }) => {
                for ev in events {
                    if let UserEvent::ServerListStatus(status) = ev {
                        self.update_status(status);
                    }
                }
                return Some(Msg::Nop);
            }
            _ => Cmd::None,
        };

        self.perform(cmd);
        Some(Msg::Nop)
    }
}
