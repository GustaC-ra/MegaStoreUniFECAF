use std::collections::{HashMap, HashSet};

use crate::models::{Product, ProductEdge};
use crate::tokenizer::tokenize;

/// Estrutura de índices do mecanismo de busca.
///
/// O índice invertido mapeia token -> conjunto de IDs de produto.
/// Já os índices auxiliares reduzem o custo de filtros por marca/categoria.
#[derive(Debug, Default)]
pub struct SearchIndex {
    pub products: HashMap<String, Product>,
    pub token_index: HashMap<String, HashSet<String>>,
    pub brand_index: HashMap<String, HashSet<String>>,
    pub category_index: HashMap<String, HashSet<String>>,
    pub graph: HashMap<String, Vec<(String, f32, String)>>,
}

impl SearchIndex {
    /// Cria uma estrutura vazia.
    pub fn new() -> Self {
        Self::default()
    }

    /// Indexa um conjunto de produtos.
    pub fn build(products: Vec<Product>, edges: Vec<ProductEdge>) -> Self {
        let mut index = Self::new();

        for product in products {
            index.insert_product(product);
        }

        for edge in edges {
            index.insert_edge(edge);
        }

        index
    }

    /// Insere um produto em todos os índices necessários.
    pub fn insert_product(&mut self, product: Product) {
        let id = product.id.clone();

        let searchable_text = format!(
            "{} {} {} {}",
            product.name,
            product.brand,
            product.category,
            product.tags.join(" ")
        );

        for token in tokenize(&searchable_text) {
            self.token_index
                .entry(token)
                .or_default()
                .insert(id.clone());
        }

        self.brand_index
            .entry(product.brand.to_lowercase())
            .or_default()
            .insert(id.clone());

        self.category_index
            .entry(product.category.to_lowercase())
            .or_default()
            .insert(id.clone());

        self.products.insert(id, product);
    }

    /// Adiciona uma aresta do grafo em ambas as direções.
    ///
    /// Em um catálogo de e-commerce, relações como "comprado junto" e
    /// "semelhante a" geralmente funcionam bem como conexões bidirecionais.
    pub fn insert_edge(&mut self, edge: ProductEdge) {
        self.graph
            .entry(edge.from.clone())
            .or_default()
            .push((edge.to.clone(), edge.weight, edge.relation.clone()));

        self.graph
            .entry(edge.to)
            .or_default()
            .push((edge.from, edge.weight, edge.relation));
    }
}
