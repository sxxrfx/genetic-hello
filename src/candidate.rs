use std::fmt::Display;

use anes::{Color, SetForegroundColor};

#[derive(Debug, Clone)]
pub struct Candidate {
    text: String,
    fitness: i32,
    in_focus: Focus,
    visibility: Visibility,
    fitness_view: FitnessView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    On,
    Off,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Invisible,
    Visible,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitnessView {
    On,
    Off,
}

impl Candidate {
    pub fn default(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            fitness: -1,
            in_focus: Focus::Off,
            visibility: Visibility::Invisible,
            fitness_view: FitnessView::Off,
        }
    }
    pub fn new(
        text: &str,
        in_focus: Focus,
        visiblility: Visibility,
        fitness_view: FitnessView,
    ) -> Self {
        Self {
            text: text.to_owned(),
            fitness: -1,
            in_focus: Focus::Off,
            visibility: Visibility::Invisible,
            fitness_view: FitnessView::Off,
        }
    }
    pub fn toggle_focus(&mut self) {
        match self.in_focus {
            Focus::On => {
                self.in_focus = Focus::Off;
            }
            Focus::Off => {
                self.in_focus = Focus::On;
            }
        }
    }

    pub fn toggle_visibility(&mut self) {
        match self.visibility {
            Visibility::Invisible => {
                self.visibility = Visibility::Visible;
            }
            Visibility::Visible => {
                self.visibility = Visibility::Invisible;
            }
        }
    }
    pub fn toggle_fitness_view(&mut self) {
        match self.fitness_view {
            FitnessView::On => {
                self.fitness_view = FitnessView::Off;
            }
            FitnessView::Off => {
                self.fitness_view = FitnessView::On;
            }
        }
    }

    pub fn set_focus(&mut self, focus: Focus) {
        self.in_focus = focus;
    }
    pub fn set_visibility(&mut self, visiblility: Visibility) {
        self.visibility = visiblility;
    }

    pub fn set_fitness_view(&mut self, fitness_view: FitnessView) {
        self.fitness_view = fitness_view;
    }

    pub fn display_str(&self, target_str: &str) -> String {
        let prefix = if self.in_focus == Focus::On {
            "➤ "
        } else {
            "  "
        };
        if self.visibility == Visibility::Invisible {
            return format!(
                "{}{}",
                prefix,
                std::iter::repeat(" ")
                    .take(target_str.len())
                    .collect::<String>()
            );
        }
        if self.fitness < 0 || self.fitness_view == FitnessView::Off {
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
    pub fn fitness(&self) -> i32 {
        self.fitness
    }

    pub fn text(&self) -> String {
        self.text.clone()
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

impl Display for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
