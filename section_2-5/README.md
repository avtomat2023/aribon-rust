# 2-5節　グラフ

## 実行例

```
$ cargo run --bin bipartitle_1 < sample/bipartite_1.in | diff bipartitle_1.out
```

## ソースコード

- src/bin/bipartitle_*.rs: 深さ優先探索による二部グラフ判定
- src/bin/bellman-ford.rs: ベルマンフォード法による単一始点最短路
