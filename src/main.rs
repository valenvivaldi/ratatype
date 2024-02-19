use std::io::{stdout, Error};

mod phrase;
use phrase::Phrase;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::Flex,
    prelude::*,
    style::Style,
    widgets::{block::Position, *},
    Frame, Terminal,
};

fn main() -> Result<(), Error> {
    enable_raw_mode()?; // no line buffering
    stdout().execute(EnterAlternateScreen);
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;
    let mut is_correct: Option<bool> = None;

    let mut phrase = Phrase::new();

    while !should_quit {
        terminal.draw(|f| ui(f, &phrase))?;
        (is_correct, should_quit) = handle_event(&phrase).unwrap();
        phrase.update(is_correct);
    }
    disable_raw_mode();
    stdout().execute(LeaveAlternateScreen);

    Ok(())
}

// returns 'should_quit'
pub fn handle_event(phrase: &Phrase) -> Result<(Option<bool>, bool), Error> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(k) = event::read()? {
            if k.kind == event::KeyEventKind::Press {
                let chars: Vec<char> = phrase.phrase.chars().collect();
                let cur_char: char = *chars.get(phrase.char_ptr).unwrap();
                let char_kc = KeyCode::Char(cur_char);
                match k.code {
                    KeyCode::Char('q') => {
                        return Ok((Some(false), true));
                    }
                    KeyCode::Char(c) => {
                        if c == cur_char {
                            return Ok((Some(true), false));
                        }
                        return Ok((Some(false), false));
                    }
                    _ => {
                        return Ok((Some(false), false));
                    }
                }
            }
        }
    }
    return Ok((None, false));
}

pub fn ui(frame: &mut Frame, phrase: &Phrase) {
    let a: Text = Text::from(Span::styled("jejeje", Style::new().fg(Color::Red)));

    let main_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::Percentage(30)],
    )
    .flex(Flex::Center)
    .split(frame.size());
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title("ratatype".white().to_centered_line())
            .bold()
            .white(),
        main_layout[0],
    );

    let inner_layout = Layout::new(Direction::Vertical, [Constraint::Percentage(100)])
        .flex(Flex::Center)
        .split(main_layout[1]);
    frame.render_widget(
        Paragraph::new(colored_text(phrase)).block(Block::new().borders(Borders::ALL)),
        inner_layout[0],
    );
}

pub fn colored_text(phrase: &Phrase) -> Text {
    let mut colored_chars: Vec<Span> = Vec::new();
    let phrase_chars: Vec<char> = phrase.phrase.chars().collect();
    for i in 0..phrase.phrase.len() {
        let phrase_char = phrase_chars.get(i).unwrap();
        if i < phrase.colors.len() {
            let correct = phrase.colors.get(i).unwrap();
            colored_chars.push(Span::styled(
                phrase_char.to_string().clone(),
                Style::new().bg(if *correct {
                    Color::LightGreen
                } else {
                    Color::Red
                }),
            ));
        } else {
            colored_chars.push(Span::raw(phrase_char.to_string().clone()));
        }
    }
    let line = Line::from(colored_chars);
    Text::from(vec![line])
}

/*
 *how to create the cursor?
how to make the cursor move and also to change the color of each lett
er when pressed?

maybe: create logic OUTSIDE ratatui/crossterm to generate the entire
PHRASES.
           maybe we can have a cursor for that specific PHRASE to see
 in which letter we currently are,
           and with the event handlers move the cursor and change the
 colors.
           when the phrase ends, start again (generate new phrase)

 * */