# API Reference - Animation

Animation primitives for smooth, performant TUI animations.

## Overview

cTUI's animation system provides easing functions, keyframe animations, spring physics, and smooth transitions.

```rust
use ctui_animate::{EasingFunction, Keyframe, KeyframeAnimation, PlaybackMode};

let animation = KeyframeAnimation::new()
    .keyframe(Keyframe::new(0.0, 0.0))
    .keyframe(Keyframe::new(1.0, 100.0))
    .duration_ms(1000)
    .easing(EasingFunction::EaseOutCubic);
```

---

## Easing Functions

Easing functions control the rate of change over time.

### EasingFunction

```rust
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
    Custom(Box<dyn Fn(f64) -> f64>),
}
```

### Easing Curves

```
Linear:      ─────────────────
EaseIn:      ────╮
                 │
                 ╰─────────────
EaseOut:    ────────╮
                   │
             ╰─────
EaseInOut:       ╭────────
            ─────┤
                 ╰─────
```

### Usage

```rust
// Apply easing to animation
let animation = KeyframeAnimation::new()
    .easing(EasingFunction::EaseOutBounce);

// Or use directly
let t = 0.5; // Progress (0.0 - 1.0)
let eased = EasingFunction::EaseOutCubic.apply(t);
```

---

## Keyframe Animations

Define animations with keyframes.

### Keyframe

```rust
pub struct Keyframe {
    pub position: f64,  // Position (0.0 - 1.0)
    pub value: f64,     // Value at this position
}

let keyframe = Keyframe::new(0.0, 0.0);       // Start at 0
let keyframe = Keyframe::new(1.0, 100.0);    // End at 100
let keyframe = Keyframe::new(0.5, 50.0);     // Middle at 50
```

### KeyframeAnimation

```rust
let mut animation = KeyframeAnimation::new()
    .keyframe(Keyframe::new(0.0, 0.0))
    .keyframe(Keyframe::new(0.3, 20.0))   // Early peak
    .keyframe(Keyframe::new(1.0, 100.0))
    .duration_ms(2000)
    .easing(EasingFunction::EaseOutCubic)
    .playback_mode(PlaybackMode::Once);
```

### PlaybackMode

```rust
pub enum PlaybackMode {
    Once,        // Play once and stop
    Loop,        // Loop from start
    Reverse,     // Play forward then backward
    LoopReverse, // Loop alternating directions
}
```

### Animation Control

```rust
// Tick the animation
animation.tick(delta_ms);

// Get current value
let value = animation.current_value();

// Check if complete
let done = animation.is_complete();

// Reset animation
animation.reset();

// Pause/resume
animation.pause();
animation.resume();
```

---

## Spring Physics

Natural motion with spring simulation.

### SpringAnimation

```rust
use ctui_animate::{SpringAnimation, SpringConfig, SpringBuilder};

let spring = SpringAnimation::new()
    .from(0.0)
    .to(100.0)
    .config(SpringConfig::bouncy());
```

### SpringConfig

```rust
impl SpringConfig {
    fn bouncy() -> Self {
        Self {
            stiffness: 300.0,
            damping: 10.0,
            mass: 1.0,
        }
    }

    fn stiff() -> Self {
        Self {
            stiffness: 500.0,
            damping: 30.0,
            mass: 1.0,
        }
    }

    fn slow() -> Self {
        Self {
            stiffness: 100.0,
            damping: 20.0,
            mass: 1.0,
        }
    }
}
```

### Spring Parameters

| Parameter | Effect |
|-----------|--------|
| `stiffness` | Higher = faster oscillation |
| `damping` | Higher = less bounce |
| `mass` | Higher = heavier, slower |

### Usage

```rust
let mut spring = SpringAnimation::new()
    .from(0.0)
    .to(100.0)
    .config(SpringConfig {
        stiffness: 200.0,
        damping: 20.0,
        mass: 1.0,
    });

// In render loop
spring.tick(delta_ms);
let value = spring.current_value();

// Set new target (spring animates to it)
spring.set_target(150.0);
```

---

## Transitions

Smooth transitions between values.

### Transition

```rust
use ctui_animate::{Transition, TransitionBuilder, TransitionProperty};

let transition = Transition::new()
    .property(TransitionProperty::Position)
    .duration_ms(300)
    .easing(EasingFunction::EaseOutCubic);
```

### TransitionProperty

```rust
pub enum TransitionProperty {
    Position,
    Size,
    Color,
    Opacity,
    Scale,
    Rotation,
    Custom(String),
}
```

### TransitionBuilder

```rust
let transition = TransitionBuilder::new()
    .position(300)        // Position transition
    .color(200)           // Color transition
    .easing(EasingFunction::EaseInOut)
    .build();
```

### interpolate

```rust
use ctui_animate::{lerp, interpolate_color, interpolate_position};

// Linear interpolation
let value = lerp(0.0, 100.0, 0.5);  // 50.0

// Color interpolation
let color = interpolate_color(Color::Red, Color::Blue, 0.5);

// Position interpolation
let pos = interpolate_position(start, end, 0.5);
```

---

## Animation Scheduler

Manage multiple animations.

### AnimationScheduler

```rust
use ctui_animate::{AnimationScheduler, Animation, AnimationId};

let mut scheduler = AnimationScheduler::new();

// Schedule animation
let id = scheduler.schedule(Animation::new()
    .duration_ms(1000)
    .easing(EasingFunction::EaseOut));

// Update all animations
scheduler.tick(delta_ms);

// Cancel animation
scheduler.cancel(id);

// Pause/resume
scheduler.pause(id);
scheduler.resume(id);
```

---

## Animation Manager

High-level animation management.

### AnimationManager

```rust
use ctui_animate::{AnimationManager, ManagedId};

let mut manager = AnimationManager::new();

// Create managed animation
let id = manager.create(KeyframeAnimation::new()
    .keyframe(Keyframe::new(0.0, 0.0))
    .keyframe(Keyframe::new(1.0, 100.0)));

// Update and get values
manager.tick(delta_ms);
let value = manager.get_value(id);

// Queue animations
manager.queue(id, next_animation);
```

### AnimationStats

```rust
pub struct AnimationStats {
    pub active_count: usize,
    pub completed_count: usize,
    pub cancelled_count: usize,
    pub average_duration_ms: f64,
}
```

---

## Animation Sequences

Chain multiple animations.

### AnimationSequence

```rust
use ctui_animate::{AnimationSequence, SequenceId};

let sequence = AnimationSequence::new()
    .then(move_left)
    .then(move_up)
    .then(fade_out)
    .play();

// Or use delays
let sequence = AnimationSequence::new()
    .then_delayed(500, move_left)  // After 500ms
    .then(move_up);
```

### AnimationGroup

Run animations in parallel.

```rust
use ctui_animate::AnimationGroup;

let group = AnimationGroup::new()
    .add(fade_animation)
    .add(move_animation)
    .add(scale_animation)
    .play();

// Wait for all to complete
let complete = group.is_complete();
```

### AnimationController

```rust
pub struct AnimationController {
    // Control methods
    pub fn play(&mut self);
    pub fn pause(&mut self);
    pub fn resume(&mut self);
    pub fn stop(&mut self);
    pub fn restart(&mut self);
    
    // State
    pub fn is_playing(&self) -> bool;
    pub fn is_paused(&self) -> bool;
    pub fn progress(&self) -> f64;
}
```

---

## Animated Wrappers

### AnimatedStyle

```rust
use ctui_animate::AnimatedStyle;

let animated_style = AnimatedStyle::new(base_style)
    .animate_fg(Color::Red, Color::Blue, 500)
    .animate_opacity(1.0, 0.5, 300);

let current_style = animated_style.current_style();
```

### AnimatedLayout

```rust
use ctui_animate::AnimatedLayout;

let animated_layout = AnimatedLayout::new()
    .animate_position(start_pos, end_pos, 500)
    .animate_size(start_size, end_size, 300);

let current_rect = animated_layout.current_rect();
```

---

## Interpolator Trait

Custom interpolation for types.

```rust
use ctui_animate::Interpolator;

pub trait Interpolator {
    fn interpolator(&self, other: &Self, t: f64) -> Self;
}

// Implement for custom types
impl Interpolator for MyType {
    fn interpolator(&self, other: &Self, t: f64) -> Self {
        MyType {
            value: lerp(self.value, other.value, t),
        }
    }
}
```

---

## Animation Mode

```rust
pub enum AnimationMode {
    Once,
    Loop,
    PingPong,
    Reverse,
}
```

---

## Examples

### Button Hover Effect

```rust
impl Component for Button {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let color = if self.hovered {
            self.hover_animation.current_value()
        } else {
            self.normal_color
        };
        
        // Render with animated color
    }
}

fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
    if msg.is::<MouseEnter>() {
        self.hover_animation = SpringAnimation::new()
            .from(self.normal_color)
            .to(self.hover_color)
            .config(SpringConfig::bouncy());
    }
    Cmd::Render
}
```

### Smooth Transitions

```rust
impl Component for Panel {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Animate position
        let current_x = self.slide_animation.current_value();
        let animated_area = Rect::new(current_x as u16, area.y, area.width, area.height);
        
        // Render at animated position
        self.content.render(animated_area, buf);
    }
}
```

### Loading Animation

```rust
let mut spinner_frame = KeyframeAnimation::new()
    .keyframe(Keyframe::new(0.0, 0.0))
    .keyframe(Keyframe::new(1.0, 10.0))  // 10 frames
    .duration_ms(1000)
    .playback_mode(PlaybackMode::Loop);

// In render loop
spinner_frame.tick(delta_ms);
let frame = spinner_frame.current_value() as usize;
let spinner_char = SPINNER_CHARS[frame];
```
