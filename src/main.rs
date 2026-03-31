use clap::{Parser, Subcommand};
use megastore_search::{SearchEngine, SearchQuery};

/// CLI simples para demonstrar o mecanismo de busca da MegaStore.
#[derive(Debug, Parser)]
#[command(
    author = "Dexter",
    version,
    about = "Sistema de busca otimizado com HashMap e recomendações por grafo"
)]
struct Cli {
    /// Caminho do catálogo em JSON.
    #[arg(long, default_value = "data/products.json")]
    products: String,

    /// Caminho do grafo de relações em JSON.
    #[arg(long, default_value = "data/relations.json")]
    relations: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Executa uma busca textual com filtros.
    Search {
        #[arg(long)]
        query: String,
        #[arg(long)]
        brand: Option<String>,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        min_price: Option<f64>,
        #[arg(long)]
        max_price: Option<f64>,
        #[arg(long)]
        seed_product: Option<String>,
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    /// Retorna produtos relacionados a um item de referência.
    Recommend {
        #[arg(long)]
        product_id: String,
        #[arg(long, default_value_t = 5)]
        limit: usize,
    },
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let engine = SearchEngine::from_json_files(&cli.products, &cli.relations)?;

    match cli.command {
        Commands::Search {
            query,
            brand,
            category,
            min_price,
            max_price,
            seed_product,
            limit,
        } => {
            let search_query = SearchQuery {
                text: query,
                brand,
                category,
                min_price,
                max_price,
                seed_product_id: seed_product,
                limit,
            };

            let results = engine.search(&search_query);

            if results.is_empty() {
                println!("Nenhum produto encontrado.");
            } else {
                println!("Resultados encontrados: {}\n", results.len());
                for (index, item) in results.iter().enumerate() {
                    println!(
                        "{}. {} | {} | {} | R$ {:.2} | score {:.2}",
                        index + 1,
                        item.product.id,
                        item.product.name,
                        item.product.brand,
                        item.product.price,
                        item.score
                    );
                    for reason in &item.reasons {
                        println!("   - {}", reason);
                    }
                    println!();
                }
            }
        }
        Commands::Recommend { product_id, limit } => {
            let recommendations = engine.recommend(&product_id, limit);

            if recommendations.is_empty() {
                println!("Nenhuma recomendação encontrada para o produto {product_id}.");
            } else {
                println!("Recomendações para {product_id}:\n");
                for (index, item) in recommendations.iter().enumerate() {
                    println!(
                        "{}. {} | {} | {} | R$ {:.2} | relação '{}' | peso {:.2}",
                        index + 1,
                        item.product.id,
                        item.product.name,
                        item.product.brand,
                        item.product.price,
                        item.relation,
                        item.score
                    );
                }
            }
        }
    }

    Ok(())
}
