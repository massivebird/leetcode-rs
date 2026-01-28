mod tests;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn my_sqrt(x: i32) -> i32 {
        let x: i64 = x.into();
        if x < 2 {
            return x as i32;
        }

        let mut l: i64 = 0;
        let mut r: i64 = x / 2;

        loop {
            let mid = l.midpoint(r);

            let test = (mid + 1) * (mid + 1);

            if (mid + 1).pow(2) > x && mid.pow(2) < x {
                return mid as i32;
            }

            if test == x {
                return (mid + 1) as i32;
            } else if test > x {
                r = mid;
            } else if l == mid {
                l = r;
            } else {
                l = mid;
            }
        }
    }
}
