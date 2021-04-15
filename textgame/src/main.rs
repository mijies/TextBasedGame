use textgame::{Game, GameError, run_next};

fn main() {
    let mut game = Game::new();
    game.message_start();

    loop {
        game.message_status();

        if let Err(e) = run_next(&mut game) {
            if e == GameError::Over {
                break game.win_or_loss();
            }
            println!("{}", e);
        }
    }
    game.message_end();
}
