use crate::index::SearchIndex;
use crate::models::Recommendation;

/// Recupera recomendações de um produto com base no grafo de relacionamento.
pub fn recommend(index: &SearchIndex, product_id: &str, limit: usize) -> Vec<Recommendation> {
    let mut results = Vec::new();

    if let Some(neighbors) = index.graph.get(product_id) {
        for (target_id, weight, relation) in neighbors {
            if let Some(product) = index.products.get(target_id).cloned() {
                results.push(Recommendation {
                    product,
                    score: *weight,
                    relation: relation.clone(),
                });
            }
        }
    }

    results.sort_by(|left, right| {
        right
            .score
            .partial_cmp(&left.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    results.truncate(limit);
    results
}
