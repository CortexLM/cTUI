//! Integration tests for the animation system

use approx::assert_relative_eq;
use ctui_animate::{
    lerp, Animation, AnimationId, AnimationScheduler, EasingFunction, Keyframe, KeyframeAnimation,
    PlaybackMode,
};
use ctui_core::backend::test::TestBackend;
use ctui_core::{Buffer, Rect, Terminal, Widget};

#[test]
fn test_scheduler_spawn_and_tick() {
    let mut scheduler = AnimationScheduler::new();

    let id = scheduler.spawn(1000, EasingFunction::Linear);
    assert!(scheduler.is_active(id));
    assert_eq!(scheduler.active_count(), 1);

    let updates = scheduler.tick(250);
    assert_eq!(updates.len(), 1);
    assert_relative_eq!(updates[0].1, 0.25, epsilon = 1e-6);
}

#[test]
fn test_scheduler_multiple_animations() {
    let mut scheduler = AnimationScheduler::new();

    let id1 = scheduler.spawn(1000, EasingFunction::Linear);
    let id2 = scheduler.spawn(500, EasingFunction::QuadOut);
    let id3 = scheduler.spawn(250, EasingFunction::CubicInOut);

    assert_eq!(scheduler.active_count(), 3);

    let mut updates = scheduler.tick(500);
    updates.sort_by_key(|(id, _)| id.0);

    assert!(!scheduler.is_active(id2));
    assert!(!scheduler.is_active(id3));
    assert!(scheduler.is_active(id1));
    assert_eq!(scheduler.active_count(), 1);
}

#[test]
fn test_scheduler_cancel() {
    let mut scheduler = AnimationScheduler::new();

    let id = scheduler.spawn(1000, EasingFunction::Linear);
    assert!(scheduler.is_active(id));

    scheduler.cancel(id);
    assert!(!scheduler.is_active(id));
    assert_eq!(scheduler.active_count(), 0);
}

#[test]
fn test_scheduler_clear() {
    let mut scheduler = AnimationScheduler::new();

    scheduler.spawn(1000, EasingFunction::Linear);
    scheduler.spawn(1000, EasingFunction::Linear);
    scheduler.spawn(1000, EasingFunction::Linear);

    assert_eq!(scheduler.active_count(), 3);

    scheduler.clear();

    assert_eq!(scheduler.active_count(), 0);
}

#[test]
fn test_easing_linear() {
    assert_relative_eq!(EasingFunction::Linear.eval(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(EasingFunction::Linear.eval(0.5), 0.5, epsilon = 1e-10);
    assert_relative_eq!(EasingFunction::Linear.eval(1.0), 1.0, epsilon = 1e-10);
}

#[test]
fn test_easing_quad_out() {
    assert_relative_eq!(EasingFunction::QuadOut.eval(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(EasingFunction::QuadOut.eval(0.5), 0.75, epsilon = 1e-10);
    assert_relative_eq!(EasingFunction::QuadOut.eval(1.0), 1.0, epsilon = 1e-10);
}

#[test]
fn test_easing_cubic_out() {
    assert_relative_eq!(EasingFunction::CubicOut.eval(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(EasingFunction::CubicOut.eval(0.5), 0.875, epsilon = 1e-10);
    assert_relative_eq!(EasingFunction::CubicOut.eval(1.0), 1.0, epsilon = 1e-10);
}

#[test]
fn test_easing_clamping() {
    assert_relative_eq!(EasingFunction::Linear.eval(-1.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(EasingFunction::Linear.eval(2.0), 1.0, epsilon = 1e-10);
}

#[test]
fn test_animation_progress() {
    let anim = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);
    assert_relative_eq!(anim.progress(), 0.0, epsilon = 1e-6);

    let mut anim = anim;
    anim.elapsed_ms = 500;
    assert_relative_eq!(anim.progress(), 0.5, epsilon = 1e-6);
}

#[test]
fn test_animation_eased_progress() {
    let mut anim = Animation::new(AnimationId(1), 1000, EasingFunction::QuadOut);
    anim.elapsed_ms = 500;

    let eased = anim.eased_progress();
    assert_relative_eq!(eased, 0.75, epsilon = 1e-6);
}

#[test]
fn test_animation_completion() {
    let mut anim = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);
    assert!(!anim.is_complete());

    anim.elapsed_ms = 1000;
    assert!(anim.is_complete());

    anim.elapsed_ms = 1500;
    assert!(anim.is_complete());
}

#[test]
fn test_lerp_basic() {
    assert_relative_eq!(lerp(0.0, 100.0, 0.0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(lerp(0.0, 100.0, 0.5), 50.0, epsilon = 1e-6);
    assert_relative_eq!(lerp(0.0, 100.0, 1.0), 100.0, epsilon = 1e-6);
}

#[test]
fn test_lerp_negative() {
    assert_relative_eq!(lerp(-50.0, 50.0, 0.5), 0.0, epsilon = 1e-6);
    assert_relative_eq!(lerp(-100.0, 0.0, 0.25), -75.0, epsilon = 1e-6);
}

#[test]
fn test_keyframe_animation_basic() {
    let mut anim = KeyframeAnimation::new()
        .keyframe(Keyframe::new(0.0, 0.0))
        .keyframe(Keyframe::new(0.5, 50.0))
        .keyframe(Keyframe::new(1.0, 100.0))
        .duration_ms(1000);

    assert_relative_eq!(anim.current_value().unwrap_or(0.0), 0.0, epsilon = 1e-6);

    anim.tick(500);
    assert_relative_eq!(anim.current_value().unwrap_or(0.0), 50.0, epsilon = 1e-6);

    anim.tick(500);
    assert_relative_eq!(anim.current_value().unwrap_or(0.0), 100.0, epsilon = 1e-6);
}

#[test]
fn test_keyframe_animation_loop() {
    let mut anim = KeyframeAnimation::new()
        .keyframe(Keyframe::new(0.0, 0.0))
        .keyframe(Keyframe::new(1.0, 100.0))
        .duration_ms(1000)
        .playback_mode(PlaybackMode::Loop);

    anim.tick(500);
    assert_relative_eq!(anim.current_value().unwrap_or(0.0), 50.0, epsilon = 1e-6);

    anim.tick(600);
    assert_relative_eq!(anim.current_value().unwrap_or(0.0), 10.0, epsilon = 1e-6);
}

#[test]
fn test_keyframe_animation_reverse() {
    let mut anim = KeyframeAnimation::new()
        .keyframe(Keyframe::new(0.0, 0.0))
        .keyframe(Keyframe::new(1.0, 100.0))
        .duration_ms(1000)
        .playback_mode(PlaybackMode::Reverse);

    anim.tick(500);
    assert_relative_eq!(anim.current_value().unwrap_or(0.0), 50.0, epsilon = 1e-6);
}

#[test]
fn test_keyframe_animation_pingpong() {
    let mut anim = KeyframeAnimation::new()
        .keyframe(Keyframe::new(0.0, 0.0))
        .keyframe(Keyframe::new(1.0, 100.0))
        .duration_ms(1000)
        .playback_mode(PlaybackMode::PingPong);

    anim.tick(1000);
    assert_relative_eq!(anim.current_value().unwrap_or(0.0), 100.0, epsilon = 1e-6);

    anim.tick(500);
    assert_relative_eq!(anim.current_value().unwrap_or(0.0), 50.0, epsilon = 1e-6);
}

#[test]
fn test_animation_with_frame_rate() {
    let mut scheduler = AnimationScheduler::new();
    let id = scheduler.spawn(1000, EasingFunction::Linear);

    let frame_count = 10;
    let frame_time = 100;
    let mut progress_values: Vec<f32> = Vec::new();

    for i in 0..frame_count {
        let updates = scheduler.tick(frame_time);
        if !updates.is_empty() {
            progress_values.push(updates[0].1);
        }
    }

    assert!(!progress_values.is_empty());
    assert_relative_eq!(progress_values[0], 0.1, epsilon = 0.01);
}

#[test]
fn test_easing_variants_consistency() {
    let easings = [
        EasingFunction::QuadIn,
        EasingFunction::QuadOut,
        EasingFunction::CubicIn,
        EasingFunction::CubicOut,
        EasingFunction::QuartIn,
        EasingFunction::QuartOut,
        EasingFunction::SineIn,
        EasingFunction::SineOut,
        EasingFunction::ExpoIn,
        EasingFunction::ExpoOut,
        EasingFunction::CircIn,
        EasingFunction::CircOut,
        EasingFunction::BackIn,
        EasingFunction::BackOut,
        EasingFunction::ElasticIn,
        EasingFunction::ElasticOut,
        EasingFunction::BounceIn,
        EasingFunction::BounceOut,
    ];

    for easing in easings {
        assert_relative_eq!(easing.eval(0.0), 0.0, epsilon = 1e-6);
        assert_relative_eq!(easing.eval(1.0), 1.0, epsilon = 1e-6);
    }
}

struct AnimatedWidget {
    progress: f32,
}

impl Widget for AnimatedWidget {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let width = (area.width as f32 * self.progress).min(area.width as f32) as u16;
        for x in area.x..area.x.saturating_add(width) {
            buf.modify_cell(x, area.y, |cell| { cell.symbol = "#".to_string(); });
        }
    }
}

#[test]
fn test_animated_render_integration() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut scheduler = AnimationScheduler::new();
    let _id = scheduler.spawn(1000, EasingFunction::Linear);

    for frame in 0..=10 {
        let progress = frame as f32 / 10.0;
        let widget = AnimatedWidget { progress };

        terminal
            .draw(|f| {
                widget.render(Rect::new(0, 0, 80, 1), f.buffer_mut());
            })
            .unwrap();
    }

    let backend = terminal.backend();
    assert_eq!(backend.buffer().get(0, 0).unwrap().symbol, "#");
}

#[test]
fn test_animation_pause_resume() {
    let mut scheduler = AnimationScheduler::new();
    let id = scheduler.spawn(1000, EasingFunction::Linear);

    scheduler.tick(200);

    let updates1 = scheduler.tick(200);
    let progress1 = updates1[0].1;

    scheduler.cancel(id);
    assert!(!scheduler.is_active(id));

    let _id2 = scheduler.spawn(600, EasingFunction::Linear);
    let updates2 = scheduler.tick(300);
    let progress2 = updates2[0].1;

    assert_relative_eq!(progress1, 0.4, epsilon = 1e-6);
    assert_relative_eq!(progress2, 0.5, epsilon = 1e-6);
}

#[test]
fn test_animation_sequence() {
    let mut scheduler = AnimationScheduler::new();

    let id1 = scheduler.spawn(500, EasingFunction::Linear);
    let updates1 = scheduler.tick(250);
    assert_relative_eq!(updates1[0].1, 0.5, epsilon = 1e-6);

    scheduler.tick(250);
    assert!(!scheduler.is_active(id1));

    let id2 = scheduler.spawn(500, EasingFunction::QuadOut);
    assert!(scheduler.is_active(id2));
    assert_eq!(scheduler.active_count(), 1);
}

#[test]
fn test_active_animation_ids() {
    let mut scheduler = AnimationScheduler::new();

    let id1 = scheduler.spawn(1000, EasingFunction::Linear);
    let id2 = scheduler.spawn(1000, EasingFunction::Linear);
    let id3 = scheduler.spawn(1000, EasingFunction::Linear);

    let ids: Vec<_> = scheduler.active_ids().collect();
    assert_eq!(ids.len(), 3);
    assert!(ids.contains(&id1));
    assert!(ids.contains(&id2));
    assert!(ids.contains(&id3));
}

#[test]
fn test_animation_advance_overflow() {
    let mut anim = Animation::new(AnimationId(1), 1000, EasingFunction::Linear);
    anim.elapsed_ms = u64::MAX - 100;
    anim.advance(200);
    assert_eq!(anim.elapsed_ms, u64::MAX);
}

#[test]
fn test_zero_duration_animation() {
    let mut scheduler = AnimationScheduler::new();
    let id = scheduler.spawn(0, EasingFunction::Linear);

    let updates = scheduler.tick(0);
    assert!(!scheduler.is_active(id));
    assert_relative_eq!(updates[0].1, 1.0, epsilon = 1e-6);
}
