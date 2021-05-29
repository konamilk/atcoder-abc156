use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        x: [i32; n]
    };

    let mut ans = std::i32::MAX;

    for p in 1..=100{
        let mut cost = 0;

        for i in 0..n{
            cost += (x[i]-p) * (x[i]-p)
        }

        ans = min(ans, cost)
    }

    println!("{}", ans);
}
