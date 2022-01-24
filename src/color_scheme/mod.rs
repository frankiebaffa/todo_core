use termion::color;
fn fg_colorize<C>(color: color::Fg<C>, message: impl AsRef<str>) -> String
where
    C: color::Color,
{
    format!(
        "{}{}{}",
        color,
        message.as_ref(),
        color::Fg(color::Reset),
    )
}
pub fn primary(message: impl AsRef<str>) -> String {
    fg_colorize(color::Fg(color::Blue), message)
}
pub fn info(message: impl AsRef<str>) -> String {
    fg_colorize(color::Fg(color::Cyan), message)
}
pub fn success(message: impl AsRef<str>) -> String {
    fg_colorize(color::Fg(color::Green), message)
}
pub fn warning(message: impl AsRef<str>) -> String {
    fg_colorize(color::Fg(color::Yellow), message)
}
pub fn danger(message: impl AsRef<str>) -> String {
    fg_colorize(color::Fg(color::Red), message)
}
