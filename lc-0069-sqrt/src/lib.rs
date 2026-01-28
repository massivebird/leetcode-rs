mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn my_sqrt(x: i32) -> i32 {
        let mut last: f32 = 1.0; // Initial guess
        let mut next: f32 = 0.0;

        for _ in 0..10 {
            next = 0.5 * (last + (x as f32) / last);
            last = next;
        }

        return next.floor() as i32;
    }
}
