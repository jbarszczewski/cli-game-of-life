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

After creating new project with `cargo new` (or `cargo init` if you're already in correct directory) open your favourite editor and... ignore main.rs for now. We're gonna create game logic module first.
