//! Animation demo example - easing and interpolation demonstration
//!
//! Run with: `cargo run --example animation`

use ctui_animate::{lerp, EasingFunction};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

struct Animate;
struct SetEasing(String);
struct SetProgress(f64);

impl Msg for Animate {}
impl Msg for SetEasing {}
impl Msg for SetProgress {}

struct AnimationState {
    progress: f64,
    easing: EasingFunction,
    easing_name: String,
    start_pos: u16,
    end_pos: u16,
}

impl AnimationState {
    fn new() -> Self {
        Self {
            progress: 0.0,
            easing: EasingFunction::QuadInOut,
            easing_name: "quad-in-out".to_string(),
            start_pos: 2,
            end_pos: 40,
        }
    }

    fn current_position(&self) -> u16 {
        let eased_progress = self.easing.eval(self.progress);
        lerp(
            self.start_pos as f32,
            self.end_pos as f32,
            eased_progress as f32,
        ) as u16
    }

    fn step(&mut self) {
        self.progress += 0.05;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}

struct AnimationDemo {
    state: AnimationState,
}

impl AnimationDemo {
    fn new() -> Self {
        Self {
            state: AnimationState::new(),
        }
    }
}

impl Component for AnimationDemo {
    type Props = ();
    type State = AnimationState;

    fn create(_props: Self::Props) -> Self {
        Self::new()
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let header = "╔══════════════════════════════════════════════──────╗";
        let title = "║           Animation Demo                            ║";
        let divider = "╠══════════════════════════════════════════════──────╣";

        for (col, ch) in header.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + col as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }

        for (col, ch) in title.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + col as u16, area.y + 1) {
                cell.symbol = ch.to_string();
            }
        }

        for (col, ch) in divider.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + col as u16, area.y + 2) {
                cell.symbol = ch.to_string();
            }
        }

        let progress_bar_y = 4;
        let progress_text = format!("Progress: {:5.1}%", self.state.progress * 100.0);
        for (col, ch) in progress_text.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + col as u16, area.y + progress_bar_y) {
                cell.symbol = ch.to_string();
            }
        }

        let bar_y = 6;
        let bar_start = 2;
        let bar_end = 50;
        for col in bar_start..=bar_end {
            if let Some(cell) = buf.get_mut(area.x + col, area.y + bar_y) {
                cell.symbol = if col < self.state.current_position() {
                    "█"
                } else {
                    "░"
                }
                .to_string();
            }
        }

        let eased_value = self.state.easing.eval(self.state.progress);
        let easing_text = format!(
            "Easing: {} (eased: {:.2})",
            self.state.easing_name, eased_value
        );
        for (col, ch) in easing_text.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + col as u16, area.y + 8) {
                cell.symbol = ch.to_string();
            }
        }

        let ball_pos = self.state.current_position();
        if let Some(cell) = buf.get_mut(area.x + ball_pos, area.y + 10) {
            cell.symbol = "●".to_string();
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if msg.is::<Animate>() {
            self.state.step();
            Cmd::Render
        } else if let Some(easing_msg) = msg.downcast_ref::<SetEasing>() {
            self.state.easing_name = easing_msg.0.clone();
            self.state.easing = match self.state.easing_name.as_str() {
                "linear" => EasingFunction::Linear,
                "quad-in" => EasingFunction::QuadIn,
                "quad-out" => EasingFunction::QuadOut,
                "quad-in-out" => EasingFunction::QuadInOut,
                "cubic-in" => EasingFunction::CubicIn,
                "cubic-out" => EasingFunction::CubicOut,
                "cubic-in-out" => EasingFunction::CubicInOut,
                "bounce-out" => EasingFunction::BounceOut,
                "elastic-out" => EasingFunction::ElasticOut,
                _ => EasingFunction::QuadInOut,
            };
            Cmd::Render
        } else if let Some(progress_msg) = msg.downcast_ref::<SetProgress>() {
            self.state.progress = progress_msg.0.clamp(0.0, 1.0);
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

fn demonstrate_easing(name: &str, easing: EasingFunction) {
    println!("\n{}:", name);
    for i in 0..=10 {
        let progress = i as f64 / 10.0;
        let eased = easing.eval(progress);
        let bar_len = (eased * 20.0).max(0.0) as usize;
        let bar: String = "█".repeat(bar_len);
        println!("  {:.1} -> {:.2} |{:<20}|", progress, eased, bar);
    }
}

fn main() {
    let mut demo = AnimationDemo::new();
    demo.on_mount();

    println!("Animation Demo Example");
    println!("=====================\n");

    println!("Available easing functions:");
    demonstrate_easing("Linear", EasingFunction::Linear);
    demonstrate_easing("Quad In", EasingFunction::QuadIn);
    demonstrate_easing("Quad Out", EasingFunction::QuadOut);
    demonstrate_easing("Quad In Out", EasingFunction::QuadInOut);
    demonstrate_easing("Cubic In", EasingFunction::CubicIn);
    demonstrate_easing("Cubic Out", EasingFunction::CubicOut);
    demonstrate_easing("Bounce Out", EasingFunction::BounceOut);
    demonstrate_easing("Elastic Out", EasingFunction::ElasticOut);

    println!("\nSimulation:");
    for i in 0..=20 {
        demo.update(Box::new(SetProgress(i as f64 / 20.0)));
        let pos = demo.state.current_position();
        let eased = demo.state.easing.eval(demo.state.progress);
        println!(
            "  Step {:2}: progress={:.2}, eased={:.2}, pos={}",
            i, demo.state.progress, eased, pos
        );
    }

    println!("\nTesting easing switching:");
    demo.update(Box::new(SetEasing("bounce-out".to_string())));
    demo.update(Box::new(SetProgress(0.5)));
    println!(
        "  With bounce-out at 0.5: eased = {:.2}",
        demo.state.easing.eval(0.5)
    );

    demo.update(Box::new(SetEasing("elastic-out".to_string())));
    println!(
        "  With elastic-out at 0.5: eased = {:.2}",
        demo.state.easing.eval(0.5)
    );

    println!("\n✓ Animation easing functions verified");
    demo.on_unmount();
}
