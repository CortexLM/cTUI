# Tutorial 06: Animations

Add smooth transitions and animated effects.

## Goals

- Use easing functions
- Create keyframe animations
- Animate component properties

## Easing Functions

cTUI provides many easing functions:

```rust
use ctui_animate::EasingFunction;

// Common easings
EasingFunction::Linear
EasingFunction::EaseIn
EasingFunction::EaseOut
EasingFunction::EaseInOut

// Cubic (most common for UI)
EasingFunction::EaseInCubic
EasingFunction::EaseOutCubic
EasingFunction::EaseInOutCubic

// Bounce effects
EasingFunction::EaseOutBounce
EasingFunction::EaseInElastic
```

### Easing Visual

```
Linear:      ████████████████████
EaseIn:      ████▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
EaseOut:     ▄▄▄▄▄▄▄▄▄▄▄▄▄████████
EaseInOut:   ▄▄▄████████████▄▄▄▄
Bounce:      ████████░░░░████░░██
```

## Keyframe Animation

```rust
use ctui_animate::{Keyframe, KeyframeAnimation, PlaybackMode};

let mut animation = KeyframeAnimation::new()
    .keyframe(Keyframe::new(0.0, 0.0))     // Start at 0
    .keyframe(Keyframe::new(1.0, 100.0))   // End at 100
    .duration_ms(1000)                      // 1 second
    .easing(EasingFunction::EaseOutCubic);

// In your update loop
animation.tick(delta_ms);
let current_value = animation.current_value();
```

## Animated Component

```rust
use ctui_animate::{Keyframe, KeyframeAnimation, EasingFunction};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

struct AnimatedCounter {
    value: i32,
    displayed_value: f64,
    animation: Option<KeyframeAnimation>,
}

impl AnimatedCounter {
    fn new() -> Self {
        Self {
            value: 0,
            displayed_value: 0.0,
            animation: None,
        }
    }

    fn set_value(&mut self, new_value: i32) {
        let animation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, self.displayed_value))
            .keyframe(Keyframe::new(1.0, new_value as f64))
            .duration_ms(300)
            .easing(EasingFunction::EaseOutCubic);
        
        self.animation = Some(animation);
        self.value = new_value;
    }
}

impl Component for AnimatedCounter {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self {
        Self::new()
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Display animated value
        let display = format!("Count: {:.0}", self.displayed_value);
        for (i, ch) in display.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        // Tick animation
        if let Some(ref mut animation) = self.animation {
            animation.tick(16); // ~60fps
            
            self.displayed_value = animation.current_value();
            
            if animation.is_complete() {
                self.displayed_value = self.value as f64;
                self.animation = None;
            }
        }
        Cmd::Render
    }
}
```

## Spring Physics

For more natural motion:

```rust
use ctui_animate::{SpringAnimation, SpringConfig};

let mut spring = SpringAnimation::new()
    .from(0.0)
    .to(100.0)
    .config(SpringConfig::bouncy());

// In update loop
spring.tick(delta_ms);
let value = spring.current_value();

// Change target (spring animates to new value)
spring.set_target(150.0);
```

### Spring Configurations

```rust
// Bouncy (lots of overshoot)
SpringConfig::bouncy()  // stiffness: 300, damping: 10

// Stiff (quick, minimal bounce)
SpringConfig::stiff()   // stiffness: 500, damping: 30

// Slow (lazy feeling)
SpringConfig::slow()    // stiffness: 100, damping: 20

// Custom
SpringConfig {
    stiffness: 400.0,
    damping: 25.0,
    mass: 1.0,
}
```

## Transition Effects

Animate style changes:

```rust
use ctui_animate::{Transition, interpolate_color};
use ctui_core::Color;

struct AnimatedButton {
    base_color: Color,
    hover_color: Color,
    current_color: Color,
    hovered: bool,
    transition: Option<Transition>,
}

impl AnimatedButton {
    fn on_hover(&mut self, hovered: bool) {
        self.hovered = hovered;
        
        let target_color = if hovered {
            self.hover_color
        } else {
            self.base_color
        };
        
        // Start color transition
        self.transition = Some(Transition::new()
            .duration_ms(200)
            .easing(EasingFunction::EaseOut));
    }
}
```

## Color Interpolation

```rust
use ctui_animate::interpolate_color;

let start = Color::Red;
let end = Color::Blue;
let t = 0.5; // 50% progress

let middle_color = interpolate_color(start, end, t);
// Returns a color between red and blue
```

## Position Animation

Animate element position:

```rust
struct SlidingPanel {
    panels: Vec<Panel>,
    positions: Vec<f64>,  // Animated positions
    targets: Vec<f64>,    // Target positions
}

impl SlidingPanel {
    fn slide_in(&mut self, index: usize) {
        self.targets[index] = 0.0;
    }

    fn slide_out(&mut self, index: usize) {
        self.targets[index] = -100.0;
    }

    fn update(&mut self) {
        for (pos, target) in self.positions.iter_mut().zip(&self.targets) {
            // Smooth interpolation
            let diff = target - *pos;
            *pos += diff * 0.1; // 10% of difference each frame
        }
    }
}
```

## Spinner Animation

```rust
use ctui_components::{Spinner, SpinnerStyle};

struct LoadingIndicator {
    spinner: Spinner,
    frame: usize,
}

impl LoadingIndicator {
    fn new() -> Self {
        Self {
            spinner: Spinner::new().style(SpinnerStyle::Dots),
            frame: 0,
        }
    }

    fn tick(&mut self) {
        self.frame = (self.frame + 1) % 10;
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let spinner_char = frames[self.frame];
        
        // Render spinner
        if let Some(cell) = buf.get_mut(area.x, area.y) {
            cell.symbol = spinner_char.to_string();
        }
    }
}
```

## Progress Animation

```rust
use ctui_components::ProgressBar;

struct AnimatedProgress {
    progress: f64,
    target: f64,
}

impl AnimatedProgress {
    fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    fn update(&mut self, delta_ms: u32) {
        // Animate towards target
        let diff = self.target - self.progress;
        if diff.abs() > 0.001 {
            self.progress += diff * 0.05; // Smooth step
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let bar = ProgressBar::new()
            .ratio(self.progress)
            .show_percent(true);
        
        bar.render(area, buf);
    }
}
```

## Exercise

1. Create a button that bounces when clicked
2. Animate a progress bar from 0 to 100%
3. Create a sliding panel that animates in and out

## Solution

```rust
use ctui_animate::{Keyframe, KeyframeAnimation, EasingFunction, SpringAnimation, SpringConfig};
use ctui_core::{Buffer, Rect};

// Bouncing button
struct BounceButton {
    label: String,
    scale: f64,
    spring: SpringAnimation,
    pressed: bool,
}

impl BounceButton {
    fn on_press(&mut self) {
        self.pressed = true;
        self.spring = SpringAnimation::new()
            .from(0.8)
            .to(1.0)
            .config(SpringConfig::bouncy());
    }

    fn update(&mut self) {
        self.spring.tick(16);
        self.scale = self.spring.current_value();
    }
}

// Animated progress
struct AnimatedProgressBar {
    current: f64,
    target: f64,
    animation: KeyframeAnimation,
}

impl AnimatedProgressBar {
    fn set_progress(&mut self, target: f64) {
        self.target = target;
        self.animation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, self.current))
            .keyframe(Keyframe::new(1.0, target))
            .duration_ms(500)
            .easing(EasingFunction::EaseOutCubic);
    }

    fn update(&mut self, delta: u32) {
        self.animation.tick(delta);
        self.current = self.animation.current_value();
    }
}

// Sliding panel
struct SlidingPanel {
    x: f64,
    target_x: f64,
    visible: bool,
}

impl SlidingPanel {
    fn show(&mut self) {
        self.visible = true;
        self.target_x = 0.0;
    }

    fn hide(&mut self) {
        self.target_x = -50.0;
    }

    fn update(&mut self) {
        let diff = self.target_x - self.x;
        self.x += diff * 0.15; // Smooth step
        
        if diff.abs() < 0.1 && self.target_x < 0.0 {
            self.visible = false;
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if !self.visible { return; }
        
        let render_x = area.x as f64 + self.x;
        // Render panel at animated position
    }
}
```

## Next Steps

Continue to [Tutorial 07: Events](07-events.md) to learn about handling user input.

## See Also

- [Animation API](../api/animation.md)
- [Spinner Component](../gallery/progress.md)
