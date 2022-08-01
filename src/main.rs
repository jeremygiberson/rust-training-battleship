mod game;
mod ship;
mod location;
mod player;
mod shot;


fn main() {
    //let operation = std::env::args().nth(1).expect("no pattern given");
    // switch on operation - new, place, place
    //let path = std::env::args().nth(2).expect("no path given");

    println!("Hello, world!");

    let game = game::GameState::new();
}
