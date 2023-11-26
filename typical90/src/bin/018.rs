use proconio::input;
use std::f64::consts::PI;

// 三角関数

fn radian_to_degree(r: f64) -> f64 {
    r * 180.0 / PI
}

fn degree_to_radion(d: f64) -> f64 {
    d * PI / 180.0
}

fn main() {
    input! {
        t: i32,
        l: i32,
        x: i32,
        y: i32,
        q: usize,
        e: [i32; q],
    };

    let radius = l as f64 / 2.0;

    for ei in e {
        let turn_rate = 360.0 * ei as f64 / t as f64;
        let turn_radian = degree_to_radion(turn_rate);

        let wheel_y = radius * turn_radian.sin();
        let wheel_z = radius - (radius * turn_radian.sin());

        let length = (x.pow(2) as f64 + (y as f64 - wheel_y) * (y as f64 - wheel_y)).sqrt();

        let target_radian = wheel_z / length;

        let ans = radian_to_degree(target_radian.asin());

        println!("{}", ans)
    }
}
