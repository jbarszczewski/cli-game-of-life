mod game;

fn main() {
    let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    print!("{}", game);
}
