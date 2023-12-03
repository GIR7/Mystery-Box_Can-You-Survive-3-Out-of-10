use std::time::Instant;
use mint::Point2;
use ggez::event::KeyCode;

// Define the game state
#[derive(Clone)]
pub struct GameState {
    pub player_health: i32,
    pub opened_boxes: usize,
    pub game_start_time: Instant,
    pub player_position:mint::Point2<f32>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player_health: 100,
            opened_boxes: 0,
            game_start_time: Instant::now(),
            player_position: Point2 { x: 300.0, y: 550.0 },
        }
    }
}

// Define the game window

pub struct GameWindow {
    pub game_state: GameState,
}

impl GameWindow {
    pub fn new() -> Self {
        GameWindow {
            game_state: GameState::new(),
        }
    }
     pub fn render(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Clear the screen
        ggez::graphics::clear(ctx, ggez::graphics::Color::BLACK);

        // Render player position
        let player_rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(
                self.game_state.player_position.x,
                self.game_state.player_position.y,
                30.0,
                30.0,
            ),
            ggez::graphics::Color::WHITE,
        )?;
        ggez::graphics::draw(ctx, &player_rect, ggez::graphics::DrawParam::default())?;
        
        // Render player health, opened boxes, and elapsed time
        let text = ggez::graphics::Text::new(format!(
            "Health: {}\nBoxes opened: {}\nTime elapsed: {:?}",
            self.game_state.player_health,
            self.game_state.opened_boxes,
            self.game_state.game_start_time.elapsed()
        ));
        // Draw the text in the center of the screen
        let text_dest = ggez::graphics::DrawParam::new()
            .dest(Point2 { x:15.0, y: 10.0 });
        ggez::graphics::draw(ctx, &text, text_dest)?;

        // Present the frame
        ggez::graphics::present(ctx)?;
        Ok(())
    }
}

pub fn handle_input(game_state: &mut GameState, ctx: &mut ggez::Context) {
        let speed = 5.0; 
        // Check keyboard input
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            game_state.player_position.y -= speed;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            game_state.player_position.x -= speed;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            game_state.player_position.x += speed;
        }
}    