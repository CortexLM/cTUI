# ctui-animate - AGENTS.md

## OVERVIEW

Animation primitives: easing functions, keyframes, spring physics, transitions. Frame-rate independent with global manager.

## WHERE TO LOOK

| Need | File |
|------|------|
| Easing curves | `easing.rs` |
| Keyframe animation | `keyframe.rs` |
| Spring physics | `spring.rs` |
| Value interpolation | `interpolate.rs`, `transition.rs` |
| Animation manager | `manager.rs`, `scheduler.rs` |
| Sequences/groups | `sequence.rs` |
| Animated wrappers | `animated.rs` |

## KEY TYPES

```rust
EasingFunction                    // Linear, EaseIn, EaseOut, Elastic, Bounce...
KeyframeAnimation, Keyframe       // Timeline animation
PlaybackMode                      // Once, Loop, PingPong, Reverse
SpringAnimation, SpringConfig    // Physics-based motion
Interpolator, lerp                // Value blending
AnimationManager, AnimationId     // Global state
Transition, TransitionBuilder    // Property transitions
```

## CONVENTIONS

- Time in milliseconds, values as f32/f64
- Spring physics: mass, stiffness, damping
- Keyframes: 0.0-1.0 normalized time
