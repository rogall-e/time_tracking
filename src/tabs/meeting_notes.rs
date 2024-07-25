use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    style::palette::tailwind, 
     
    symbols::border::PROPORTIONAL_TALL,   
    widgets::{
        Block, Padding, Paragraph, Widget
    }
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeetingNotesTab {
    block: Block<'static>,
}

impl MeetingNotesTab {
    pub fn new() -> Self {
        Self {
            block: Block::default(),
        }
    }
    pub fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(tailwind::INDIGO.c700)
    }
}

impl Widget for MeetingNotesTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
            .block(self.block())
            .render(area, buf);    
    }    
}
