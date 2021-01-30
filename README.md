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

# Create Universe the TDD way

After creating new project something like `cargo new cli-game-of-life` (or `cargo init` if you're already in correct directory) open your favourite editor and... ignore main.rs for now. We're gonna create game logic module first, so go ahead and create a new file `src/game.rs`. As mentioned in before, I will base the logic on official wasm tutorial so if you've done it before it will be very familiar. Let's start with defining an enum that will represent single cell in our game universe:

```rust
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
	Dead = 0,
	Alive = 1,
}
```

`derive` attribute will tell the compiler to provide basic implementation of passed attributes so that we can assign cells with enum values and compare them.

**Note:** We could use simple bool value as well but enum will give us better readability while having the same memory footprint.

Our game universe is defined as follows:

```rust
pub struct Universe {
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}
```

Now we can start implementing functions for our game. Let's start with a handy constructor that will initialize
