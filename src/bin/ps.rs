extern crate gnuplot;
extern crate rand;
extern crate rand_distr;

use rand::prelude::*;
use rand_distr::{Poisson, Distribution};
use rand::distributions::Standard;
use rand::rngs::ThreadRng;


pub fn generate_spike(fr: f64, dt: f64, t: f64, rng: &mut ThreadRng) -> Vec<usize> {
    let num = (t/dt) as usize;
    let v: Vec<usize> = rng.sample_iter(Standard).take(num).map(|x: f64| if x < fr*dt { 1 } else { 0 } ).collect();
    v
}

fn main() {
    const N: usize = 100000;
    const LAMBDA: f64 = 60.0;

    let mut rng = rand::thread_rng();
    let mut fg = gnuplot::Figure::new();

    {
        // poisson spike
        let mut spike_cnts = Vec::with_capacity(N);
        for _ in 0..N {
            let spikes = generate_spike(LAMBDA, 0.0001, 1.0, &mut rng);
            let spike_cnt = spikes.iter().sum::<usize>();
            spike_cnts.push(spike_cnt);
        }

        let max_cnt = spike_cnts.iter().max().unwrap();
        let xs = (0..*max_cnt).collect::<Vec<usize>>();

        let mut spike_hist = vec![0;*max_cnt+1];
        for spike_cnt in &spike_cnts {
            spike_hist[*spike_cnt] += 1;
        }

        let ax = fg.axes2d();
        ax.lines(xs.iter(), spike_hist.iter(), &[gnuplot::Caption("poisson spike"), gnuplot::Color("blue")]);


        // poisson distributions
        let mut spike_cnts: Vec<f64> = Vec::with_capacity(N);
        for _ in 0..N {
            let poi: Poisson<f64> = Poisson::new(LAMBDA).unwrap();
            let v  = poi.sample(&mut rng);
            spike_cnts.push(v);
        }
        let max_cnt= spike_cnts.iter().fold(0.0/0.0, |m, v: &f64| v.max(m));

        let mut spike_hist = vec![0;(max_cnt as usize + 1) as usize];
        for spike_cnt in &spike_cnts {
            spike_hist[*spike_cnt as usize] += 1;
        }

        ax.lines(xs.iter(), spike_hist.iter(), &[gnuplot::Caption("poisson distributions"), gnuplot::Color("red")]);

    }
    fg.echo_to_file("ps.plt");
}