use ggez::{event, ContextBuilder, GameResult};

// Import crate
use Mystery_Box_Can_You_Survive_3_Out_of_10::{GameState, GameWindow};

// Define the event handler for the game
struct MainState {
    game_window: GameWindow,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        // Create a new GameState and GameWindow
        let game_state = GameState::new();
        let game_window = GameWindow::new(game_state);

        Ok(MainState { game_window })
    }
}

// Implement the event handler for the game
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        // Add logic later, first try
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Render the game window
        self.game_window.render(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    // Create a GGEZ context and a window
    let (ctx, event_loop) = &mut ContextBuilder::new("Mystery Box", "Y.T.")
        .build()?;
    // Create a new instance of MainState
    let main_state = &mut MainState { game_window: GameWindow::new(GameState::new()) };
    // Run the game loop
    event::run(ctx, event_loop, main_state)
}

