use std::time::Instant;
use mint::Point2;
use ggez::event::KeyCode;
use rand::Rng;

// Define the game state
#[derive(Clone)]
pub struct GameState {
    pub player_health: i32,
    pub opened_boxes: usize,
    pub game_start_time: Instant,
    pub player_position:mint::Point2<f32>,
    pub boxes: Vec<GameBox>,
}

// Define the effect of opening a box
#[derive(Clone, Copy)]
pub enum BoxEffect {
    Good,
    Bad,
}

// Define the box struct
#[derive(Clone)]
pub struct GameBox {
    pub position: Point2<f32>,
    pub effect: BoxEffect,
    pub opened: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut boxes = Vec::new();

        // Generate 10 boxes with random positions
        for _ in 0..10 {
            let box_position = Point2 {
                x: rng.gen_range(0.0..800.0),
                y: rng.gen_range(0.0..600.0),
            };
            let box_effect = if rng.gen::<f32>() < 0.5 {
                BoxEffect::Good
            } else {
                BoxEffect::Bad
            };
            boxes.push(GameBox {
                position: box_position,
                effect: box_effect,
                opened: false,
            });
        }
        GameState {
            player_health: 100,
            opened_boxes: 0,
            game_start_time: Instant::now(),
            player_position: Point2 { x: 300.0, y: 550.0 },
            boxes,
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

        // Render boxes
        for box_entity in &self.game_state.boxes {
            if !box_entity.opened{
            let box_color = match box_entity.effect {
                BoxEffect::Good => ggez::graphics::Color::GREEN,
                BoxEffect::Bad => ggez::graphics::Color::RED,
            };

            let box_rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                ggez::graphics::Rect::new(
                    box_entity.position.x,
                    box_entity.position.y,
                    30.0,
                    30.0,
                ),
                box_color,
            )?;
            ggez::graphics::draw(ctx, &box_rect, ggez::graphics::DrawParam::default())?;
        }
        }

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

        // Check for box interactions
    for box_entity in &mut game_state.boxes {
        if player_near_box(&game_state.player_position, &box_entity.position) && !box_entity.opened {
            if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Space) {
                // Open the box and apply its effect to player health
                match box_entity.effect {
                    BoxEffect::Good => game_state.player_health += 10,
                    BoxEffect::Bad => game_state.player_health -= 10,
                }
                game_state.opened_boxes += 1;
                box_entity.opened = true; // Set the opened flag to true
            }
        }
    }

}    

fn player_near_box(player_position: &Point2<f32>, box_position: &Point2<f32>) -> bool {
    // Simple distance check for player proximity to a box
    let distance = ((player_position.x - box_position.x).powi(2)
        + (player_position.y - box_position.y).powi(2))
    .sqrt();
    distance < 30.0 // Adjust this value based on your preference
}