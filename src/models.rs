use serde::{Deserialize, Serialize};

/// Estrutura principal de produto do catálogo.
///
/// Os campos foram escolhidos para representar um recorte realista de um
/// e-commerce: identificação, atributos usados na busca e metadados de preço.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub price: f64,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
}

/// Aresta do grafo de relacionamento entre produtos.
///
/// `weight` representa a força do vínculo:
/// - co-compra
/// - co-visualização
/// - similaridade manual
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductEdge {
    pub from: String,
    pub to: String,
    pub weight: f32,
    pub relation: String,
}

/// Consulta recebida pelo mecanismo de busca.
///
/// Os filtros opcionais permitem refinar o resultado sem reindexar o catálogo.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchQuery {
    pub text: String,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub seed_product_id: Option<String>,
    pub limit: usize,
}

impl SearchQuery {
    /// Cria uma consulta textual simples com limite padrão.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            brand: None,
            category: None,
            min_price: None,
            max_price: None,
            seed_product_id: None,
            limit: 10,
        }
    }
}

/// Resultado individual de busca.
///
/// `reasons` ajuda na explicabilidade, algo importante em sistemas de busca
/// reais para depuração e ajustes de relevância.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub product: Product,
    pub score: f64,
    pub reasons: Vec<String>,
}

/// Resultado de recomendação vindo do grafo de produtos.
#[derive(Debug, Clone, PartialEq)]
pub struct Recommendation {
    pub product: Product,
    pub score: f32,
    pub relation: String,
}
