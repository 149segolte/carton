use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::{Component, Event, MockComponent};

use crate::constants::{Msg, UserEventIter};

#[derive(MockComponent)]
pub struct ServerList {
    component: List,
}

impl Default for ServerList {
    fn default() -> Self {
        Self {
            component: List::default()
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
                .rows(
                    TableBuilder::default()
                        .add_col(TextSpan::from("01").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit"))
                        .add_row()
                        .add_col(TextSpan::from("02").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Maecenas tincidunt dui ut gravida fringilla"))
                        .add_row()
                        .add_col(TextSpan::from("03").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Duis est neque, fringilla sit amet enim id, congue hendrerit mauris"))
                        .add_row()
                        .add_col(TextSpan::from("04").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Nulla facilisi. Vestibulum tincidunt tempor orci, in pellentesque lacus placerat id."))
                        .add_row()
                        .add_col(TextSpan::from("05").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Integer at nisl scelerisque, egestas ipsum in, iaculis tellus. Pellentesque tincidunt vestibulum nisi, ut vehicula augue scelerisque at"))
                        .add_row()
                        .add_col(TextSpan::from("06").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Quisque quis tincidunt tellus. Nam accumsan leo non nunc finibus feugiat."))
                        .add_row()
                        .add_col(TextSpan::from("07").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("non lacus ac orci fermentum aliquam ut feugiat libero. Suspendisse eget nunc in erat molestie egestas eu at massa"))
                        .add_row()
                        .add_col(TextSpan::from("08").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Donec feugiat dui quis libero ornare, vel sodales mauris ornare."))
                        .add_row()
                        .add_col(TextSpan::from("09").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Aenean tempor porta nisi, at sodales eros semper ut. Vivamus sit amet commodo risus"))
                        .add_row()
                        .add_col(TextSpan::from("10").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Etiam urna nisi, ullamcorper at justo et, rhoncus pellentesque dui. Nunc ante velit, ultrices a ornare sit amet, sagittis in ex. Nam pulvinar tellus tortor. Praesent ac accumsan nunc, ac consectetur nisi."))
                        .add_row()
                        .add_col(TextSpan::from("11").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Proin non elit fermentum, pretium diam eget, facilisis mi"))
                        .add_row()
                        .add_col(TextSpan::from("12").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Duis suscipit nibh lacus, quis porta enim accumsan vel"))
                        .add_row()
                        .add_col(TextSpan::from("13").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Etiam volutpat magna tortor, a laoreet ex accumsan sit amet"))
                        .build()
                )
                .selected_line(2),
        }
    }
}

impl Component<Msg, UserEventIter> for ServerList {
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
            _ => Cmd::None,
        };

        self.perform(cmd);
        Some(Msg::Nop)
    }
}
