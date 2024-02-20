use rand::prelude::*;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub struct Phrase {
    pub queue: (String, String),
    pub colors: Vec<bool>,
    pub char_ptr: usize,
}

fn gen_phrase() -> String {
    let mut rng = thread_rng();
    let mut words = [
        "sam",
        "frodo",
        "gondor",
        "comarca",
        "fakita",
        "nolocasesacolon",
    ];
    words.shuffle(&mut rng);
    let mut w = String::new();
    for word in words {
        w.push_str(format!("{} ", word).as_str());
    }
    w
}

impl Phrase {
    pub fn new() -> Phrase {
        let p = Phrase {
            queue: (String::from(gen_phrase()), String::from(gen_phrase())),
            colors: Vec::new(),
            char_ptr: 0,
        };
        p
    }

    pub fn reset(&mut self) {
        // move queue.1 (pending) to queue.0 (current)
        self.queue.0.clear();
        self.queue.0.push_str(self.queue.1.as_str());

        // gen new queue.1
        self.queue.1.clear();
        self.queue.1.push_str(&gen_phrase());
        self.char_ptr = 0;
        self.colors = Vec::new();
    }

    pub fn update(&mut self, correct: Option<bool>) {
        if let Some(v) = correct {
            self.colors.push(v);
            self.char_ptr += 1;
        }

        // check if ptr reached end of phrase
        if self.char_ptr == self.queue.0.len() {
            self.reset();
        }
    }
}
