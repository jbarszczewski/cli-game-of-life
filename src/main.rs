use crossterm::{
    cursor,
    event::{read, Event},
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".magenta()))?;
            }
        }
    }
    stdout.flush()?;

    match read()? {
        Event::Key(event) => println!("{:?}", event),
        Event::Mouse(event) => println!("{:?}", event),
        Event::Resize(width, height) => println!("New size {}x{}", width, height),
    }

    Ok(())
}
