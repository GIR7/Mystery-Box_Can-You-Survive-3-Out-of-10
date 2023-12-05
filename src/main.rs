// main.rs

use ggez::{event, ContextBuilder, GameResult};
use ggez::event::KeyCode;


use Mystery_Box_Can_You_Survive_3_Out_of_10::{GameState, GameWindow, handle_input};

struct MainState {
    game_window: GameWindow,
    game_over: Option<GameOver>,
}

enum GameOver {
    Win,
    Lose,
}

impl MainState {
    fn new(_ctx: &mut ggez::Context) -> GameResult<Self> {
        let game_window = GameWindow::new();

        Ok(MainState { game_window,game_over: None, })
    }

    fn check_game_conditions(&mut self, ctx: &mut ggez::Context) {
        let game_state = &self.game_window.game_state;

        // Check winning conditions
        if game_state.opened_boxes >= 3
            && game_state.player_health > 0
            && game_state.game_start_time.elapsed().as_secs() <= 60
        {
            self.game_over = Some(GameOver::Win);
        }

        // Check losing conditions
        if game_state.player_health <= 0
            || game_state.game_start_time.elapsed().as_secs() > 60
        {
            self.game_over = Some(GameOver::Lose);
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {

        // Check game conditions
        self.check_game_conditions(ctx);

        handle_input(&mut self.game_window.game_state, ctx);
        println!("Player Position: {:?}", self.game_window.game_state.player_position);
        

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        self.game_window.render(ctx)?;

        // If the game is over, display the corresponding message
        if let Some(game_over) = &self.game_over {
            let message = match game_over {
                GameOver::Win => "You Win!",
                GameOver::Lose => "You Lose!",
            };

            // Draw the message in the center of the screen
            let text = ggez::graphics::Text::new(message);
            let text_dest = ggez::graphics::DrawParam::new()
                .dest(ggez::mint::Point2 { x: 300.0, y: 200.0 });
            ggez::graphics::draw(ctx, &text, text_dest)?;

            // Present the frame
            ggez::graphics::present(ctx)?;
        }
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
