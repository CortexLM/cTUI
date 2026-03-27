use std::collections::VecDeque;
use std::time::{Duration, Instant};

type Callback = Box<dyn FnMut(f64)>;
type UpdateCallback = Box<dyn FnMut(f64)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy)]
pub struct FrameStats {
    pub fps: f64,
    pub frame_time_ms: f64,
    pub total_frames: u64,
    pub elapsed_time: Duration,
}

impl Default for FrameStats {
    fn default() -> Self {
        Self {
            fps: 0.0,
            frame_time_ms: 0.0,
            total_frames: 0,
            elapsed_time: Duration::ZERO,
        }
    }
}

pub struct RenderLoop {
    state: LoopState,
    target_fps: f64,
    frame_interval: Duration,
    last_frame: Instant,
    start_time: Instant,
    frame_count: u64,
    update_callbacks: Vec<(u64, UpdateCallback)>,
    render_callback: Option<Callback>,
    stats: FrameStats,
    fps_history: VecDeque<f64>,
    max_history: usize,
}

impl Default for RenderLoop {
    fn default() -> Self {
        Self {
            state: LoopState::Stopped,
            target_fps: 60.0,
            frame_interval: Duration::from_secs_f64(1.0 / 60.0),
            last_frame: Instant::now(),
            start_time: Instant::now(),
            frame_count: 0,
            update_callbacks: Vec::new(),
            render_callback: None,
            stats: FrameStats::default(),
            fps_history: VecDeque::with_capacity(60),
            max_history: 60,
        }
    }
}

impl RenderLoop {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_fps(mut self, fps: f64) -> Self {
        self.target_fps = fps.max(1.0);
        self.frame_interval = Duration::from_secs_f64(1.0 / self.target_fps);
        self
    }

    pub fn with_target_fps(mut self, fps: f64) -> Self {
        self.target_fps = fps.max(1.0);
        self.frame_interval = Duration::from_secs_f64(1.0 / self.target_fps);
        self
    }

    pub fn target_fps(&self) -> f64 {
        self.target_fps
    }

    pub fn state(&self) -> LoopState {
        self.state
    }

    pub fn is_running(&self) -> bool {
        self.state == LoopState::Running
    }

    pub fn is_paused(&self) -> bool {
        self.state == LoopState::Paused
    }

    pub fn stats(&self) -> &FrameStats {
        &self.stats
    }

    pub fn start(&mut self) {
        if self.state == LoopState::Running {
            return;
        }
        self.state = LoopState::Running;
        self.start_time = Instant::now();
        self.last_frame = self.start_time;
        self.frame_count = 0;
    }

    pub fn stop(&mut self) {
        self.state = LoopState::Stopped;
    }

    pub fn pause(&mut self) {
        if self.state == LoopState::Running {
            self.state = LoopState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.state == LoopState::Paused {
            self.state = LoopState::Running;
            self.last_frame = Instant::now();
        }
    }

    pub fn set_render_callback<F: FnMut(f64) + 'static>(&mut self, callback: F) {
        self.render_callback = Some(Box::new(callback));
    }

    pub fn add_update_callback<F: FnMut(f64) + 'static>(&mut self, interval_ms: u64, callback: F) {
        self.update_callbacks
            .push((interval_ms, Box::new(callback)));
    }

    pub fn clear_callbacks(&mut self) {
        self.update_callbacks.clear();
        self.render_callback = None;
    }

    pub fn tick(&mut self) -> bool {
        if self.state != LoopState::Running {
            return false;
        }

        let now = Instant::now();
        let elapsed = now - self.last_frame;

        if elapsed < self.frame_interval {
            return false;
        }

        let delta = elapsed.as_secs_f64();

        for (interval_ms, callback) in &mut self.update_callbacks {
            if self.frame_count % *interval_ms == 0 {
                callback(delta);
            }
        }

        if let Some(ref mut callback) = self.render_callback {
            callback(delta);
        }

        self.last_frame = now;
        self.frame_count += 1;

        let frame_time_ms = delta * 1000.0;
        let instant_fps = if frame_time_ms > 0.0 {
            1000.0 / frame_time_ms
        } else {
            0.0
        };

        self.fps_history.push_back(instant_fps);
        if self.fps_history.len() > self.max_history {
            self.fps_history.pop_front();
        }

        let avg_fps = if self.fps_history.is_empty() {
            0.0
        } else {
            self.fps_history.iter().sum::<f64>() / self.fps_history.len() as f64
        };

        self.stats = FrameStats {
            fps: avg_fps,
            frame_time_ms,
            total_frames: self.frame_count,
            elapsed_time: now - self.start_time,
        };

        true
    }

    pub fn delta_time(&self) -> f64 {
        self.stats.frame_time_ms / 1000.0
    }

    pub fn delta_time_ms(&self) -> f64 {
        self.stats.frame_time_ms
    }
}

pub struct TimedCallback {
    interval: Duration,
    accumulated: Duration,
    callback: Box<dyn FnMut()>,
}

impl TimedCallback {
    pub fn new(interval_ms: u64, callback: Box<dyn FnMut()>) -> Self {
        Self {
            interval: Duration::from_millis(interval_ms),
            accumulated: Duration::ZERO,
            callback,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.accumulated += delta;
        if self.accumulated >= self.interval {
            self.accumulated = Duration::ZERO;
            (self.callback)();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_render_loop_new() {
        let loop_ = RenderLoop::new();
        assert_eq!(loop_.state(), LoopState::Stopped);
        assert!(!loop_.is_running());
        assert_eq!(loop_.target_fps(), 60.0);
    }

    #[test]
    fn test_render_loop_with_fps() {
        let loop_ = RenderLoop::new().with_fps(30.0);
        assert_eq!(loop_.target_fps(), 30.0);
    }

    #[test]
    fn test_render_loop_start_stop() {
        let mut loop_ = RenderLoop::new();
        assert_eq!(loop_.state(), LoopState::Stopped);

        loop_.start();
        assert_eq!(loop_.state(), LoopState::Running);
        assert!(loop_.is_running());

        loop_.stop();
        assert_eq!(loop_.state(), LoopState::Stopped);
        assert!(!loop_.is_running());
    }

    #[test]
    fn test_render_loop_pause_resume() {
        let mut loop_ = RenderLoop::new();
        loop_.start();
        assert!(loop_.is_running());

        loop_.pause();
        assert!(loop_.is_paused());
        assert!(!loop_.is_running());

        loop_.resume();
        assert!(loop_.is_running());
        assert!(!loop_.is_paused());
    }

    #[test]
    fn test_render_loop_callback() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let mut loop_ = RenderLoop::new().with_fps(1000.0);
        loop_.set_render_callback(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        loop_.start();

        std::thread::sleep(Duration::from_millis(10));

        let mut ticks = 0;
        for _ in 0..100 {
            if loop_.tick() {
                ticks += 1;
            }
            std::thread::sleep(Duration::from_micros(100));
        }

        assert!(ticks > 0);
        assert!(counter.load(Ordering::SeqCst) > 0);
    }

    #[test]
    fn test_render_loop_update_callback() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let mut loop_ = RenderLoop::new().with_fps(10000.0);
        loop_.add_update_callback(1, move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        loop_.start();

        let mut successful_ticks = 0;
        for _ in 0..100 {
            if loop_.tick() {
                successful_ticks += 1;
            }
            std::thread::sleep(Duration::from_micros(200));
            if successful_ticks >= 5 {
                break;
            }
        }

        assert!(
            counter.load(Ordering::SeqCst) >= 5,
            "Expected at least 5 callback calls, got {}, successful_ticks={}",
            counter.load(Ordering::SeqCst),
            successful_ticks
        );
    }

    #[test]
    fn test_frame_stats_default() {
        let stats = FrameStats::default();
        assert_eq!(stats.fps, 0.0);
        assert_eq!(stats.frame_time_ms, 0.0);
        assert_eq!(stats.total_frames, 0);
    }

    #[test]
    fn test_timed_callback() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let mut callback = TimedCallback::new(
            10,
            Box::new(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }),
        );

        assert!(!callback.update(Duration::from_millis(5)));
        assert!(callback.update(Duration::from_millis(10)));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
