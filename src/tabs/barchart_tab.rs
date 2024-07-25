use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    style::palette::tailwind, 
    symbols::border::PROPORTIONAL_TALL,   
    widgets::{
        Block, Padding, Widget
    }
};
use crate::barchart::{draw_bar_with_group_labels, draw_legend, BarChartApp};


#[derive(Clone)]
pub struct BarChartTab {
    barchart_app: BarChartApp<'static>,
}

impl BarChartTab {
    pub fn new() -> Self {
        Self {
            barchart_app: BarChartApp::new(),
        }
    }
}

impl Widget for BarChartTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        const LEGEND_HEIGHT: u16 = 6;
    
        let barchart_app = BarChartApp::new();
        let barchart = draw_bar_with_group_labels(
                &barchart_app,
                false,
                Block::bordered()
                    .border_set(PROPORTIONAL_TALL)
                    .padding(Padding::horizontal(1))
                    .border_style(tailwind::INDIGO.c700)  
            );
        barchart.render(area, buf);
    
    
        if area.height >= 20 && area.width >= 50 {
            let legend_width =  "Time (in min) for:".len() as u16 + 4;
            let legend_area = Rect {
                height: LEGEND_HEIGHT,
                width: legend_width,
                y: area.y,
                x: area.right() - legend_width,
            };
            let legend = draw_legend(
                Block::bordered()
                    .border_set(PROPORTIONAL_TALL)
                    .padding(Padding::horizontal(1))
                    .border_style(tailwind::INDIGO.c700)
            );
            legend.render(legend_area, buf);
        }       
    }
}
