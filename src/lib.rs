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
    Cure(i32),
    Injury(i32),
    NoEffect,
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
        //fixed the boxes over-lapped issue
        for _ in 0..10 {
            let mut box_position;
            loop {
                // Generate a potential box position
                box_position = Point2 {
                    x: rng.gen_range(0.0..800.0),
                    y: rng.gen_range(0.0..600.0),
                };

                // Check if the potential position collides with any existing box
                let collides = boxes.iter().any(|existing_box:&GameBox| {
                    (box_position.x + 30.0 > existing_box.position.x)
                        && (existing_box.position.x + 30.0 > box_position.x)
                        && (box_position.y + 30.0 > existing_box.position.y)
                        && (existing_box.position.y + 30.0 > box_position.y)
                });

                if !collides {
                    break; // Break the loop if the position doesn't collide with any existing box
                }
            }

            let box_effect = match rng.gen_range(0..2) {
                0 => BoxEffect::Cure(rng.gen_range(5..50)), // Add between 5 and 50 points
                1 => BoxEffect::Injury(rng.gen_range(50..100)),    // Deduct between 50 and 100 points
                _ => BoxEffect::NoEffect,//nothing happen
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
        ggez::graphics::clear(ctx, ggez::graphics::Color::from_rgb(108, 122, 137));

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
        
        // Render health bar background (border)
        let border_color = ggez::graphics::Color::BLACK; // Change to the color you want for the border
        let health_bar_background_rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::stroke(2.0), // Adjust the thickness of the border
            ggez::graphics::Rect::new(10.0, 10.0, 200.0, 15.0),
            border_color,
        )?;
        ggez::graphics::draw(ctx, &health_bar_background_rect, ggez::graphics::DrawParam::default())?;

        // Render health bar
        let health_percentage = self.game_state.player_health as f32 / 100.0;
        let health_bar_width = 200.0 * health_percentage;
        let health_bar_rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(10.0, 10.0, health_bar_width, 15.0),
            ggez::graphics::Color::GREEN,
        )?;
        ggez::graphics::draw(ctx, &health_bar_rect, ggez::graphics::DrawParam::default())?;

        // Render player health, opened boxes, and elapsed time
        let text = ggez::graphics::Text::new(format!(
            "Health: {}\nBoxes opened: {}\nTime elapsed: {:?}",
            self.game_state.player_health,
            self.game_state.opened_boxes,
            self.game_state.game_start_time.elapsed()
        ));
        // Draw the text in the center of the screen
        let text_dest = ggez::graphics::DrawParam::new()
            .dest(Point2 { x:15.0, y: 10.0 })
            .color(ggez::graphics::Color::BLACK);
        ggez::graphics::draw(ctx, &text, text_dest)?;

        // Render boxes
        for box_entity in &self.game_state.boxes {
            if !box_entity.opened{
            let box_color = ggez::graphics::Color::GREEN;

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
                    BoxEffect::Injury(points) => game_state.player_health =std::cmp::max(
                        game_state.player_health - points,
                        0,
                    ), 
                    // Ensure health doesn't exceed 100
                    BoxEffect::Cure(points) => game_state.player_health = std::cmp::min(
                        game_state.player_health + points,
                        100,
                    ), 
                    BoxEffect::NoEffect => (),
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