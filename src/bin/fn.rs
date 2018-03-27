extern crate gnuplot;

fn rk4<F: Fn(f64)->f64>(f: F, y: f64, dt: f64) -> f64 {
    let k1 = dt * f(y);
    let k2 = dt * f(y + k1*0.5);
    let k3 = dt * f(y + k2*0.5);
    let k4 = dt * f(y + k3);
    (k1 + 2.0*k2 + 2.0*k3 + k4) / 6.0
}

fn main(){
    const A: f64 = 0.7;
    const B: f64 = 0.8;
    const C: f64 = 10.0;

    let mut t = -30.0;
    let dt = 0.001;
    let mut v = 0.0;
    let mut w = 0.0;
    let mut i_ext ;

    let mut x = Vec::new();
    let mut y = Vec::new();

    while t <= 100.0 {

        if t >= 30.0 && t <= 70.0 {
            i_ext = 0.35;
        } else {
            i_ext = 0.0;
        }

        let d_w = move |y:f64| v - B * y + A;
        w += rk4(d_w, w, dt);

        let d_v = move |y:f64| C*(-y.powf(3.0)/3.0 + y - w + i_ext);
        v += rk4(d_v, v, dt);

        if t >= 0.0 {
            y.push(v);
            x.push(t);
        }

        t += dt;
    }

    let mut fg = gnuplot::Figure::new();
    fg.axes2d().lines(x.iter(), y.iter(), &[gnuplot::Color("blue")]);
    fg.echo_to_file("fn.plt");
}
