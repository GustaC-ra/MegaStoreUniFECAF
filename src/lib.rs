//! Biblioteca principal do sistema de busca da MegaStore.
//!
//! A solução combina:
//! - índice invertido com `HashMap` para busca textual rápida;
//! - índices auxiliares por marca e categoria;
//! - um grafo de relacionamento entre produtos para recomendação.

pub mod engine;
pub mod index;
pub mod models;
pub mod recommendation;
pub mod search;
pub mod tokenizer;

pub use engine::SearchEngine;
pub use models::{
    Product,
    ProductEdge,
    Recommendation,
    SearchQuery,
    SearchResult,
};
