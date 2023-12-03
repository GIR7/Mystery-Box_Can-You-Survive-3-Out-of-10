// main.rs

use ggez::{event, ContextBuilder, GameResult};
use ggez::event::KeyCode;


use Mystery_Box_Can_You_Survive_3_Out_of_10::{GameState, GameWindow, handle_input};

struct MainState {
    game_window: GameWindow,
}

impl MainState {
    fn new(_ctx: &mut ggez::Context) -> GameResult<Self> {
        let game_window = GameWindow::new();

        Ok(MainState { game_window })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        handle_input(&mut self.game_window.game_state, ctx);
        println!("Player Position: {:?}", self.game_window.game_state.player_position);
        // Add logic later, first try

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        self.game_window.render(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Mystery Box", "Y.T.")
        .window_setup(ggez::conf::WindowSetup::default().title("Mystery Box! Can You Survive 3 Out of 10?"))
        .build()?;

    let main_state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, main_state)
}
