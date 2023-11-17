use std::time::Instant;
use mint::Point2;

// Define the game state
pub struct GameState {
    pub player_health: i32,
    pub opened_boxes: usize,
    pub game_start_time: Instant,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player_health: 100,
            opened_boxes: 0,
            game_start_time: Instant::now(),
        }
    }
}

// Define the game window
pub struct GameWindow {
    pub game_state: GameState,
}

impl GameWindow {
    pub fn new(game_state: GameState) -> Self {
        GameWindow { game_state }
    }

     pub fn render(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Clear the screen
        ggez::graphics::clear(ctx, ggez::graphics::Color::BLACK);

        // Render player health, opened boxes, and elapsed time
        let text = ggez::graphics::Text::new(format!(
            "Health: {}\nBoxes opened: {}\nTime elapsed: {:?}",
            self.game_state.player_health,
            self.game_state.opened_boxes,
            self.game_state.game_start_time.elapsed()
        ));

        // Draw the text in the center of the screen
        let text_dest = ggez::graphics::DrawParam::new()
            .dest(Point2 { x: 300.0, y: 200.0 });
        ggez::graphics::draw(ctx, &text, text_dest)?;

        // Present the frame
        ggez::graphics::present(ctx)?;
        Ok(())
    }
}
