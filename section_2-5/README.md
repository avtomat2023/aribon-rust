# 2-5節　グラフ

## 実行例

```
$ cargo run --bin bipartite_1 < samples/bipartite_1.in | diff samples/bipartite_1.out
```

## ソースコード

- src/bin/bipartite_*.rs: 深さ優先探索による二部グラフ判定
- src/bin/bellman-ford.rs: ベルマンフォード法による単一始点最短路

