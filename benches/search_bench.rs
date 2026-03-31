use criterion::{criterion_group, criterion_main, Criterion};
use megastore_search::{Product, ProductEdge, SearchEngine, SearchQuery};

fn build_engine() -> SearchEngine {
    let mut products = Vec::new();

    for i in 0..5_000 {
        products.push(Product {
            id: format!("P{i}"),
            name: format!("Notebook Gamer Turbo {i}"),
            brand: if i % 2 == 0 { "MegaTech" } else { "PowerPlay" }.to_string(),
            category: "Informatica".to_string(),
            price: 3500.0 + i as f64,
            tags: vec!["notebook".into(), "gamer".into(), "ssd".into()],
            description: "Produto gerado para benchmark".into(),
        });
    }

    let edges = (0..4_999)
        .map(|i| ProductEdge {
            from: format!("P{i}"),
            to: format!("P{}", i + 1),
            weight: 1.0,
            relation: "similar".into(),
        })
        .collect();

    SearchEngine::new(products, edges)
}

fn benchmark_search(c: &mut Criterion) {
    let engine = build_engine();
    let query = SearchQuery {
        text: "notebook gamer".into(),
        brand: Some("MegaTech".into()),
        category: Some("Informatica".into()),
        min_price: Some(3600.0),
        max_price: Some(5000.0),
        seed_product_id: Some("P10".into()),
        limit: 20,
    };

    c.bench_function("search_5000_products", |b| {
        b.iter(|| {
            let _ = engine.search(&query);
        })
    });
}

criterion_group!(benches, benchmark_search);
criterion_main!(benches);
