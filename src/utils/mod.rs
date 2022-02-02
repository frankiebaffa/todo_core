pub fn get_printable_coords(nums: &Vec<usize>) -> String {
    nums.into_iter().map(|num| {
        num.to_string()
    }).collect::<Vec<String>>().join(", ").to_string()
}
pub mod styler {
    use crossterm::style::{
        Attribute,
        Color,
        Stylize,
    };
    pub fn primary(msg: impl AsRef<str>) -> String {
        format!("{}", msg.as_ref().with(Color::Blue))
    }
    pub fn success(msg: impl AsRef<str>) -> String {
        format!("{}", msg.as_ref().with(Color::Green))
    }
    pub fn info(msg: impl AsRef<str>) -> String {
        format!("{}", msg.as_ref().with(Color::Cyan))
    }
    pub fn warning(msg: impl AsRef<str>) -> String {
        format!("{}", msg.as_ref().with(Color::Yellow))
    }
    pub fn danger(msg: impl AsRef<str>) -> String {
        format!("{}", msg.as_ref().with(Color::Red))
    }
    pub fn bold(msg: impl AsRef<str>) -> String {
        format!("{}", msg.as_ref().attribute(Attribute::Bold))
    }
    pub fn italic(msg: impl AsRef<str>) -> String {
        format!("{}", msg.as_ref().attribute(Attribute::Italic))
    }
}
