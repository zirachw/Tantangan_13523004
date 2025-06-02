pub struct Style;

impl Style {
    // Colors
    pub const RESET: &'static str = "\x1b[0m";
    pub const BRIGHT_RED: &'static str = "\x1b[91m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &'static str = "\x1b[95m";
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";
    pub const BRIGHT_WHITE: &'static str = "\x1b[97m";

    // Font styles
    pub const BOLD: &'static str = "\x1b[1m";
    pub const ITALIC: &'static str = "\x1b[3m";

    // Helper methods for common UI elements
    pub fn header(text: &str) -> String {
        format!("{}{}{}{}", Self::BOLD, Self::BRIGHT_CYAN, text, Self::RESET)
    }

    pub fn info(text: &str) -> String {
        format!("{}{}{}", Self::BRIGHT_BLUE, text, Self::RESET)
    }

    pub fn warning(text: &str) -> String {
        format!("{}{}{}", Self::BRIGHT_YELLOW, text, Self::RESET)
    }

    pub fn error(text: &str) -> String {
        format!("{}{}{}", Self::BRIGHT_RED, text, Self::RESET)
    }

    pub fn success(text: &str) -> String {
        format!("{}{}{}", Self::BRIGHT_GREEN, text, Self::RESET)
    }

    pub fn prompt(text: &str) -> String {
        format!("{}{}{}{}", Self::BOLD, Self::BRIGHT_MAGENTA, text, Self::RESET)
    }

    pub fn highlight(text: &str) -> String {
        format!("{}{}{}", Self::BRIGHT_WHITE, text, Self::RESET)
    }

    pub fn italic(text: &str) -> String {
        format!("{}{}{}", Self::ITALIC, text, Self::RESET)
    }

    pub fn bold(text: &str) -> String {
        format!("{}{}{}", Self::BOLD, text, Self::RESET)
    }

    pub fn city_name(text: &str) -> String {
        format!("{}{}{}{}", Self::BOLD, Self::BRIGHT_GREEN, text, Self::RESET)
    }

    pub fn number(text: &str) -> String {
        format!("{}{}{}", Self::BRIGHT_YELLOW, text, Self::RESET)
    }

    pub fn arrow() -> String {
        format!("{}{}{}", Self::BRIGHT_CYAN, "→", Self::RESET)
    }
}