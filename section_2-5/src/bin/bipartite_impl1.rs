// 頂点数V, 辺数Eの連結な無向グラフが与えられる。
// グラフが二部グラフであるか判定せよ。
//
// 入力
// V E
// u_1 v_1
// u_2 v_2
// ...
// u_E v_E
//
// 出力
// 二部グラフであれば"Yes", そうでなければ"No"

#[macro_use] extern crate atcoder_snippets;
use atcoder_snippets::{read::*, utils::yn};

fn dfs(v: usize, colors: &mut [Option<bool>], adj: &[Vec<usize>]) -> bool {
    let c = colors[v].unwrap();
    for &next in adj[v].iter() {
        match colors[next] {
            None => {
                colors[next] = Some(!c);
                if !dfs(next, colors, adj) {
                    return false;
                }
            },
            Some(next_c) => if c == next_c {
                return false;
            }
        }
    }
    true
}

fn main() {
    read!(v_count = usize, _ = ());
    let edges = readx::<(usize, usize)>();
    let mut adj = vec![Vec::new(); v_count];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut colors = vec![None; v_count];
    colors[0] = Some(true);
    yn(dfs(0, &mut colors, &adj));
}
