mod game;
mod matrix;
mod constants;
mod vec2;

use crate::game::Game;

fn main() {
    let game_instance = Game::new(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT, "Tetris");
    let (mut rl, thread) = raylib::init()
        .size(game_instance.width, game_instance.height)
        .title(&game_instance.title)
        .vsync()
        .build();

    rl.set_target_fps(constants::FPS);

    while !rl.window_should_close() {
        game_instance.update();
        // DRAW
        let mut d = rl.begin_drawing(&thread);
        game_instance.render(&mut d);
    }
}
