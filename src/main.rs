#![allow(dead_code)]
#![allow(unused_variables)]
use indicatif::{ProgressBar, ProgressStyle};

use rand_distr::{Distribution, Normal};
use rand::rng;


const SIGMA: f64 = 1.;

fn main() {


    println!("{:?}", step_1_simulation(
                SimulationParameters {
                    simulations_per_sigma: 100,
                    starting_sigma: 0.1,
                    sigma_increment: 0.1,
                    num_sigmas_to_consider: 100,
                    
                    s_guess_start: -0.1,
                    s_guess_increment: 0.001,
                    s_guesses_to_consider: 600,

                    r: 0.1,
                    time_steps_considered: 25,
                    total_time_steps: 80,
                    mean: 0.
                }

            ));
    
}

struct SimulationParameters {
    simulations_per_sigma: usize,

    starting_sigma: f64,
    sigma_increment: f64,
    num_sigmas_to_consider: usize,

    s_guess_increment: f64,
    s_guesses_to_consider: usize,
    s_guess_start: f64,

    r: f64,
    time_steps_considered: usize,
    total_time_steps: usize,

    mean: f64,
}
fn y(series: &[f64], start_ind: usize, r: f64) -> f64 {
    series.iter().enumerate().map(|(i,x)| if i < start_ind {0.} else{x * (1.+r).powf(-(i as f64-start_ind as f64))}).sum()
}
fn step_1_simulation(parameters: SimulationParameters) -> Vec<f64> {
    println!("Simulating...");

    let std_normal = Normal::new(0., 1.).unwrap();
    let mut sigma_s_map = Vec::new();
    for i in 0..parameters.num_sigmas_to_consider {
        
        let sigma = parameters.starting_sigma + parameters.sigma_increment * i as f64;


        println!("Simulating sigma of {}...", sigma);
        let prog2 = ProgressBar::new(parameters.s_guesses_to_consider as u64);
        prog2.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.white/gray} {pos:>7}/{len:7} {msg}").unwrap());

        let mut best_s_guess = None;
        let mut best_prob = None;

        for j in 0..parameters.s_guesses_to_consider {
        let mut overs = 0;
            let s_guess = parameters.s_guess_start + parameters.s_guess_increment * j as f64;
            for _ in 0..parameters.simulations_per_sigma {
                let mut variance_1_sample = Vec::new();
                let mut rng1 = rng();
                for _ in 0..parameters.total_time_steps {
                    variance_1_sample.push(std_normal.sample(&mut rng1));
                }

                let variance_sigma_sample = variance_1_sample.clone().iter().map(|x| x * sigma + parameters.mean).collect::<Vec<f64>>();
                variance_1_sample = variance_1_sample.iter().map(|x| x + parameters.mean).collect();

                let mut over = false;
                for step in 0..parameters.time_steps_considered {
                    let epsilon = if (sigma < 1.4 && sigma > 0.5) {1.} else if (sigma >= 0.3 && sigma <=0.5) {2.} else if (sigma > 3.) {7.} else if (sigma >=1.4 && sigma <= 2.) {2.} else {4.};
                    if (y(&variance_1_sample, step,parameters.r) -y(&variance_sigma_sample, step,parameters.r + s_guess)).abs() > epsilon{
                        over = true;
                    }
                }
                if over {
                    overs += 1;
                }

            }
            let prob = overs as f64/parameters.simulations_per_sigma as f64;
            // println!("{:?}:{:?}",s_guess, prob);
            //

            if best_prob.unwrap_or(f64::MAX) >= prob {
                best_prob = Some(prob);
                best_s_guess = Some(s_guess);
            }
            if j % 100 == 0 {
                prog2.inc(100);
            }
        }
        prog2.finish();
        sigma_s_map.push(best_s_guess.unwrap());
        println!("best_s_guess: {}, prob: {}", best_s_guess.unwrap(), best_prob.unwrap());
    }


    sigma_s_map

}
