extern crate gnuplot;
extern crate rand;

use rand::{Rng, XorShiftRng, SeedableRng};
use rand::distributions::{Poisson, Distribution};

use std::mem::transmute;


pub fn generate_spike(fr: f64, dt: f64, t: f64, rng: &mut XorShiftRng) -> Vec<usize> {
    let num = (t/dt) as usize;
    let v: Vec<usize> = rng.gen_iter().take(num).map(|x: f64| if x < fr*dt { 1 } else { 0 } ).collect();
    v
}

fn main() {
    let seed = 1;
    let x: u32 = 123456789;
    let y: u32 = 362436069;
    let z: u32 = 521288629;
    let w: u32 = seed;
    let seeds: [u32; 4] = [x, y, z, w];
    const N: usize = 1000000;
    const LAMBDA: f64 = 60.0;

    let seeds = unsafe {transmute(seeds)};
    let mut rng = XorShiftRng::from_seed(seeds);
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
        let mut spike_cnts = Vec::with_capacity(N);
        for _ in 0..N {
            let poi = Poisson::new(LAMBDA);
            let v = poi.sample(&mut rng);
            spike_cnts.push(v);
        }
        let max_cnt = spike_cnts.iter().max().unwrap();

        let mut spike_hist = vec![0;(*max_cnt+1) as usize];
        for spike_cnt in &spike_cnts {
            spike_hist[*spike_cnt as usize] += 1;
        }

        ax.lines(xs.iter(), spike_hist.iter(), &[gnuplot::Caption("poisson distributions"), gnuplot::Color("red")]);

    }
    fg.echo_to_file("ps.plt");
}