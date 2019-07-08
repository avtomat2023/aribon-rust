// 根つき木が与えられる。
// 頂点uと頂点vの共通の祖先(u自体、または、v自体でありうる)のうち、
// 根からの距離が最も大きい頂点を答えよ。
//
// 各頂点は0 <= v <= N-1なる整数vで表される。根は頂点0である。
// 辺はu_i v_iの形で与えられ、u_iが親、v_iが子である。
//
// 入力
// N
// u_1 v_1
// u_2 v_2
// ...
// u_(N-1) v_(N-1)
// u v
//
// 出力
// 最低共通祖先

#[macro_use] extern crate atcoder_snippets;
use atcoder_snippets::read::*;
use atcoder_snippets::num::*;

fn dfs_depth(v: usize, children: &[Vec<usize>], depth: usize, res: &mut [usize]) {
    res[v] = depth;
    for &next in &children[v] {
        dfs_depth(next, children, depth + 1, res);
    }
}

fn main() {
    read!(n = usize);
    let mut children = vec![Vec::new(); n];
    let mut parent = vec![None; n];
    readn_loop!(n-1, |u = usize, v = usize| {
        children[u].push(v);
        parent[v] = Some(u);
    });
    read!(u = usize, v = usize);

    let mut depth = vec![0; n];
    dfs_depth(0, &children, 0, &mut depth);

    let len = depth.iter().cloned()
        .max().unwrap()
        .ceil_log2().unwrap_or(0) + 1;
    let mut doubling_parent = vec![vec![None; n]; len];
    doubling_parent[0] = parent;
    for d in 1..len {
        for v in 0..n {
            doubling_parent[d][v] = doubling_parent[d-1][v]
                .and_then(|p| doubling_parent[d-1][p]);
        }
    }

    let (mut u, mut v) = if depth[u] <= depth[v] { (u, v) } else { (v, u) };
    for k in 0..len {
        if (depth[v] - depth[u]) >> k & 1 != 0 {
            v = doubling_parent[k][v].unwrap();
        }
    }

    for k in (0..len).rev() {
        if doubling_parent[k][u] != doubling_parent[k][v] {
            u = doubling_parent[k][u].unwrap();
            v = doubling_parent[k][v].unwrap();
        }
    }

    println!("{:?}", u);
}
