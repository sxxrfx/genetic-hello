use anes::{Color, SetForegroundColor};

#[derive(Debug, Clone)]
pub struct Candidate {
    text: String,
    fitness: i32,
    in_focus: bool,
}

impl Candidate {
    pub fn default(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            fitness: -1,
            in_focus: false,
        }
    }
    pub fn new(text: &str, in_focus: bool) -> Self {
        Self {
            text: text.to_owned(),
            fitness: -1,
            in_focus,
        }
    }
    pub fn toggle_in_focus(&mut self) {
        self.in_focus = !self.in_focus;
    }

    pub fn display_str(&self, target_str: &str) -> String {
        let prefix = if self.in_focus { "➤ " } else { "  " };
        if self.fitness < 0 {
            return format!("{}{}", prefix, self.text);
        }
        let mut out = prefix.to_string();

        for (char, target_char) in self.text.chars().zip(target_str.chars()) {
            if char == target_char {
                out = format!(
                    "{}{}{}",
                    out,
                    SetForegroundColor(Color::Green),
                    char.to_string()
                );
            } else {
                out = format!(
                    "{}{}{}",
                    out,
                    SetForegroundColor(Color::Red),
                    char.to_string()
                );
            }
        }

        format!("{}{}", out, SetForegroundColor(Color::Default))
    }

    pub fn set_fitness(&mut self, target_str: &str) {
        self.fitness = self
            .text
            .chars()
            .zip(target_str.chars())
            .filter(|(a, b)| a == b)
            .count() as i32;
    }
}
