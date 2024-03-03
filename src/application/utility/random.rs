use rand::distributions::uniform::SampleUniform;
use rand::Rng;

/// Generate a random number between `from` and `to`.
pub fn random<T>(from: T, to: T) -> T where T: SampleUniform + PartialOrd {
  rand::thread_rng().gen_range(from..to)
}

/// Choose a random item from a slice.
pub fn choose<T>(choices: &[T]) -> &T {
  let index = random(0, choices.len());
  &choices[index]
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_CHOICES: [i32; 5] = [1, 2, 3, 4, 5];
  const TEST_TIMES: i32 = 10;

  const TEST_FROM: i32 = 0;
  const TEST_TO: i32 = 10;

  #[test]
  fn test_random() {
    for _ in 0..TEST_TIMES {
      let result = random(TEST_FROM, TEST_TO);
      assert!(result >= TEST_FROM && result < TEST_TO);
    }
  }

  #[test]
  fn test_choose() {
    for _ in 0..TEST_TIMES {
      let result = choose(&TEST_CHOICES);
      assert!(TEST_CHOICES.contains(result));
    }
  }
}