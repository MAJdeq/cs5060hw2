use rand::distributions::{Distribution, WeightedIndex};
use rand::{Rng, SeedableRng};
use rand::prelude::SliceRandom;
use rand::rngs::{SmallRng};
use rand_distr::{Beta, Normal};

pub const STEPS: i32 = 10000;
pub fn probabilities(step: i32) -> Vec<Normal<f32>> {
  let drift = step as f32 * -0.001;

  let bandit0_offset = if step > 3000 {
    7.0
  } else {
    0.0
  };

  let bandit2_offset = if step > 3000 {
    3.0
  } else {
    0.0
  };

  let bandit7_offset = if step > 3000 {
    1.0
  } else {
    0.0
  };

  let bandit18_offset = if step > 3000 {
    2.0
  } else {
    0.0
  };

  vec![
    Normal::new(0.0 + drift + bandit0_offset, 5.0).unwrap(),
    Normal::new(-0.5 + drift, 12.0).unwrap(),
    Normal::new(2.0 + drift + bandit2_offset, 3.9).unwrap(),
    Normal::new(-0.5 + drift, 7.0).unwrap(),
    Normal::new(-1.2 + drift, 8.0).unwrap(),
    Normal::new(-3.0 + drift, 7.0).unwrap(),
    Normal::new(-10.0 + drift, 20.0).unwrap(),
    Normal::new(-0.5 + drift + bandit7_offset, 1.0).unwrap(),
    Normal::new(-1.0 + drift, 2.0).unwrap(),
    Normal::new(1.0 + drift, 6.0).unwrap(),
    Normal::new(0.7 + drift, 4.0).unwrap(),
    Normal::new(-6.0 + drift, 11.0).unwrap(),
    Normal::new(-7.0 + drift, 1.0).unwrap(),
    Normal::new(-0.5 + drift, 2.0).unwrap(),
    Normal::new(-6.5 + drift, 1.0).unwrap(),
    Normal::new(-3.0 + drift, 6.0).unwrap(),
    Normal::new(0.0 + drift, 8.0).unwrap(),
    Normal::new(2.0 + drift, 3.9).unwrap(),
    Normal::new(-9.0 + drift + bandit18_offset, 12.0).unwrap(),
    Normal::new(-1.0 + drift, 6.0).unwrap(),
    Normal::new(-4.5 + drift, 8.0).unwrap()
  ]
}

pub trait ExplorationStrategy {
  fn pick(&mut self, explore: bool, choices: &Vec<Vec<f32>>, current_best: usize) -> usize;
}

pub struct EpsilonGreedy<T: ExplorationStrategy> {
  pub rng: SmallRng,
  pub expected_rewards: Vec<Vec<f32>>,
  strategy: T
}

impl<T: ExplorationStrategy> EpsilonGreedy<T> {
  pub fn new(n: usize, strategy: T) -> Self {
    Self {
      rng: SmallRng::from_entropy(),
      expected_rewards: vec![vec![0.0]; n],
      strategy
    }
  }

  pub fn reward(&self) -> f32 {
    let total_len = self.expected_rewards.iter().fold(0, |acc, x| acc + x.len());
    self.expected_rewards.iter().fold(0.0, |acc, x| acc + x.iter().sum::<f32>()) / total_len as f32
  }

  pub fn pick(&mut self, epsilon: f32) -> usize {
    let current_best = self.expected_rewards
      .iter()
      .map(|x| x.iter().sum::<f32>())
      .enumerate()
      .max_by(|x, y| x.1.total_cmp(&y.1))
      .unwrap().0;

    let explore = self.rng.gen_range(0f32..1f32) < epsilon;

    self.strategy.pick(explore, &self.expected_rewards, current_best)
  }

  pub fn update(&mut self, bandit: usize, reward: f32) {
    self.expected_rewards[bandit].push(reward);
  }
}

pub struct ExploreRandom {
  rng: SmallRng,
}

impl ExploreRandom {
  pub fn new() -> Self {
    Self {
      rng: SmallRng::from_entropy(),
    }
  }
}

impl ExplorationStrategy for ExploreRandom {
  fn pick(&mut self, explore: bool, choices: &Vec<Vec<f32>>, current_best: usize) -> usize {
    if explore {
      self.rng.gen_range(0..choices.len())
    } else {
      current_best
    }
  }
}

pub struct ExploreNotBest {
  rng: SmallRng,
}

impl ExploreNotBest {
  pub fn new() -> Self {
    Self {
      rng: SmallRng::from_entropy(),
    }
  }
}

impl ExplorationStrategy for ExploreNotBest {
  fn pick(&mut self, explore: bool, choices: &Vec<Vec<f32>>, current_best: usize) -> usize {
    if explore {
      let mut choices: Vec<usize> = (0..choices.len()).filter(|x| *x != current_best).collect();
      choices.shuffle(&mut self.rng);
      choices[0]
    } else {
      current_best
    }
  }
}

pub struct ExploreWeighted {
  rng: SmallRng,
}

impl ExploreWeighted {
  pub fn new() -> Self {
    Self {
      rng: SmallRng::from_entropy(),
    }
  }
}

impl ExplorationStrategy for ExploreWeighted {
  fn pick(&mut self, explore: bool, choices: &Vec<Vec<f32>>, current_best: usize) -> usize {
    if explore {
      let weights_raw = choices.iter().map(|x| 1.0 / (x.len() + 1) as f32);
      let weight_sum = (&weights_raw).clone().sum::<f32>();
      let weights = weights_raw.map(|x| x / weight_sum);
      let dist = WeightedIndex::new(weights).unwrap();
      dist.sample(&mut self.rng)
    } else {
      current_best
    }
  }
}

pub struct ThompsonSample {
  pub rng: SmallRng,
  pub experience: Vec<(i32, i32)>,
  pub rewards: Vec<Vec<f32>>,
}

impl ThompsonSample {
  pub fn new(n: usize) -> Self {
    Self {
      rng: SmallRng::from_entropy(),
      experience: vec![(0, 0); n],
      rewards: vec![vec![]; n],
    }
  }

  pub fn reward(&self) -> f32 {
    let total_len = self.rewards.iter().fold(0, |acc, x| acc + x.len());
    self.rewards.iter().fold(0.0, |acc, x| acc + x.iter().sum::<f32>()) / total_len as f32
  }

  pub fn pick(&mut self) -> usize {
    self.experience
      .iter()
      .map(|(a, b)| {
        let dist = Beta::new((*a + 1) as f32, (*b + 1) as f32).unwrap();
        dist.sample(&mut self.rng)
      })
      .enumerate()
      .max_by(|x, y| x.1.total_cmp(&y.1))
      .unwrap().0
  }

  pub fn update(&mut self, bandit: usize, reward: f32) {
    if reward > 0.0 {
      self.experience[bandit].0 += 1
    } else {
      self.experience[bandit].1 += 1
    }
    self.rewards[bandit].push(reward);
  }
}