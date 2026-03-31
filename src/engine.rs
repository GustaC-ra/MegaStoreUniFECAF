use std::fs;
use std::path::Path;

use crate::index::SearchIndex;
use crate::models::{Product, ProductEdge, Recommendation, SearchQuery, SearchResult};

/// Fachada principal do sistema.
///
/// Ela concentra:
/// - carregamento de dados;
/// - construção do índice;
/// - busca;
/// - recomendação.
#[derive(Debug)]
pub struct SearchEngine {
    index: SearchIndex,
}

impl SearchEngine {
    /// Cria um mecanismo a partir de vetores já carregados em memória.
    pub fn new(products: Vec<Product>, edges: Vec<ProductEdge>) -> Self {
        Self {
            index: SearchIndex::build(products, edges),
        }
    }

    /// Carrega um catálogo JSON e um arquivo JSON de relações.
    pub fn from_json_files(
        products_path: impl AsRef<Path>,
        edges_path: impl AsRef<Path>,
    ) -> Result<Self, String> {
        let products_data = fs::read_to_string(products_path.as_ref())
            .map_err(|error| format!("Falha ao ler catálogo: {error}"))?;
        let edges_data = fs::read_to_string(edges_path.as_ref())
            .map_err(|error| format!("Falha ao ler relações: {error}"))?;

        let products: Vec<Product> = serde_json::from_str(&products_data)
            .map_err(|error| format!("Falha ao desserializar catálogo: {error}"))?;
        let edges: Vec<ProductEdge> = serde_json::from_str(&edges_data)
            .map_err(|error| format!("Falha ao desserializar relações: {error}"))?;

        Ok(Self::new(products, edges))
    }

    /// Executa busca no índice.
    pub fn search(&self, query: &SearchQuery) -> Vec<SearchResult> {
        crate::search::execute(&self.index, query)
    }

    /// Recupera recomendações por relacionamento de grafo.
    pub fn recommend(&self, product_id: &str, limit: usize) -> Vec<Recommendation> {
        crate::recommendation::recommend(&self.index, product_id, limit)
    }

    /// Exposição controlada do índice para testes e futuras extensões.
    pub fn product_count(&self) -> usize {
        self.index.products.len()
    }
}
