use std::fmt;

const INDENT: usize = 2;
const COL_WIDTH: usize = 22;

#[derive(Default, Debug)]
pub(super) struct Banner {
    status: String,
    title: String,
    value: String,
}

impl Banner {
    pub fn new(status: &str, title: &str, value: &str) -> Self {
        Banner {
            status: status.to_string(),
            title: title.to_string(),
            value: value.to_string(),
        }
    }
}

impl fmt::Display for Banner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\u{0020}{:\u{0020}<indent$} {:\u{0020}<width$}\u{2502}\u{0020}{}", self.status, self.title, self.value, indent = INDENT, width = COL_WIDTH)
    }
}