use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    Result,
};
use std::io::stdout;
use std::time::Duration;
mod game;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    execute!(
        stdout(),
        EnterAlternateScreen,
        SetForegroundColor(Color::Magenta),
        DisableLineWrap,
        Hide
    )?;

    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        } else {
            execute!(
                stdout(),
                Clear(ClearType::All),
                MoveTo(0, 0),
                Print(&game),
                Print("Press enter to exit................")
            )?;
            game.tick();
        }
    }
    execute!(
        stdout(),
        ResetColor,
        EnableLineWrap,
        Show,
        LeaveAlternateScreen
    )?;
    disable_raw_mode()?;
    Ok(())
}
