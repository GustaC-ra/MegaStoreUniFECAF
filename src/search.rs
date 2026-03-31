use std::collections::{HashMap, HashSet};

use crate::index::SearchIndex;
use crate::models::{SearchQuery, SearchResult};
use crate::tokenizer::{normalize, tokenize};

/// Executa a busca textual e aplica ranking.
///
/// A estratégia adotada é intencionalmente simples para fins didáticos:
/// - tokens da consulta recuperam candidatos pelo índice invertido;
/// - score recebe pesos por casamento em nome, marca, categoria e tags;
/// - filtros são aplicados no final;
/// - opcionalmente, o grafo pode adicionar um boost contextual.
pub fn execute(index: &SearchIndex, query: &SearchQuery) -> Vec<SearchResult> {
    let normalized_brand = query.brand.as_ref().map(|value| normalize(value));
    let normalized_category = query.category.as_ref().map(|value| normalize(value));

    let mut candidate_ids: HashSet<String> = HashSet::new();
    let tokens = tokenize(&query.text);

    if tokens.is_empty() {
        candidate_ids.extend(index.products.keys().cloned());
    } else {
        for token in &tokens {
            if let Some(ids) = index.token_index.get(token) {
                candidate_ids.extend(ids.iter().cloned());
            }
        }
    }

    if let Some(brand) = &normalized_brand {
        if let Some(ids) = index.brand_index.get(brand) {
            candidate_ids = candidate_ids
                .intersection(ids)
                .cloned()
                .collect::<HashSet<_>>();
        } else {
            candidate_ids.clear();
        }
    }

    if let Some(category) = &normalized_category {
        if let Some(ids) = index.category_index.get(category) {
            candidate_ids = candidate_ids
                .intersection(ids)
                .cloned()
                .collect::<HashSet<_>>();
        } else {
            candidate_ids.clear();
        }
    }

    let graph_boosts = collect_graph_boosts(index, query.seed_product_id.as_deref());

    let mut results = Vec::new();

    for id in candidate_ids {
        let Some(product) = index.products.get(&id).cloned() else {
            continue;
        };

        if let Some(min_price) = query.min_price {
            if product.price < min_price {
                continue;
            }
        }

        if let Some(max_price) = query.max_price {
            if product.price > max_price {
                continue;
            }
        }

        let mut score = 0.0;
        let mut reasons = Vec::new();

        let name_normalized = normalize(&product.name);
        let brand_normalized = normalize(&product.brand);
        let category_normalized = normalize(&product.category);
        let tags_normalized = normalize(&product.tags.join(" "));

        for token in &tokens {
            if name_normalized.contains(token) {
                score += 5.0;
                reasons.push(format!("Token '{}' encontrado no nome", token));
            }
            if brand_normalized.contains(token) {
                score += 3.0;
                reasons.push(format!("Token '{}' encontrado na marca", token));
            }
            if category_normalized.contains(token) {
                score += 2.5;
                reasons.push(format!("Token '{}' encontrado na categoria", token));
            }
            if tags_normalized.contains(token) {
                score += 2.0;
                reasons.push(format!("Token '{}' encontrado nas tags", token));
            }
        }

        if let Some(brand) = &normalized_brand {
            if brand == &brand_normalized {
                score += 3.0;
                reasons.push("Filtro de marca aplicado".to_string());
            }
        }

        if let Some(category) = &normalized_category {
            if category == &category_normalized {
                score += 2.0;
                reasons.push("Filtro de categoria aplicado".to_string());
            }
        }

        if let Some(boost) = graph_boosts.get(&product.id) {
            score += *boost as f64;
            reasons.push(format!("Boost de grafo aplicado ({:.2})", boost));
        }

        if tokens.is_empty() {
            score += 1.0;
            reasons.push("Consulta sem texto: ordenação por filtros e score base".to_string());
        }

        if score > 0.0 || tokens.is_empty() {
            results.push(SearchResult {
                product,
                score,
                reasons,
            });
        }
    }

    results.sort_by(|left, right| {
        right
            .score
            .partial_cmp(&left.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                left.product
                    .price
                    .partial_cmp(&right.product.price)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });

    results.truncate(query.limit);
    results
}

/// Coleta boosts de relevância vindos do grafo de relacionamento.
fn collect_graph_boosts(index: &SearchIndex, seed_product_id: Option<&str>) -> HashMap<String, f32> {
    let mut boosts = HashMap::new();

    let Some(seed) = seed_product_id else {
        return boosts;
    };

    if let Some(neighbors) = index.graph.get(seed) {
        for (target_id, weight, _relation) in neighbors {
            boosts.insert(target_id.clone(), *weight);
        }
    }

    boosts
}
