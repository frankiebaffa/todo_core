use {
    crate::traits::Terminal,
    crossterm::style::{
        Attribute,
        Color,
        Stylize,
    },
    std::io::Error as IOError,
};
pub fn get_printable_coords(nums: &Vec<usize>) -> String {
    nums.into_iter().map(|num| {
        num.to_string()
    }).collect::<Vec<String>>().join(", ").to_string()
}
pub fn primary(msg: impl AsRef<str>) -> String {
    format!("{}", msg.as_ref().with(Color::Blue))
}
pub fn info(msg: impl AsRef<str>) -> String {
    format!("{}", msg.as_ref().with(Color::Cyan))
}
pub fn warning(msg: impl AsRef<str>) -> String {
    ctx.write_str(format!("{}", msg.as_ref().with(Color::Yellow)))
}
pub fn danger(ctx: &mut impl Terminal, msg: impl AsRef<str>) -> Result<(), IOError> {
    ctx.write_str(format!("{}", msg.as_ref().with(Color::Red)))
}
