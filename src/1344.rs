struct Solution {}

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let min_ang = f64::from(minutes) * 6.0;
        let hour_ang = f64::from(hour % 12) * 30.0 + min_ang / 12.0;

        let ang_diff = (min_ang - hour_ang + 360.0) % 360.0;

        if ang_diff > 180.0 { 360.0 - ang_diff } else { ang_diff }
    }
}

fn main() {}