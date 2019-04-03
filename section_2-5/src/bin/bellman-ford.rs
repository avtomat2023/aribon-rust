// 頂点数N, 辺数Eの無向グラフが与えられる。
// 頂点0から頂点(N-1)までの最短路のコストを、ベルマンフォード法で計算せよ。
//
// 入力
// N E
// u_0 v_0 cost_0
// u_1 v_1 cost_1
// ...
// u_{N-1} v_{N-1} cost_{N-1}
//
// 出力
// 最短路のコスト

#[macro_use] extern crate atcoder_snippets;
use atcoder_snippets::read::*;

struct Edge {
    from: usize,
    to: usize,
    cost: u32
}

fn main() {
    read!(v_count = usize, _ = ());
    let mut es = Vec::new();
    readx_loop!(|from = usize, to = usize, cost = u32| {
        es.push(Edge { from, to, cost });
        es.push(Edge { from: to, to: from, cost});
    });

    let mut dists = vec![None; v_count];
    dists[0] = Some(0);
    loop {
        let mut updated = false;
        for e in &es {
            if let Some(d) = dists[e.from] {
                if d + e.cost < dists[e.to].unwrap_or(u32::max_value()) {
                    dists[e.to] = Some(d + e.cost);
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }

    println!("{}", dists.last().unwrap().unwrap());
}
