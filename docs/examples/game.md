# Game Example

Simple terminal game demonstrating game loop and real-time updates.

## Overview

The game demonstrates:

- Game loop pattern
- Real-time rendering
- Input handling
- Animation
- State machines

## Render Output

```
╭ Snake Game ─────────────────────────────────────────────────────╮
│ Score: 150  Level: 3                                             │
│                                                                 │
│                                                                 │
│       ████████                                                   │
│       █        █                                ████            │
│       █   ●    █                                █  █            │
│       █        █                                ████            │
│       ████████                                                   │
│                                           ████                   │
│                                                                 │
│                                                                 │
│                                                                 │
│                                                                 │
│                                                                 │
╰──────────────────────────────────────────────────────────────────╯
  Arrow keys or WASD to move | P to pause | Q to quit
```

## Source Code

```rust
//! Game example - simple snake game

use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Event, KeyCode};
use ctui_components::{Block, Borders};
use ctui_layout::{Layout, FlexDirection, Constraint};
use ctui_animate::{KeyframeAnimation, EasingFunction};
use std::collections::VecDeque;

struct SnakeGame {
    snake: VecDeque<Position>,
    food: Position,
    direction: Direction,
    score: u32,
    level: u32,
    paused: bool,
    game_over: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: u16,
    y: u16,
}

impl SnakeGame {
    fn new(width: u16, height: u16) -> Self {
        let center_x = width / 2;
        let center_y = height / 2;
        
        let mut snake = VecDeque::new();
        snake.push_back(Position { x: center_x, y: center_y });
        snake.push_back(Position { x: center_x - 1, y: center_y });
        snake.push_back(Position { x: center_x - 2, y: center_y });
        
        Self {
            snake,
            food: Position { x: 5, y: 5 },
            direction: Direction::Right,
            score: 0,
            level: 1,
            paused: false,
            game_over: false,
        }
    }
    
    fn tick(&mut self, width: u16, height: u16) {
        if self.paused || self.game_over {
            return;
        }
        
        // Calculate new head position
        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position { x: head.x, y: head.y.saturating_sub(1) },
            Direction::Down => Position { x: head.x, y: head.y + 1 },
            Direction::Left => Position { x: head.x.saturating_sub(1), y: head.y },
            Direction::Right => Position { x: head.x + 1, y: head.y },
        };
        
        // Wrap around
        let new_head = Position {
            x: new_head.x % width,
            y: new_head.y % height,
        };
        
        // Check collision with self
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }
        
        // Move snake
        self.snake.push_front(new_head);
        
        // Check if eating food
        if new_head == self.food {
            self.score += 10;
            if self.score % 100 == 0 {
                self.level += 1;
            }
            self.spawn_food(width, height);
        } else {
            self.snake.pop_back();
        }
    }
    
    fn spawn_food(&mut self, width: u16, height: u16) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        loop {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            let pos = Position { x, y };
            
            if !self.snake.contains(&pos) {
                self.food = pos;
                break;
            }
        }
    }
    
    fn speed_ms(&self) -> u64 {
        // Speed increases with level
        200 - (self.level as u64 * 10).min(150)
    }
}

impl Component for SnakeGame {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self {
        Self::new(40, 15)
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::flex()
            .direction(FlexDirection::Column);

        let rects = layout.split(area, &[
            Constraint::Length(3),   // Header
            Constraint::Min(0),     // Game area
            Constraint::Length(1),  // Footer
        ]);

        // Render header
        let header = format!("Score: {:04}  Level: {}", self.score, self.level);
        for (i, ch) in header.chars().enumerate() {
            if let Some(cell) = buf.get_mut(rects[0].x + i as u16, rects[0].y + 1) {
                cell.symbol = ch.to_string();
            }
        }

        // Render game area
        let game_area = rects[1];
        
        // Draw border
        let block = Block::new().borders(Borders::ALL);
        block.render(game_area, buf);

        // Draw snake
        for (i, pos) in self.snake.iter().enumerate() {
            let ch = if i == 0 { "█" } else { "█" };
            if let Some(cell) = buf.get_mut(
                game_area.x + 1 + pos.x,
                game_area.y + 1 + pos.y,
            ) {
                cell.symbol = ch.to_string();
            }
        }

        // Draw food
        if let Some(cell) = buf.get_mut(
            game_area.x + 1 + self.food.x,
            game_area.y + 1 + self.food.y,
        ) {
            cell.symbol = "●".to_string();
        }

        // Game over message
        if self.game_over {
            let msg = "GAME OVER - Press R to restart";
            let x = game_area.x + (game_area.width / 2) - (msg.len() as u16 / 2);
            let y = game_area.y + game_area.height / 2;
            
            for (i, ch) in msg.chars().enumerate() {
                if let Some(cell) = buf.get_mut(x + i as u16, y) {
                    cell.symbol = ch.to_string();
                }
            }
        }

        // Render footer
        let footer = if self.paused { 
            "PAUSED".to_string() 
        } else { 
            "Arrow keys or WASD | P=Pause | Q=Quit | R=Restart".to_string() 
        };
        
        for (i, ch) in footer.chars().take(rects[2].width as usize).enumerate() {
            if let Some(cell) = buf.get_mut(rects[2].x + i as u16, rects[2].y) {
                cell.symbol = ch.to_string();
            }
        }
    }

    fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
        match event {
            Event::Key(key) => match key.code {
                // Movement
                KeyCode::Up | KeyCode::Char('w') => {
                    if self.direction != Direction::Down {
                        self.direction = Direction::Up;
                    }
                }
                KeyCode::Down | KeyCode::Char('s') => {
                    if self.direction != Direction::Up {
                        self.direction = Direction::Down;
                    }
                }
                KeyCode::Left | KeyCode::Char('a') => {
                    if self.direction != Direction::Right {
                        self.direction = Direction::Left;
                    }
                }
                KeyCode::Right | KeyCode::Char('d') => {
                    if self.direction != Direction::Left {
                        self.direction = Direction::Right;
                    }
                }
                // Pause
                KeyCode::Char('p') => {
                    self.paused = !self.paused;
                }
                // Restart
                KeyCode::Char('r') => {
                    *self = Self::new(40, 15);
                }
                _ => {}
            },
            _ => {}
        }
        Some(Box::new(UpdateGame))
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if msg.is::<UpdateGame>() {
            self.tick(38, 13);  // Game area size minus borders
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

struct UpdateGame;
impl Msg for UpdateGame {}

fn main() {
    let mut game = SnakeGame::new(40, 15);
    game.on_mount();

    println!("Snake Game Example");
    println!("=================\n");
    println!("Controls:");
    println!("  Arrow keys or WASD to move");
    println!("  P to pause");
    println!("  Q to quit");
    println!("  R to restart");
    println!("\nScore: {}", game.score);
    println!("Level: {}", game.level);

    game.on_unmount();
}
```

## Key Concepts

### Game Loop

```rust
// Main game loop
loop {
    // Handle input
    if let Some(event) = terminal.poll_event(timeout)? {
        game.handle_event(&event);
    }

    // Update game state
    game.tick();

    // Render
    terminal.draw(|frame| {
        game.render(frame.area(), frame.buffer());
    })?;

    // Control frame rate
    std::thread::sleep(Duration::from_millis(game.speed_ms()));
}
```

### Movement

```rust
enum Direction {
    Up, Down, Left, Right,
}

fn move_player(&mut self, direction: Direction) {
    // Prevent 180-degree turns
    match (self.direction, direction) {
        (Up, Down) | (Down, Up) | (Left, Right) | (Right, Left) => return,
        _ => {}
    }
    self.direction = direction;
}
```

### Collision Detection

```rust
fn check_collisions(&self) -> Collision {
    let head = self.snake.front().unwrap();
    
    if self.snake.iter().skip(1).any(|p| p == head) {
        Collision::SelfCollision
    } else if head == &self.food {
        Collision::Food
    } else {
        Collision::None
    }
}
```

## Enhancements

### Add Levels

```rust
impl SnakeGame {
    fn spawn_obstacles(&mut self, level: u32) {
        let obstacle_count = level * 2;
        for _ in 0..obstacle_count {
            // Spawn obstacles that snake must avoid
        }
    }
}
```

### Add Power-ups

```rust
enum PowerUp {
    Speed(Duration),
    Invincibility(Duration),
    DoublePoints(Duration),
}

impl SnakeGame {
    fn apply_powerup(&mut self, powerup: PowerUp) {
        match powerup {
            PowerUp::Speed(d) => self.speed_boost = Some(d),
            PowerUp::Invincibility(d) => self.invincible = Some(d),
            PowerUp::DoublePoints(d) => self.double_points = Some(d),
        }
    }
}
```

### Add High Scores

```rust
struct HighScores {
    scores: Vec<ScoreEntry>,
}

impl HighScores {
    fn load() -> Self { /* ... */ }
    fn save(&self) { /* ... */ }
    fn add(&mut self, score: ScoreEntry) -> Option<usize> {
        // Returns rank if high score
    }
}
```

## Run the Example

```bash
cargo run --example game
```

## See Also

- [Animation API](../api/animation.md) - Animation system
- [Dashboard](dashboard.md) - Real-time updates
