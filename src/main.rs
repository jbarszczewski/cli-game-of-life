use clap::{crate_version, App, Arg};
use crossterm::{
	cursor::{Hide, MoveTo, Show},
	event::{poll, read, Event, KeyCode, KeyEvent},
	execute, queue,
	style::{Color, Print, ResetColor, SetForegroundColor},
	terminal::{
		disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap,
		EnterAlternateScreen, LeaveAlternateScreen,
	},
	Result,
};
use std::io::{stdout, Write};
use std::time::Duration;
mod game;

fn main() -> Result<()> {
	let matches = App::new("CLI Game Of Life")
		.version(crate_version!())
		.author("jbarszczewski")
		.about("Simple implementation of Conway's Game Of Life in Rust.")
		.after_help("Have fun!")
		.arg(
			Arg::with_name("INPUT")
				.help("Sets the input file to configure initial state of game")
				.short("i")
				.long("input")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("DELAY")
				.help("Sets the delay between game ticks. Value is in miliseconds")
				.short("d")
				.long("delay")
				.takes_value(true)
				.default_value("500"),
		)
		.get_matches();

	let delay: u64 = matches.value_of("DELAY").unwrap().parse().unwrap();

	let mut game = game::Universe::new(5, 5);
	let mut stdout = stdout();
	game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
	enable_raw_mode()?;
	execute!(
		stdout,
		EnterAlternateScreen,
		SetForegroundColor(Color::Magenta),
		DisableLineWrap,
		Hide
	)?;

	loop {
		queue!(stdout, Clear(ClearType::All))?;
		let mut i = 0;
		while let Some(line) = game.row_as_string(i) {
			queue!(stdout, MoveTo(0, i as u16), Print(line))?;
			i += 1;
		}

		queue!(
			stdout,
			MoveTo(0, (i + 1) as u16),
			Print("Press Esc to exit...")
		)?;
		stdout.flush()?;
		if poll(Duration::from_millis(delay))? {
			if let Event::Key(KeyEvent { code, .. }) = read()? {
				match code {
					KeyCode::Esc => {
						break;
					}
					_ => {}
				}
			}
		}

		game.tick();
	}
	execute!(
		stdout,
		ResetColor,
		EnableLineWrap,
		Show,
		LeaveAlternateScreen
	)?;
	disable_raw_mode()?;
	Ok(())
}
