use plotters::prelude::*;
use rand::distributions::Distribution;
use cs5060hw2::{probabilities, EpsilonGreedy, ExploreNotBest, ExploreRandom, ExploreWeighted, STEPS};

fn linear_quench(t: i32) -> f32 {
  0f32.max(1.0 - (t as f32 / STEPS as f32))
}

fn asymptotic_quench(t: i32) -> f32 {
  1.0 / (1.0 + 0.01 * t as f32)
}

fn heavy_asymptotic_quench(t: i32) -> f32 {
  1.0 / (1.0 + 0.0001 * (t * t) as f32)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let root = BitMapBackend::new("part2-1.png", (640, 480))
    .into_drawing_area();
  root.fill(&WHITE)?;
  let mut chart = ChartBuilder::on(&root)
    .caption("Greedy Epsilon", ("sans-serif", 25).into_font())
    .margin(5)
    .margin_right(20)
    .x_label_area_size(30)
    .y_label_area_size(30)
    .build_cartesian_2d(0..STEPS, -2f32..2.4f32)?;

  chart.configure_mesh().draw()?;

  let probabilities = probabilities(0);
  let mut epsilon_greedy = EpsilonGreedy::new(probabilities.len(), ExploreRandom::new());


  let line1 = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let choice = epsilon_greedy.pick(linear_quench(x));
        let reward = probabilities[choice].sample(&mut epsilon_greedy.rng);
        epsilon_greedy.update(choice, reward);
        (x, epsilon_greedy.reward())
      }),
    &RED,
  );

  let mut epsilon_greedy = EpsilonGreedy::new(probabilities.len(), ExploreRandom::new());

  let line2 = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let choice = epsilon_greedy.pick(asymptotic_quench(x));
        let reward = probabilities[choice].sample(&mut epsilon_greedy.rng);
        epsilon_greedy.update(choice, reward);
        (x, epsilon_greedy.reward())
      }),
    &BLUE,
  );

  let mut epsilon_greedy = EpsilonGreedy::new(probabilities.len(), ExploreRandom::new());

  let line3 = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let choice = epsilon_greedy.pick(heavy_asymptotic_quench(x));
        let reward = probabilities[choice].sample(&mut epsilon_greedy.rng);
        epsilon_greedy.update(choice, reward);
        (x, epsilon_greedy.reward())
      }),
    &GREEN,
  );

  chart
    .draw_series(line1)?
    .label("linear Quench")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

  chart
    .draw_series(line2)?
    .label("Asymptotic Quench")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

  chart
    .draw_series(line3)?
    .label("Heavy Asymptotic Quench")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

  chart
    .configure_series_labels()
    .background_style(&WHITE.mix(0.8))
    .border_style(&BLACK)
    .draw()?;

  root.present()?;

  let root = BitMapBackend::new("part2-2.png", (640, 480))
    .into_drawing_area();
  root.fill(&WHITE)?;
  let mut chart = ChartBuilder::on(&root)
    .caption("Exploration methods (Heavy Asymptotic Quench)", ("sans-serif", 25).into_font())
    .margin(5)
    .margin_right(20)
    .x_label_area_size(30)
    .y_label_area_size(30)
    .build_cartesian_2d(0..STEPS, -0.25f32..2.4f32)?;

  chart.configure_mesh().draw()?;

  let mut explore_random = EpsilonGreedy::new(probabilities.len(), ExploreRandom::new());

  let random = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let choice = explore_random.pick(heavy_asymptotic_quench(x));
        let reward = probabilities[choice].sample(&mut explore_random.rng);
        explore_random.update(choice, reward);
        (x, explore_random.reward())
      }),
    &RED,
  );

  let mut explore_not_best = EpsilonGreedy::new(probabilities.len(), ExploreNotBest::new());

  let not_best = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let choice = explore_not_best.pick(heavy_asymptotic_quench(x));
        let reward = probabilities[choice].sample(&mut explore_not_best.rng);
        explore_not_best.update(choice, reward);
        (x, explore_not_best.reward())
      }),
    &BLUE,
  );

  let mut explore_weighted = EpsilonGreedy::new(probabilities.len(), ExploreWeighted::new());

  let weighted = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let choice = explore_weighted.pick(heavy_asymptotic_quench(x));
        let reward = probabilities[choice].sample(&mut explore_weighted.rng);
        explore_weighted.update(choice, reward);
        (x, explore_weighted.reward())
      }),
    &GREEN,
  );

  chart
    .draw_series(random)?
    .label("Explore Random")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

  chart
    .draw_series(not_best)?
    .label("Explore Away from Best")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

  chart
    .draw_series(weighted)?
    .label("Explore Weighted")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

  chart
    .configure_series_labels()
    .background_style(&WHITE.mix(0.8))
    .border_style(&BLACK)
    .draw()?;

  root.present()?;

  Ok(())
}