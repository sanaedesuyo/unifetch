use crate::cli::args::DisplayStyle;

pub trait Component {
    fn get_info(&self, style: DisplayStyle) -> String;
}