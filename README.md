---
title: Rust CLI Game of Life tutorial.
published: false
description: Tutorial showing how to implement Conway's Game of Life as a CLI application written in Rust.
tags: rust,cli,beginner,tutorial
//cover_image: https://direct_url_to_image.jpg
---

# Intro

Hi! If you're here that means you are curious about Rust and/or want to learn it. I've written my first Rust tutorial, [Rust + Actix + CosmosDB (MongoDB) tutorial api](https://dev.to/jbarszczewski/rust-actix-cosmosdb-mongodb-tutorial-api-17i5), back in June 2020. This time I've decided I will try to cover another use case for Rust which is CLI. To make it more interesting it will be implementation of Game of Life based on [Official Rust WebAssembly tutorial](https://rustwasm.github.io/docs/book/game-of-life/rules.html) enhanced with some user interaction logic.

As previously I highly recommend going through official [rustlings tutorial](https://github.com/rust-lang/rustlings).

"Final" code can be found on my [github repo](https://github.com/jbarszczewski/cli-game-of-life)

Let's start!

# Create The Universe

After creating new project something like `cargo new cli-game-of-life` (or `cargo init` if you're already in correct directory) open your favourite editor and... ignore main.rs for now. We're gonna create game logic module first, so go ahead and create a new file `src/game.rs`. As mentioned in before, I will base the logic on official wasm tutorial so if you've done it before it will be very familiar. Let's start with defining an enum that will represent single cell in our game universe:

```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
	Dead = 0,
	Alive = 1,
}
```

`derive` attribute will tell the compiler to provide basic implementation of passed traits so that we can assign cells with enum values and compare them.

**Note:** We could use simple bool value as well but enum will give us better readability while having the same memory footprint.

Our game universe is defined as follows:

```rust
pub struct Universe {
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}
```

Now we can start implementing functions for our game. Let's start with a handy constructor that will initialize Universe with given size and assign Cells starting values and `set_cells` function that will accept an array of cells coordinates and set them to Alive state.

```rust
impl Universe {
	pub fn new(width: u32, height: u32) -> Universe {
		Universe {
			width: width,
			height: height,
			cells: vec![Cell::Dead; (width * height) as usize],
		}
	}

	pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
		for (row, col) in cells.iter().cloned() {
			let idx = self.get_index(row, col);
			self.cells[idx] = Cell::Alive;
		}
	}

	fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
	}
}
```

The `get_index` is a helper fumction that will translate Universe coordinates into index of coresponding cell in `cells` array.

Next we will implement `Display` trait for easy printing out current state of our game:

```rust
use std::fmt;

impl fmt::Display for Universe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for line in self.cells.as_slice().chunks(self.width as usize) {
			for &cell in line {
				let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
				write!(f, "{}", symbol)?;
			}
			write!(f, "\n")?;
		}

		Ok(())
	}
}
```

Perfect! Now we have something to run. Head over to your `main.rs` and replace all with the following content:

```rust
mod game;

fn main() {
	let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    print!("{}", game);
}
```

And run your prohect with `cargo run`. Ok it works! Of course nothing is really happening here so let's do another step and add the `tick` function to our code:

```rust
	pub fn tick(&mut self) {
		let mut next = self.cells.clone();
		for row in 0..self.height {
			for col in 0..self.width {
				let idx = self.get_index(row, col);
				let cell = self.cells[idx];
				let live_neighbours = self.live_neighbour_count(row, col);
				next[idx] = match (cell, live_neighbours) {
					(Cell::Alive, x) if x < 2 => Cell::Dead,
					(Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
					(Cell::Alive, x) if x > 3 => Cell::Dead,
					(Cell::Dead, 3) => Cell::Alive,
					(otherwise, _) => otherwise,
				};
			}
		}
		self.cells = next;
	}

	fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
		let mut count = 0;
		for delta_row in [self.height - 1, 0, 1].iter().cloned() {
			for delta_col in [self.width - 1, 0, 1].iter().cloned() {
				if delta_row == 0 && delta_col == 0 {
					continue;
				}

				let neighbour_row = (row + delta_row) % self.height;
				let neighbour_col = (column + delta_col) % self.width;
				let idx = self.get_index(neighbour_row, neighbour_col);
				count += self.cells[idx] as u8;
			}
		}

		count
	}
```

This code comes straight from the WASM rust book and it applies Conway's Game Of Life rules to our universe while also taking care of edge wrapping so that our universe seems looped ([See flavour 3](https://rustwasm.github.io/docs/book/game-of-life/implementing.html)).
Before we can use tick we need to prepare our terminal to display animated game Universe. Let's hop into that right now!

P.S. - You can find source code for this chapter on my [GitHub](https://github.com/jbarszczewski/cli-game-of-life/tree/42c60e1c10073dd65819af7d1a6d7b049d1a449d)

# Animate The Universe

To work with terminal input/output we will use [Crossterm crate](https://crates.io/crates/crossterm), so let's add it to our `Cargo.toml`:

```yaml
[dependencies]
crossterm = "0.19.0"
```

This crate have some really handy functions to manipulate terminal and it's cross platform we don't need to worry about any differences. Most of the crossterm commands are self explanatory as they are grouped into relevan modules, like `cursor::Hide` does exactly what it says: it hides the cursor.

Because our game Universe will be updated and displayed in a loop we want to clear the screen before each tick. We will move into the alternate screen for the game time and go back to original terminal screen once we are done. First let's make sure we have all the necessery imports:

```rust
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::io::stdout;
use std::time::Duration;
```

Also our `main` function need complete overhaul and now will look like this:

```rust
fn main() -> Result<()> {
    let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    execute!(
        stdout(),
        EnterAlternateScreen,
        SetForegroundColor(Color::Magenta),
        Hide
    )?;

    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(_) => break,
                _ => {}
            }
        } else {
            execute!(
                stdout(),
                Clear(ClearType::All),
                MoveTo(0, 0),
                Print(&game),
                Print("Press enter to exit...")
            )?;
            game.tick();
        }
    }
    execute!(stdout(), ResetColor, Show, LeaveAlternateScreen)?;
    Ok(())
}
```

Ok let's break down what we did here:

1. `main` now returns Result type. This will allow us to provide feedback to users and set appropriate exit codes where needed.
2. We set up our terminal in `execute!` macro, which accepts `std::io::Writer` type (stdout in our case) as first argument followed by one or more commands.
3. In a loop we try do read the user input wrapped in a `poll` which ensure that we don't block the execution. We break the loop when user press the Enter key. If no user input is available in 500ms then we draw current state of the Universe and compute next state with `tick()`
4. Once the loop is over we leave the alternate screen of the terminal.

Now run the app with `cargo run` and you should see simple pattern alternating between horizontal and vertical lines.
Ok but pressing Enter is not really what user expect when trying to exit the app. Let's modify our code so that it could respond to different keys.

# Interact with The Universe

Reason why we could only process Enter is that by default input is being processed on enter press. Which makes sense as usually you first want to type in the command and execute when it's all ready. But in our case we want user to be able to interact with single key presses. That means we need to enable [raw mode](https://docs.rs/crossterm/0.19.0/crossterm/terminal/#raw-mode). New code changes are as follow:

```rust
// add required imports:
use terminal::{disable_raw_mode, enable_raw_mode};

// add this line at the very begining of the main() function:
enable_raw_mode()?;

// replace code block when poll returns true, the match statement, with following:

if let Event::Key(KeyEvent { code, .. }) = read()? {
    match code {
        KeyCode::Esc => {
            break;
        }
        _ => {}
    }
}

// finaly disable raw mode at the end of the function before returning Ok(()):
disable_raw_mode()?;
```

It's very importat to add ability to exit from the loop as raw mode disables ctrl+c funcionality.
