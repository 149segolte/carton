use tuirealm::props::{Alignment, Borders, Color, Style};
use tuirealm::tui::widgets::Block;

pub fn get_block<'a>(props: Borders, title: (String, Alignment), focus: bool) -> Block<'a> {
    Block::default()
        .borders(props.sides)
        .border_style(match focus {
            true => props.style(),
            false => Style::default().fg(Color::Reset).bg(Color::Reset),
        })
        .border_type(props.modifiers)
        .title(title.0)
        .title_alignment(title.1)
}
