use ratatui::{
    buffer::Buffer, 
    layout::{
        Constraint, 
        Direction, 
        Layout, 
        Rect
    }, 
    style::{
        palette::tailwind, Color, Modifier, Style, Stylize
    }, 
    symbols::border::PROPORTIONAL_TALL, text::Line,  
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, Padding, Paragraph, Widget
    }
};
use strum::{Display, EnumIter, FromRepr};
use crate::barchart::{draw_bar_with_group_labels, draw_legend, BarChartApp};
use crate::read_json::read_json;
use crate::list::draw_help;
use std::io::Read;
use std::fs::File;



#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Barplot")]
    Tab1,
    #[strum(to_string = "List Overview")]
    Tab2,
    #[strum(to_string = "Overtime")]
    Tab3,
    #[strum(to_string = "Focus Time")]
    Tab4,
    #[strum(to_string = "Meeting Notes")]
    Tab5,
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    pub fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    pub fn render_tab0(self, area: Rect, buf: &mut Buffer) {
        const LEGEND_HEIGHT: u16 = 6;

        let barchart_app = BarChartApp::new();
        let barchart = draw_bar_with_group_labels(&barchart_app, false, self.block());
        barchart.render(area, buf);


        if area.height >= 20 && area.width >= 50 {
            let legend_width =  "Time (in min) for:".len() as u16 + 4;
            let legend_area = Rect {
                height: LEGEND_HEIGHT,
                width: legend_width,
                y: area.y,
                x: area.right() - legend_width,
            };
            let legend = draw_legend(self.block());
            legend.render(legend_area, buf);
        }       
    }

    pub fn render_tab1(self, area: Rect, buf: &mut Buffer) {
        const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
        match read_json() {
            Ok(worktime) => {
                const HELP_HEIGHT: u16 = 6;
                let list_item: Vec<ListItem> = worktime
                    .iter()
                    .map(|worktime| {
                        ListItem::new(format!(
                            "Date: {}, Starttime: {}, Endtime: {}",
                            worktime.date, worktime.starttime, worktime.endtime
                        )
                    )
                    })
                    .rev()
                    .collect();

                let list = List::new(list_item)
                    .block(self.block())
                    .highlight_style(Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::REVERSED)
                        .fg(SELECTED_STYLE_FG),)
                    .highlight_symbol(">>")
                    .highlight_spacing(HighlightSpacing::Always);

                list.render(area, buf);

                if area.height >= 20 && area.width >= 50 {
                    let help_width =  "Select - Enter".len() as u16 + 4;
                    let help_area = Rect {
                        height: HELP_HEIGHT,
                        width: help_width,
                        y: area.bottom() - HELP_HEIGHT,
                        x: area.right() - help_width,
                    };
                    let help = draw_help(self.block());
                    help.render(help_area, buf);
                }     
                
                
            }
            Err(_e) => {
                Paragraph::new("No history yet!")
                .block(self.block())
                .render(area, buf);
            }
        }     
    }

    pub fn render_tab2(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
            .block(self.block())
            .render(area, buf);
    }


    pub fn render_tab3(self, area: Rect, buf: &mut Buffer) {
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
                    .border_style(self.palette().c700)
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
                    .border_style(self.palette().c700)
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

    pub fn render_tab4(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
            .block(self.block())
            .render(area, buf);
    }


    /// A block surrounding the tab's content
    pub fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Tab1 => tailwind::INDIGO,
            Self::Tab2 => tailwind::EMERALD,
            Self::Tab3 => tailwind::RED,
            Self::Tab4 => tailwind::BLUE,
            Self::Tab5=> tailwind::GREEN,
        }
    }    
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // in a real app these might be separate widgets
        match self {
            Self::Tab1 => self.render_tab0(area, buf),
            Self::Tab2 => self.render_tab1(area, buf),
            Self::Tab3 => self.render_tab2(area, buf),
            Self::Tab4 => self.render_tab3(area, buf),
            Self::Tab5 => self.render_tab4(area, buf),
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