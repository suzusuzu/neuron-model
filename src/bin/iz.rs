extern crate gnuplot;

fn rk4<F: Fn(f64)->f64>(f: F, y: f64, dt: f64) -> f64 {
    let k1 = dt * f(y);
    let k2 = dt * f(y + k1*0.5);
    let k3 = dt * f(y + k2*0.5);
    let k4 = dt * f(y + k3);
    (k1 + 2.0*k2 + 2.0*k3 + k4) / 6.0
}

fn main(){
    const A: f64 = 0.02;
    const B: f64 = 0.2;
    const C: f64 = -65.0;
    const D: f64 = 6.0;

    let mut t = -0.0;
    let dt = 0.001;
    let mut i_ext ;
    let mut v = -70.0;
    let mut u = B*v;

    let mut x = Vec::new();
    let mut y = Vec::new();

    while t <= 100.0 {

        if t > 30.0 && t < 70.0 {
            i_ext = 14.0;
        } else {
            i_ext = 0.0;
        }

        let d_v = move |y:f64| 0.04*y*y+5.0*y+140.0-B*u+i_ext;
        v += rk4(d_v, v, dt);

        let d_u = move |y:f64| A*(v - y);
        u += rk4(d_u, u, dt);

        if v > 30.0 {
            y.push(30.0);
            v = C;
            u += D;
        }else {
            y.push(v);
        }
        x.push(t);

        t += dt;
    }

    let mut fg = gnuplot::Figure::new();
    fg.axes2d().lines(x.iter(), y.iter(), &[gnuplot::Color("blue")]);
    fg.echo_to_file("iz.plt");
}