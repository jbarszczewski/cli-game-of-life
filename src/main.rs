mod game;

fn main() {
    let game = game::Universe::new(8, 8);
    print!("{}", game);
}
