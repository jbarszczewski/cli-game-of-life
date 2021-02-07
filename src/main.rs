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
use std::fs::File;
use std::io::{stdout, Write};
use std::io::{BufRead, BufReader};
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

	let mut stdout = stdout();
	let delay: u64 = matches.value_of("DELAY").unwrap().parse().unwrap();

	let mut game = match matches.value_of("INPUT") {
		Some(path) => create_game_from_file(path),
		None => {
			let mut default_game = game::Universe::new(5, 5);
			default_game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
			default_game
		}
	};

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

fn create_game_from_file(path: &str) -> game::Universe {
	let file = File::open(path).unwrap();
	let mut reader = BufReader::new(file);
	let mut rows = String::default();
	let mut rows_number = 0;
	if let Ok(success) = reader.read_line(&mut rows) {
		if success > 0 {
			rows.pop();
			rows_number = rows.parse().unwrap();
		} else {
			panic!("Rows number not detected!");
		}
	};

	let mut cols = String::default();
	let mut cols_number = 0;
	if let Ok(success) = reader.read_line(&mut cols) {
		if success > 0 {
			cols.pop();
			cols_number = cols.parse().unwrap();
		} else {
			panic!("Columns number not detected!");
		}
	};
	let mut game_universe = game::Universe::new(cols_number, rows_number);
	let mut row = 0;
	loop {
		let mut line = String::default();
		match reader.read_line(&mut line) {
			Ok(0) => break,
			Ok(_) => {
				let mut col = 0;
				for char in line.chars() {
					match char {
						'1' => game_universe.set_cells(&[(row, col)]),
						_ => {}
					}
					col += 1;
				}
			}
			_ => break,
		}

		row += 1;
	}

	game_universe
}
