use megastore_search::{Product, ProductEdge, SearchEngine, SearchQuery};

fn sample_engine() -> SearchEngine {
    let products = vec![
        Product {
            id: "P1".into(),
            name: "Fone Bluetooth Pro".into(),
            brand: "SoundMax".into(),
            category: "Audio".into(),
            price: 299.90,
            tags: vec!["fone".into(), "bluetooth".into(), "premium".into()],
            description: "Cancelamento de ruído".into(),
        },
        Product {
            id: "P2".into(),
            name: "Caixa de Som Portátil".into(),
            brand: "SoundMax".into(),
            category: "Audio".into(),
            price: 199.90,
            tags: vec!["som".into(), "bluetooth".into()],
            description: "Portátil e resistente".into(),
        },
        Product {
            id: "P3".into(),
            name: "Tênis Corrida Leve".into(),
            brand: "RunFast".into(),
            category: "Calcados".into(),
            price: 349.90,
            tags: vec!["tenis".into(), "corrida".into()],
            description: "Respirável".into(),
        },
    ];

    let edges = vec![
        ProductEdge {
            from: "P1".into(),
            to: "P2".into(),
            weight: 1.8,
            relation: "co-compra".into(),
        },
    ];

    SearchEngine::new(products, edges)
}

#[test]
fn search_finds_audio_product_by_name_token() {
    let engine = sample_engine();
    let query = SearchQuery::new("fone bluetooth");

    let results = engine.search(&query);

    assert!(!results.is_empty());
    assert_eq!(results[0].product.id, "P1");
}

#[test]
fn search_applies_category_filter() {
    let engine = sample_engine();
    let query = SearchQuery {
        text: "bluetooth".into(),
        brand: None,
        category: Some("Audio".into()),
        min_price: None,
        max_price: None,
        seed_product_id: None,
        limit: 10,
    };

    let results = engine.search(&query);
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|item| item.product.category == "Audio"));
}

#[test]
fn recommendation_uses_graph_neighbors() {
    let engine = sample_engine();
    let recommendations = engine.recommend("P1", 5);

    assert_eq!(recommendations.len(), 1);
    assert_eq!(recommendations[0].product.id, "P2");
}
