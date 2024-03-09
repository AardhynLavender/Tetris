use std::time::{Duration, Instant};

fn now() -> Instant {
  Instant::now()
}

#[derive(Debug)]
pub struct Timer {
  enabled: bool,
  start: Instant,
  duration: Duration,
}

pub enum ConsumeAction {
  Restart,
  Disable,
}

impl Timer {
  pub fn new(duration: Duration, enabled: bool) -> Self {
    Self {
      enabled,
      start: now(),
      duration,
    }
  }

  /// Check if the timer has expired regardless of enabled state
  pub fn done(&self) -> bool {
    self.start.elapsed() >= self.duration
  }

  /// Start the timer
  pub fn start(&mut self) {
    self.start = now();
    self.enabled = true;
  }
  /// set the start time to now
  pub fn restart(&mut self) {
    self.start = now();
  }

  /// Check if the timer is enabled
  pub fn enabled(&self) -> bool {
    self.enabled
  }
  /// Stop the timer
  ///
  /// > If the time is already done, Timer::done() will still return true
  pub fn disable(&mut self) {
    self.enabled = false;
  }

  /// Check if the timer has expired and disable it if it has.
  ///
  /// > Essentially a mutable version Timer::done()
  pub fn consume(&mut self, action: ConsumeAction) -> bool {
    if !self.enabled {
      return false;
    }

    let done = self.done();
    if done {
      match action {
        ConsumeAction::Restart => self.restart(), // timer will be done again after duration
        ConsumeAction::Disable => self.enabled = false, // timer will not be done again
      }
    }
    done
  }
  /// Check if the timer has expired but reset each invocation
  pub fn debounce(&mut self) -> bool {
    if !self.enabled {
      return false;
    }

    let done = self.done();
    self.restart();
    done
  }
}

type TimeoutCounter = u32;

#[derive(PartialEq)]
pub enum Repeat {
  Never,
  Forever,
  For(TimeoutCounter),
}

pub enum TimeoutState {
  Active,
  Expired,
}

/// Invoke a function after a certain amount of time has passed without taking ownership of the function.
pub struct Timeout {
  timer: Timer,
  repeat: Repeat,
  counter: u32,
  dead: bool,
}

impl Timeout {
  pub fn new(duration: Duration, repeat: Repeat) -> Self {
    Self {
      timer: Timer::new(duration, true),
      repeat,
      counter: 0,
      dead: false,
    }
  }

  /// Restart the timer regardless of state
  pub fn reset(&mut self) {
    self.timer.restart();
  }

  /// Restart the timer and reset the counter
  pub fn hard_reset(&mut self) {
    self.dead = false;
    self.counter = 0;
    self.reset();
  }

  /// Check if the timeout has expired and invoke the visitor if it has.
  /// ## Returns
  /// - `Ok(())` timeout has not expired
  /// - `Err(())` timeout has expired
  pub fn on_timeout(&mut self, visitor: &mut dyn FnMut()) -> TimeoutState {
    if self.dead {
      return TimeoutState::Expired;
    }

    if self.timer.done() {
      // invoke visitor
      (visitor)();
      self.counter += 1;

      if self.repeat == Repeat::Never {
        self.dead = true;
      } else if let Repeat::For(times) = self.repeat {
        if self.counter >= times {
          self.dead = true;
        }
      } else {
        self.timer.restart();
      }
    }

    TimeoutState::Active
  }
}