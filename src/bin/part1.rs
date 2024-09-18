use plotters::prelude::*;
use plotters::style::full_palette::{ORANGE, PURPLE};
use rand::distributions::Distribution;
use cs5060hw2::{probabilities, EpsilonGreedy, ExploreRandom, ThompsonSample, STEPS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let root = BitMapBackend::new("part1.png", (640, 480))
    .into_drawing_area();
  root.fill(&WHITE)?;
  let mut chart = ChartBuilder::on(&root)
    .caption("Greedy Epsilon", ("sans-serif", 25).into_font())
    .margin(5)
    .margin_right(20)
    .x_label_area_size(30)
    .y_label_area_size(30)
    // .build_cartesian_2d(0..STEPS, -100f32..100f32)?;
    .build_cartesian_2d(0..STEPS, -0.75f32..2.4f32)?;

  chart.configure_mesh().draw()?;

  let probabilities = probabilities(0.0);
  let mut epsilon_greedy = EpsilonGreedy::new(probabilities.len(), ExploreRandom::new());

  let line1 = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let choice = epsilon_greedy.pick(0.01);
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
        let choice = epsilon_greedy.pick(0.05);
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
        let choice = epsilon_greedy.pick(0.1);
        let reward = probabilities[choice].sample(&mut epsilon_greedy.rng);
        epsilon_greedy.update(choice, reward);
        (x, epsilon_greedy.reward())
      }),
    &GREEN,
  );

  let mut epsilon_greedy = EpsilonGreedy::new(probabilities.len(), ExploreRandom::new());

  let line4 = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let choice = epsilon_greedy.pick(0.4);
        let reward = probabilities[choice].sample(&mut epsilon_greedy.rng);
        epsilon_greedy.update(choice, reward);
        (x, epsilon_greedy.reward())
      }),
    &PURPLE,
  );

  let mut thompson = ThompsonSample::new(probabilities.len());

  let line5 = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let choice = thompson.pick();
        let reward = probabilities[choice].sample(&mut thompson.rng);
        thompson.update(choice, reward);
        (x, thompson.reward())
      }),
    &ORANGE,
  );

  chart
    .draw_series(line1)?
    .label("epsilon = 0.01")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

  chart
    .draw_series(line2)?
    .label("epsilon = 0.05")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

  chart
    .draw_series(line3)?
    .label("epsilon = 0.1")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

  chart
    .draw_series(line4)?
    .label("epsilon = 0.4")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &PURPLE));

  chart
    .draw_series(line5)?
    .label("Thomson Sampling")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &ORANGE));


  chart
    .configure_series_labels()
    .background_style(&WHITE.mix(0.8))
    .border_style(&BLACK)
    .position(SeriesLabelPosition::LowerRight)
    .draw()?;

  root.present()?;

  Ok(())
}
