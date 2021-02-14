---
title: Rust CLI Game of Life tutorial - PART 2
published: false
description: Tutorial showing how to implement Conway's Game of Life as a CLI application written in Rust. Makes use of Crossterm crate which is amazing for working with terminal.
tags: rust,cli,beginner,tutorial
cover_image: https://dev-to-uploads.s3.amazonaws.com/i/f2x3fslw1wc5c920zv6y.jpg
---

# Intro

Welcome to the second part of my Rust CLI tutorial. First one can be found [here](https://dev.to/jbarszczewski/rust-cli-game-of-life-tutorial-part-1-57pp). In this part we will explore how to make our application configurable by adding command line arguments. Plan is to be able to set game Universe from the input file and control speed by passing its value.

"Final" code can be found on my [github repo](https://github.com/jbarszczewski/cli-game-of-life)

Let's start!

# Accept The Args

Simples way to accept arguments is to use function provided by standard library `std::env::args` for very simple situations it will be enough but I want to show you how to easily create rich experience using external crate [clap](https://crates.io/crates/clap). There are 3 different ways to configure `clap` in your application:

- 'Builder Pattern'
- YAML config file
- macros

Personally 'Builder Pattern' is my favourite as it allows you to dynamically create args and offer compile time error check. For simple project like this it's totally fine to put the config in the `main.rs` but as project grow you might consider moving it to separate file for cleaner code and readability.
Time to see this crate in action. First add the dependency in `Cargo.toml`:

```yaml
clap = "2.33.3"
```

and update our `main.rs` file:

```rust
use clap::{crate_version, App, Arg};

//below code goes at the beginning of main() function:
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
```

The `clap` crate creates two subcommands for you (unless you explicitly override them):

- help (-h or --help)
- version (-V --version)
  That's why we provide basic info about the app. You may notice `crate_version!` macro, this will grab the version number from your `Cargo.toml` file so you don't need to manually update it.
  Then we add two arguments, INPUT and DELAY, with some description how to use it. Build your app with `cargo build` (you will find binary in /target/debug directory) and run like this `./cli-game-of-life -h` which will print out help page:

```
CLI Game Of Life 0.2.0
jbarszczewski
Simple implementation of Conway's Game Of Life in Rust.

USAGE:
    cli-game-of-life [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delay <DELAY>    Sets the delay between game ticks. Value is in miliseconds [default: 500]
    -i, --input <INPUT>    Sets the input file to configure initial state of game

Have fun!
```

Now to get the passed values you can use:

```rust
if let Some(input) = matches.value_of("INPUT") {
	println!("A config file was passed: {}", input);
}
```

`value_of()` will return `Option<T>` so you can act accordingly depending if the value exist or not. Notice that our DELAY argument have `default` value set which means we will always have some value to work with.
Now, we won't use it in our application but often you will have flag arguments as well. By default all `clap` arguments are flags that's why we had to add `takes_value()` when describing INPUT and DELAY. Because flags don't have value you can use them like this:

```rust
if matches.is_present("TEST") {
	println!("TEST!");
}
```

There are so many possible configuration option so I strongly advise checking [documentation](https://docs.rs/clap/2.33.3/clap/struct.Arg.html) just be familiar what you can use.

Ok, so we've configured our application to accept arguments, but they don't do anything yet. That will change in a moment

# Control The Speed

First we will make use of our DELAY argument. Right now, our game use hard-coded value of 500ms as a delay between each ticks. Changing that will be super easy. First we need to read and parse (`Duration::from_millis()` accept u64) our argument value:

```rust
let delay:u64 = matches.value_of("DELAY").unwrap().parse().unwrap();
```

We use the first unwrap (which will throw panic if `None` is returned) because we know that there is a default value of `500` in case user didn't pass the delay value, and the second unwrap (which will throw panic if `Err` is returned) because if value is not a valid positive integer we want program to exit. If you want to manualy handle the second error you could use logic like this:

```rust
let delay: u64 = match matches.value_of("DELAY").unwrap().parse() {
	Ok(val) => val,
	Err(e) => {
		println!("Error parsing DELAY argument: {}", e);
		500
	}
};
```

And then we can replace `500` in our `poll` function with the `delay` variable. If you want to test how it works just use command like:
`./cli-game-of-life -d 200` (remember that value is in miliseconds).

There is one small issue. Because of how we wrote our loop, we display game after we check for the user input in the delay. That means if we pass DELAY, e.g. 5000, we will need to wait 5 seconds before anything appear on the screen. We can fix it by moving the "drawing" code out of the if statement:

```rust
// fixed loop code:
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
```

# Define The Universe

Now it's time to make use of the INPUT argument. The value of this one is a path to universe configuration file. File will be in simple text format like this:

```
5
5
00000
00100
00010
01110
00000
```

First line is the number of rows, second is the number of columns and following lines describe each cell, 0 being dead and 1 alive. Now there are two locations you can place the file:

1. In the root of your project, same directory as `Cargo.toml` is, and you can run your project using `cargo run -- -i INPUT`. When using Cargo to run your project everything that is after `--` is passed as a parameters to your project rather than Cargo.
2. In `./target/debug`. That means you need to run `cargo build` after each change and then execute `target/debug/cli-game-of-life -i starship`.

In this tutorial I recommend sticking with first option as it's simpler when developing your application. The above configuration is called `starship` pattern in Game of Life so lets call the file the same and move on to the next steps.

We will be reading the text file so we need to add new imports:

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
```

Below is the function that accept path to the configuration file, reads it and return new `game::Universe`:

```rust
fn create_game_from_file(path: &str) -> game::Universe {
	let file = File::open(path).unwrap();
	let mut reader = BufReader::new(file);
	let mut line = String::new();
	let mut rows_number = 0;
	if let Ok(success) = reader.read_line(&mut line) {
		if success > 0 {
			rows_number = line.trim().parse().unwrap();
			line.clear();
		} else {
			panic!("Rows number not detected!");
		}
	};
	let mut cols_number = 0;
	if let Ok(success) = reader.read_line(&mut line) {
		if success > 0 {
			cols_number = line.trim().parse().unwrap();
			line.clear();
		} else {
			panic!("Columns number not detected!");
		}
	};
	let mut game_universe = game::Universe::new(cols_number, rows_number);
	let mut row = 0;
	let mut live_cells = Vec::<(u32, u32)>::new();
	loop {
		match reader.read_line(&mut line) {
			Ok(0) => break,
			Ok(_) => {
				let mut col = 0;
				for char in line.chars() {
					match char {
						'1' => live_cells.push((row, col)),
						_ => {}
					}
					col += 1;
				}
			}
			_ => break,
		}

		line.clear();
		row += 1;
	}
	game_universe.set_cells(&live_cells);
	game_universe
}
```

It might seems long and for sure there is a room for some refactoring but should be easy to understand what is happening:

1. We open the file and we pass it to the BufReader which is optimized for many subsequent reads from the same source.
2. We create new mutable String called `line` which will be reused for each line we read from the file.
3. We try to read line and parse it to number of rows and collumns. `reader.read_line(&mut line)` Returns `Result<usize>` which when success will contain number of bytes read. If it reach the end of file it will return 0. Note that we need to call `.trim()` before parsing the value as `read_line` also returns end of line character. We also need to call `clear()` on our String as the `read_line` is appending to string rather than replacing it content.
4. We create new Universe with the specified size and new vector that will hold our live cells coordinates.
5. Loop that iterates through remaining lines, detects live cells and push them into the `live_cells` vector.
6. We pass call `set_cells` on our `game_universe` and return it afterwards.

Last thing that we need to do is make use of our new function. In `main()` delete the lines where we initialize new game (there will be two of them, one that creates new struct, the other that sets live cells), and place this code just under our `delay` variable initialization:

```rust
let mut game = match matches.value_of("INPUT") {
	Some(path) => create_game_from_file(path),
	None => {
		let mut default_game = game::Universe::new(5, 5);
		default_game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
		default_game
	}
};
```

This is simple one: we try to read `INPUT` argument, if one is passed then we call `create_game_from_file` otherwise we fallback to our default universe.

Now we are ready to call `cargo run -- -i starship` and enjoy the view! You might want to create bigger universe than 5x5 as this new pattern is a moving, try something like 15x15, and because we don't validate line input lenght you don't need to add trailing `0` in each line.

# Conclusion

We covered two topics in this tutorial: using command line arguments and reading a file. Both might sound simple but are really important when creating any CLI application.
I hope you've enjoyed this tutorial and as always if any suggestions/questions don't hesitate to leave a comment below.

Thanks for reading and till the next time!
