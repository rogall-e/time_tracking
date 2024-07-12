use ratatui::{
    style::{
        Color, 
        Modifier, 
        Style,
    }, 
    text::{
        Line, 
        Span, 
    }, 
    widgets::{
        Block, 
        Paragraph, 
    },
};

 
pub fn draw_help(block: Block<'static>) -> Paragraph<'static> {
    let text = vec![
        Line::from(Span::styled(
            "Navigation:",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White),
        )),
        Line::from(Span::styled(
            "Up - ↑",
            Style::default().add_modifier(Modifier::BOLD)
            .fg(Color::White),
        )),
        Line::from(Span::styled(
            "Down - ↓",
            Style::default().add_modifier(Modifier::BOLD)
            .fg(Color::White),
        )),
        Line::from(Span::styled(
            "Select - Enter",
            Style::default().add_modifier(Modifier::BOLD)
            .fg(Color::White),
        )),
    ];
    
    let paragraph = Paragraph::new(text).block(block);
    paragraph
}