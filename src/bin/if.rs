extern crate gnuplot;

fn rk4<F: Fn(f64)->f64>(f: F, y: f64, dt: f64) -> f64 {
    let k1 = dt * f(y);
    let k2 = dt * f(y + k1*0.5);
    let k3 = dt * f(y + k2*0.5);
    let k4 = dt * f(y + k3);
    (k1 + 2.0*k2 + 2.0*k3 + k4) / 6.0
}

fn main(){
    const E_L: f64 = -65.0;
    const THETA: f64 = -55.0;
    const TAU: f64 = 10.0;
    const RI: f64 = 12.0;

    let mut t = 0.0;
    let dt = 0.001;
    let mut v = E_L;

    let mut x = Vec::new();
    let mut y = Vec::new();

    while t <= 100.0 {

        if v >= THETA {
            v = E_L;
        }

        let d_v = move |y:f64| (E_L - y + RI)/TAU;
        v += rk4(d_v, v, dt);

        y.push(v);
        x.push(t);

        t += dt;
    }

    let mut fg = gnuplot::Figure::new();
    fg.axes2d().lines(x.iter(), y.iter(), &[gnuplot::Color("blue")]);
    fg.echo_to_file("if.plt");
}
