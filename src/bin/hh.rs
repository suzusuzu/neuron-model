extern crate gnuplot;

fn alpha_m(v: f64) -> f64{
    (0.1*(25.0-v)) / (((25.0-v)/10.0).exp()-1.0)
}

fn alpha_h(v: f64) -> f64{
    0.07*((-v/20.0).exp())
}

fn alpha_n(v: f64) -> f64{
    (0.01*(10.0-v)) / (((10.0-v)/10.0).exp()-1.0)
}

fn beta_m(v: f64) -> f64{
    4.0*((-v/18.0).exp())
}

fn beta_h(v: f64) -> f64{
    1.0/(((30.0-v)/10.0).exp()+1.0)
}

fn beta_n(v: f64) -> f64{
    0.125*((-v/80.0).exp())
}

fn rk4<F: Fn(f64)->f64>(f: F, y: f64, dt: f64) -> f64 {
    let k1 = dt * f(y);
    let k2 = dt * f(y + k1*0.5);
    let k3 = dt * f(y + k2*0.5);
    let k4 = dt * f(y + k3);
    (k1 + 2.0*k2 + 2.0*k3 + k4) / 6.0
}

fn main(){
    const E_NA: f64 = 115.0;
    const E_K:  f64 = -12.0;
    const E_L:  f64 = 10.613;
    const G_NA: f64 = 120.0;
    const G_K:  f64 = 36.0;
    const G_L:  f64 = 0.3;
    const C_M:  f64 = 1.0;

    let mut t = -100.0;
    let dt = 0.001;
    let mut i_ext ;
    let mut v = -10.0;
    let mut x_n = 0.0;
    let mut x_m = 0.0;
    let mut x_h = 0.0;

    let mut x = Vec::new();
    let mut y = Vec::new();

    while t <= 100.0 {

        if t > 30.0 && t < 70.0 {
            i_ext = 10.0;
        } else {
            i_ext = 0.0;
        }

        let a_n = alpha_n(v);
        let a_m = alpha_m(v);
        let a_h = alpha_h(v);

        let b_n = beta_n(v);
        let b_m = beta_m(v);
        let b_h = beta_h(v);

        let d_n = |y: f64| a_n*(1.0-y) - b_n*y;
        let d_m = |y: f64| a_m*(1.0-y) - b_m*y;
        let d_h = |y: f64| a_h*(1.0-y) - b_h*y;

        x_n += rk4(d_n, x_n, dt);
        x_m += rk4(d_m, x_m, dt);
        x_h += rk4(d_h, x_h, dt);

        let i_na = G_NA *(x_m.powf(3.0))*x_h*(v- E_NA);
        let i_k = G_K *(x_n.powf(4.0))*(v- E_K);
        let i_l = G_L *(v- E_L);
        let i_sum = i_na + i_k + i_l;

        let d_v = |y:f64| (i_ext - i_sum) / C_M;
        v += rk4(d_v, v, dt);

        if t >= 0.0{
            x.push(t);
            y.push(v);
        }
        t += dt;
    }

    let mut fg = gnuplot::Figure::new();
    fg.axes2d().lines(x.iter(), y.iter(), &[gnuplot::Color("blue")]);
    fg.echo_to_file("hh.plt");
}