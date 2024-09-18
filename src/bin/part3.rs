use plotters::prelude::*;
use rand::distributions::Distribution;
use cs5060hw2::{probabilities, EpsilonGreedy, ExploreRandom, ThompsonSample, STEPS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let root = BitMapBackend::new("part3.png", (640, 480))
    .into_drawing_area();
  root.fill(&WHITE)?;
  let mut chart = ChartBuilder::on(&root)
    .caption("Dynamic Environment", ("sans-serif", 25).into_font())
    .margin(5)
    .margin_right(20)
    .x_label_area_size(30)
    .y_label_area_size(30)
    .build_cartesian_2d(0..STEPS, -1.5f32..2f32)?;

  chart.configure_mesh().draw()?;

  let mut epsilon_greedy = EpsilonGreedy::new(21, ExploreRandom::new());

  let line1 = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let probabilities = probabilities(x);
        let choice = epsilon_greedy.pick(0.05);
        let reward = probabilities[choice].sample(&mut epsilon_greedy.rng);
        epsilon_greedy.update(choice, reward);
        (x, epsilon_greedy.reward())
      }),
    &RED,
  );

  let mut thompson = ThompsonSample::new(21);

  let line2 = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        let probabilities = probabilities(x);
        let choice = thompson.pick();
        let reward = probabilities[choice].sample(&mut thompson.rng);
        thompson.update(choice, reward);
        (x, thompson.reward())
      }),
    &BLUE,
  );

  let mut thompson = ThompsonSample::new(21);

  let line3 = LineSeries::new(
    (1..=STEPS)
      .map(|x| {
        if x == 3000 {
          thompson.experience = vec![(0,0); 21]
        }
        let probabilities = probabilities(x);
        let choice = thompson.pick();
        let reward = probabilities[choice].sample(&mut thompson.rng);
        thompson.update(choice, reward);
        (x, thompson.reward())
      }),
    &GREEN,
  );

  chart
    .draw_series(line1)?
    .label("epsilon = 0.05")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

  chart
    .draw_series(line2)?
    .label("Thompson")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

  chart
    .draw_series(line3)?
    .label("Thompson, reset at 3000")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

  chart
    .configure_series_labels()
    .background_style(&WHITE.mix(0.8))
    .border_style(&BLACK)
    .position(SeriesLabelPosition::UpperRight)
    .draw()?;

  root.present()?;

  Ok(())
}
