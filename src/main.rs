use ggez::{event, ContextBuilder, GameResult};
use ggez::event::KeyCode;

// Import crate
use Mystery_Box_Can_You_Survive_3_Out_of_10::{GameState, GameWindow, handle_input};

// Define the event handler for the game
struct MainState {
    game_window: GameWindow,
    game_state: GameState,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        // Create a new GameState and GameWindow
        let game_state = GameState::new();
        let game_window = GameWindow::new(game_state.clone());
        // let game_window = GameWindow::new(game_state);

        Ok(MainState { game_window, game_state })
    }
}

// Implement the event handler for the game
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        // Handle input
        handle_input(&mut self.game_state, _ctx);
        println!("Player Position: {:?}", self.game_state.player_position);
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
    let (mut ctx, event_loop) = ContextBuilder::new("Mystery Box", "Y.T.")
        .window_setup(ggez::conf::WindowSetup::default().title("Mystery Box! Can You Survive 3 Out of 10?"))
        .build()?;
    
    // Create a new instance of MainState
    let main_state = MainState::new(&mut ctx)?;

    // Run the game loop
    event::run(ctx, event_loop, main_state)
}

