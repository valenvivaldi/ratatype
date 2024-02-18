use ratatui::prelude::*;
use ratatui::widgets::*;

pub struct Phrase {
    pub phrase: String,
    pub colors: Vec<bool>,
    pub char_ptr: usize,
}

fn gen_phrase() -> String {
    let words = ["sam", "frodo", "gondor"];
    let mut w = String::new();
    for word in words {
        w.push_str(format!("{} ", word).as_str());
    }
    w
}

impl Phrase {
    pub fn new() -> Phrase {
        let mut p = Phrase {
            phrase: String::new(),
            colors: Vec::new(),
            char_ptr: 0,
        };
        p.reset();
        p
    }

    pub fn reset(&mut self) {
        self.phrase.clear();
        self.phrase.push_str(&gen_phrase());
        self.char_ptr = 0;
        self.colors = Vec::new();
    }

    pub fn update(&mut self, correct: Option<bool>) {
        if let Some(v) = correct {
            self.colors.push(v);
            self.char_ptr += 1;
        }

        // check if ptr == len(self.phrase)
        // if so, reset
        if self.char_ptr == self.phrase.len() + 1 {
            self.reset();
        }
    }
}
