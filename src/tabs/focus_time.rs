use ratatui::{
    buffer::Buffer, 
    layout::{
        Constraint, 
        Direction, 
        Layout, 
        Rect
    }, 
    style::{
        palette::tailwind, Color, Style, Stylize
    }, 
    symbols::border::PROPORTIONAL_TALL, 
    widgets::{
        Block, BorderType, Borders,  Padding, Paragraph, Widget
    }
};
use std::io::Read;
use std::fs::File;



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FocusTimeTab{
    row_index: usize,
}


impl FocusTimeTab {
    pub fn new() -> Self {
        Self {
            row_index: 0,
        }
    }
    pub fn previous(&mut self) {
        self.row_index = self.row_index.saturating_sub(1);
    }

    pub fn next(&mut self) {
        self.row_index = self.row_index.saturating_add(1);
    }

    pub fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(tailwind::BLUE.c700)
    }
}

impl Widget for FocusTimeTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut focus_time_cache = File::open(".tmp_cache/focus_cache.bin").unwrap();
        let mut lines = String::new();
        focus_time_cache.read_to_string(&mut lines).unwrap();

        let focus = lines.split(",").collect::<Vec<&str>>()[0].parse::<bool>().unwrap();
        let focus_time = lines.split(",").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

        Paragraph::new("")
            .block(self.block())
            .render(area, buf);

        let inner_focus_chunck = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let inner_focus_chunks_top = centered_area(50, 50, inner_focus_chunck[0]);
        let inner_focus_chunks_bottom = centered_area(50, 50, inner_focus_chunck[1]);

        if !focus {
            let text = "No focus time!".to_string();
            Paragraph::new("")
                .centered()
                .block(Block::default()
                    .title("")
                    .borders(Borders::NONE)
                    .border_set(PROPORTIONAL_TALL)
                    .padding(Padding::horizontal(1))
                    .border_style(tailwind::BLUE.c700)
                    .on_red()
                    .border_type(BorderType::Rounded)
                )
                .render(inner_focus_chunks_top,  buf);
            Paragraph::new(text)
                .style(Style::default().fg(Color::Red))
                .centered()
                .block(Block::default().title("").borders(Borders::NONE))
                .render(inner_focus_chunks_bottom,  buf);
            
        } else {
            Paragraph::new("")
                .centered()
                .block(Block::default()
                    .title("")
                    .borders(Borders::NONE)
                    .border_set(PROPORTIONAL_TALL)
                    .padding(Padding::horizontal(1))
                    .border_style(tailwind::BLUE.c700)
                    .on_blue()
                    .border_type(BorderType::Rounded)
                )
                .render(inner_focus_chunks_top, buf);
    
            Paragraph::new("Focus time: ".to_string() + focus_time.to_string().as_str() + " min")
                .centered()
                .style(Style::default().fg(Color::Blue))
                .block(Block::default().title("").borders(Borders::NONE))
                .render(inner_focus_chunks_bottom,  buf);

        }
    }  
}

fn centered_area(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}