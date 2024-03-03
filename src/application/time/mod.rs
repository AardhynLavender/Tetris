use std::time::{Duration, Instant};

fn now() -> Instant {
  Instant::now()
}

#[derive(Debug)]
pub struct Timer {
  start: Instant,
  duration: Duration,
}

impl Timer {
  pub fn new(duration: Duration) -> Self {
    Self {
      start: now(),
      duration,
    }
  }

  /// Reset the timer
  pub fn reset(&mut self) {
    self.start = now();
  }

  /// Check if the timer has expired
  pub fn done(&self) -> bool {
    self.start.elapsed() >= self.duration
  }

  /// Check if the timer has expired and reset it if it has.
  pub fn consume(&mut self) -> bool {
    let done = self.done();
    if done {
      self.reset();
    }
    done
  }
  /// Check if the timer has expired but reset each invocation
  pub fn debounce(&mut self) -> bool {
    let done = self.done();
    self.reset();
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
      timer: Timer::new(duration),
      repeat,
      counter: 0,
      dead: false,
    }
  }

  /// Restart the timer regardless of state
  pub fn reset(&mut self) {
    self.timer.reset();
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
        self.timer.reset();
      }
    }

    TimeoutState::Active
  }
}