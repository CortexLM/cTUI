//! Animation primitives for cTUI framework.
//!
//! This crate provides a comprehensive animation system for TUI applications:
//! - Easing functions for smooth motion
//! - Interpolation primitives for value blending
//! - Keyframe animations with multiple playback modes
//! - Spring physics for natural motion
//! - Animation sequences and groups
//! - Global animation management
//! - Animated style and layout wrappers
//!
//! # Example
//!
//! ```
//! use ctui_animate::{Keyframe, KeyframeAnimation, PlaybackMode, EasingFunction};
//!
//! let mut animation = KeyframeAnimation::new()
//!     .keyframe(Keyframe::new(0.0, 0.0))
//!     .keyframe(Keyframe::new(1.0, 100.0))
//!     .duration_ms(1000)
//!     .playback_mode(PlaybackMode::Loop);
//!
//! animation.tick(500);
//! let value = animation.current_value();
//! ```

// Suppress pedantic lints
#![allow(missing_docs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::similar_names)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::use_self)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::type_complexity)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::double_must_use)]
#![allow(clippy::float_cmp)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::unused_self)]
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::absurd_extreme_comparisons)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::unnecessary_min_or_max)]
pub mod animated;
pub mod easing;
pub mod interpolate;
pub mod keyframe;
pub mod manager;
pub mod scheduler;
pub mod sequence;
pub mod spring;
pub mod transition;

pub use animated::{AnimatedLayout, AnimatedStyle, AnimationMode};
pub use easing::EasingFunction;
pub use interpolate::{lerp, Interpolator};
pub use keyframe::{Keyframe, KeyframeAnimation, PlaybackMode};
pub use manager::{AnimationManager, AnimationStats, ManagedId, ManagerState};
pub use scheduler::{Animation, AnimationId, AnimationScheduler};
pub use sequence::{AnimationController, AnimationGroup, AnimationSequence, SequenceId};
pub use spring::{SpringAnimation, SpringBuilder, SpringConfig};
pub use transition::{
    interpolate_color, interpolate_position, interpolate_size, Transition, TransitionBuilder,
    TransitionContext, TransitionExt, TransitionFrom, TransitionId, TransitionProperty,
    TransitionValue,
};
