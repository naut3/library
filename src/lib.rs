//! 競技プログラミング用のライブラリ
//!
//! ## 区間クエリ
//!
//! * [累積和](cumulative_sum/struct.CumulativeSum.html)
//! * [Binary Indexed Tree](binary_indexed_tree/struct.BinaryIndexedTree.html)
//! * [動的Binary Indexed Tree](dynamic_binary_indexed_tree/struct.DynamicBinaryIndexedTree.html)
//! * [Segment Tree](segtree/struct.SegmentTree.html)
//! * [Wavelet Matrix](wavelet_matrix/struct.WaveletMatrix.html)
//! * [Sparse Table](sparse_table/struct.SparseTable.html)
//!  
//! ## グラフ
//!
//! * [グラフのための構造体・トレイト](graph/index.html)
//! * [Dijkstra法](dijkstra/index.html)
//! * [LowLink(橋・関節点)](lowlink/struct.LowLink.html)
//! * [強連結成分分解](scc/fn.strongly_connected_components.html)
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
//! * [代数的構造の構造体・トレイト](algebra/index.html)
//! * [ModInt](modint/struct.ModInt.html)
//! * [座標圧縮](coordinate_compression/struct.CoordinateCompress.html)
//! * [Union-Find](unionfind/struct.UnionFind.html)
//! * [ダブリング](doubling/struct.Doubling.html)
//! * [Binary Trie](binary_trie/struct.MultiBinaryTrie.html)
//! * [Fast Set](fastset/struct.FastSet.html)
//!

pub mod algebra;
pub mod binary_indexed_tree;
pub mod binary_trie;
pub mod coordinate_compression;
pub mod cumulative_sum;
pub mod cycle_detection;
pub mod dijkstra;
pub mod doubling;
pub mod dynamic_binary_indexed_tree;
pub mod fastset;
pub mod graph;
pub mod integer_traits;
pub mod lowlink;
pub mod modint;
pub mod rolling_hash;
pub mod scc;
pub mod segtree;
pub mod sparse_table;
pub mod tree_diameter;
pub mod tsp;
pub mod unionfind;
pub mod wavelet_matrix;
