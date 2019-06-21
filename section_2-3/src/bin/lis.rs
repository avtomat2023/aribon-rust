// 長さnの数列 a_1, a_2, ..., a_n が与えられる。
// この数列の増加部分列のうち、最長のものの長さを求めよ。
//
// 入力
// n
// a_1, a_2, ..., a_n
//
// 出力
// 最長増加部分列の長さ

use atcoder_snippets::*;
use atcoder_snippets::read::*;
use atcoder_snippets::bsearch::*;
use atcoder_snippets::cmp::*;

fn main() {
    readls!(n = usize, aa = Vec<u64>);
    let mut dp = vec![MaybeInf::inf(); n];
    for a in aa {
        let i = (0..dp.len()).bsearch_right_min(|&i| {
            MaybeInf::new(a) <= dp[i]
        }).unwrap().unwrap();
        dp[i] = MaybeInf::new(a);
    }
    let ans = 1 + (0..dp.len()).bsearch_left_max(|&i| dp[i].is_fin()).unwrap().unwrap();
    println!("{}", ans);
}
