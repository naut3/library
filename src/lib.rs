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
//! * [LowLink(橋・関節点)](lowlink/struct.LowLink.html)
//! 
//! ## 木
//! 
//! * [木の直径](tree_diameter/fn.tree_diameter.html)
//! 
//! ## それ以外のアルゴリズム・データ構造
//!  
//! * [Union-Find](unionfind/struct.UnionFind.html)
//! * [ダブリング](doubling/struct.Doubling.html)
//! 
//! 

pub mod binary_indexed_tree;
pub mod cumulative_sum;
pub mod cycle_detection;
pub mod doubling;
pub mod graph;
pub mod lowlink;
pub mod tree_diameter;
pub mod unionfind;
pub mod wavelet_matrix;
