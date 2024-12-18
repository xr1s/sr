mod argument;
mod formattable;
mod formatter;

pub use argument::Argument;
pub use formatter::Formatter;

pub fn format(template: &str, arguments: &[Argument]) -> String {
    Formatter::new(template, arguments).format()
}

pub fn format_wiki(template: &str) -> String {
    Formatter::new(template, &[]).output_wiki(true).format()
}
