//! 競技プログラミング用のライブラリ
//!
//! ## 区間クエリ
//!
//! * [累積和](cumulative_sum/struct.CumulativeSum.html)
//! * [Binary Indexed Tree](binary_indexed_tree/struct.BinaryIndexedTree.html)
//! * [Wavelet Matrix](wavelet_matrix/struct.WaveletMatrix.html)
//!  
//! ## グラフ
//!
//! * [グラフのための構造体・トレイト](graph/index.html)
//! * [Dijkstra法](dijkstra/index.html)
//! * [LowLink(橋・関節点)](lowlink/struct.LowLink.html)
//!
//! ## 木
//!
//! * [木の直径](tree_diameter/fn.tree_diameter.html)
//!
//! ## 文字列
//!
//! * [Rolling Hash](rolling_hash/struct.RollingHash.html)
//!
//! ## それ以外のアルゴリズム・データ構造
//!  
//! * [Union-Find](unionfind/struct.UnionFind.html)
//! * [ダブリング](doubling/struct.Doubling.html)
//!
//!

pub mod algebra;
pub mod binary_indexed_tree;
pub mod cumulative_sum;
pub mod cycle_detection;
pub mod dijkstra;
pub mod doubling;
pub mod graph;
pub mod integer_traits;
pub mod lowlink;
pub mod rolling_hash;
pub mod sparse_table;
pub mod tree_diameter;
pub mod unionfind;
pub mod wavelet_matrix;
