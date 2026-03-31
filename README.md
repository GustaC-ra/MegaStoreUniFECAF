# MegaStoreUniFECAF
Repositório destinado para a conclusão do Estudo de caso da matéria de "Data Structure Strategy and Implementation"


# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 1. Descrição do projeto

Este projeto implementa, em Rust, um **sistema híbrido de busca e recomendação de produtos** para a MegaStore.  
A solução foi desenhada para responder ao estudo de caso proposto na disciplina, atacando diretamente os problemas de lentidão, baixa precisão e dificuldade de escalar a busca em um catálogo com milhões de itens.

A proposta combina:

- **índice invertido com `HashMap`** para busca textual rápida por nome, marca, categoria e tags;
- **índices auxiliares** para filtros por marca e categoria;
- **grafo de relacionamento entre produtos** para recomendações e reforço de relevância;
- **arquitetura modular em Rust**, fácil de testar, manter e evoluir.

> Observação importante: o ambiente de geração deste pacote não possui `rustc`/`cargo` instalados.  
> Por isso, o código foi estruturado e comentado seguindo as boas práticas de Rust, mas a compilação e a coleta de métricas devem ser executadas localmente antes da entrega final no GitHub.

---

## 2. Objetivos do sistema

1. **Indexar o catálogo com eficiência**.
2. **Buscar produtos com rapidez e precisão**.
3. **Aplicar filtros sem custo excessivo**.
4. **Suportar crescimento do catálogo e do tráfego**.
5. **Gerar recomendações com base em relações entre produtos**.
6. **Manter o projeto didático, testável e extensível**.

---

## 3. Tecnologias utilizadas

- **Rust 2021**
- **`std::collections::HashMap` / `HashSet`** para os índices principais
- **`serde`** para serialização e desserialização
- **`serde_json`** para leitura dos arquivos do catálogo e do grafo
- **`clap`** para a interface de linha de comando
- **`criterion`** para benchmarking de desempenho
- **JSON** como formato simples de carga de dados para a demonstração

---

## 4. Estrutura do repositório

```text
megastore-search/
├── benches/
│   └── search_bench.rs
├── data/
│   ├── products.json
│   └── relations.json
├── docs/
│   └── relatorio_tecnico.md
├── src/
│   ├── engine.rs
│   ├── index.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── models.rs
│   ├── recommendation.rs
│   ├── search.rs
│   └── tokenizer.rs
├── tests/
│   └── integration.rs
├── .gitignore
├── Cargo.toml
├── LICENSE
└── README.md
```

---

## 5. Como executar o sistema de busca

### 5.1. Pré-requisitos

- Rust toolchain instalado
- Cargo disponível no terminal

### 5.2. Clonar o projeto

```bash
git clone https://github.com/seu-usuario/megastore-search.git
cd megastore-search
```

### 5.3. Executar uma busca

```bash
cargo run -- \
  --products data/products.json \
  --relations data/relations.json \
  search \
  --query "fone bluetooth" \
  --category "Audio" \
  --limit 5
```

### 5.4. Executar busca com produto-semente para boost via grafo

```bash
cargo run -- \
  search \
  --query "bluetooth" \
  --seed-product P1001 \
  --limit 5
```

### 5.5. Executar recomendação por produto

```bash
cargo run -- \
  recommend \
  --product-id P1004 \
  --limit 3
```

---

## 6. Como executar os testes

### 6.1. Testes unitários e de integração

```bash
cargo test
```

### 6.2. Benchmarks com Criterion

```bash
cargo bench
```

> O benchmark incluído cria um conjunto sintético com 5.000 produtos para medir o comportamento inicial do motor de busca.

---

## 7. Exemplos de uso

### Exemplo A - Busca textual simples

Consulta:

```bash
cargo run -- search --query "notebook gamer"
```

Saída esperada:

- Notebook Gamer Titan 15 aparece entre os primeiros resultados
- Score elevado por casamento no nome e nas tags

### Exemplo B - Busca com filtros

Consulta:

```bash
cargo run -- search --query "wifi" --category "Casa Inteligente" --max-price 200
```

Saída esperada:

- Câmera de Segurança Wi-Fi 360
- Lâmpada Inteligente Color

### Exemplo C - Recomendação por grafo

Consulta:

```bash
cargo run -- recommend --product-id P1004 --limit 2
```

Saída esperada:

- Mouse Sem Fio Ergo Fit
- Teclado Mecânico RGB Strike

---

## 8. Arquitetura do sistema

A arquitetura foi separada em camadas simples e coesas:

### `models.rs`
Define as estruturas centrais:
- `Product`
- `ProductEdge`
- `SearchQuery`
- `SearchResult`
- `Recommendation`

### `tokenizer.rs`
Responsável por:
- normalização de texto;
- remoção simples de acentos;
- quebra em tokens.

### `index.rs`
Constrói e mantém:
- índice invertido por token;
- índice por marca;
- índice por categoria;
- grafo de relacionamento entre produtos.

### `search.rs`
Implementa:
- recuperação de candidatos;
- aplicação de filtros;
- cálculo de score;
- ordenação final dos resultados.

### `recommendation.rs`
Gera:
- recomendações diretas a partir do grafo;
- ordenação por peso de relacionamento.

### `engine.rs`
Expõe a fachada do sistema:
- carregamento dos arquivos JSON;
- construção dos índices;
- busca;
- recomendação.

### `main.rs`
Fornece uma CLI de demonstração.

---

## 9. Algoritmos e estruturas de dados utilizados

### 9.1. Índice invertido com `HashMap`

Estrutura principal:

```rust
HashMap<String, HashSet<String>>
```

Uso:
- chave = token normalizado
- valor = conjunto de IDs de produtos que contêm o token

Benefícios:
- acesso rápido aos candidatos;
- boa aderência ao requisito de escalabilidade;
- fácil expansão para novos campos indexáveis.

### 9.2. Índices auxiliares

- `brand_index: HashMap<String, HashSet<String>>`
- `category_index: HashMap<String, HashSet<String>>`

Benefícios:
- filtros mais baratos;
- menos iteração sobre o catálogo inteiro.

### 9.3. Grafo de relacionamento

Estrutura escolhida:

```rust
HashMap<String, Vec<(String, f32, String)>>
```

Interpretação:
- vértice = produto
- aresta = relação entre produtos
- peso = força do vínculo

Aplicações:
- recomendação de produtos complementares;
- boost de relevância com produto-semente;
- expansão futura para navegação personalizada.

### 9.4. Estratégia de ranking

O score atual considera:
- ocorrência do token no nome;
- ocorrência na marca;
- ocorrência na categoria;
- ocorrência nas tags;
- boost por filtros coincidentes;
- boost opcional vindo do grafo.

---

## 10. Considerações sobre desempenho e escalabilidade

### 10.1. Pontos positivos da abordagem

- Busca textual baseada em índice, evitando varredura completa do catálogo.
- Filtros por marca e categoria usando estruturas hash.
- Separação entre indexação e consulta.
- Grafo leve para recomendações contextuais.

### 10.2. Escalabilidade esperada

A arquitetura foi pensada para evoluir em etapas:

1. **Catálogo pequeno/médio**  
   Pode operar integralmente em memória.

2. **Catálogo grande**  
   Pode usar persistência dos índices em disco.

3. **Escala corporativa**  
   Pode ser distribuída por partição de catálogo, categoria ou tenant.

### 10.3. Benchmarks

Foi incluído um benchmark com `criterion` em `benches/search_bench.rs`.

**Status atual:**  
Os números finais ainda precisam ser coletados localmente com `cargo bench`, pois o ambiente desta entrega não possui compilador Rust disponível.

### 10.4. Métricas recomendadas

- latência p50, p95 e p99;
- throughput por segundo;
- uso de memória do índice;
- taxa de cliques em resultados;
- conversão após busca;
- taxa de refinamento da consulta;
- abandono após pesquisa sem resultado.

---

## 11. Integração com a plataforma de e-commerce

A solução proposta pode ser integrada em quatro etapas:

1. **Pipeline de ingestão**  
   Recebe catálogo, preços, estoque e atributos.

2. **Processo de indexação**  
   Atualiza os índices e o grafo.

3. **Serviço de consulta**  
   Expõe endpoints de busca e recomendação.

4. **Camada de observabilidade**  
   Monitora latência, erros, CTR e conversão.

---

## 12. Segurança e confiabilidade

- Rust ajuda a reduzir classes de erros de memória e concorrência.
- A separação entre leitura de dados e execução de consulta simplifica auditoria.
- O sistema pode ser combinado com logs, métricas e controle de acesso por serviço.

---

## 13. Exemplo concreto já existente no mercado

Como referência de mercado, a MegaStore poderia seguir uma linha semelhante à de mecanismos como:

- **Elasticsearch/OpenSearch**, que usam índice invertido para busca textual;
- **Algolia**, que adiciona relevância, autocomplete e personalização.

Este projeto, porém, foi implementado de forma **didática e enxuta em Rust**, com foco acadêmico e arquitetura explicável.

---

## 14. Contribuições

1. Faça um fork do projeto.
2. Crie uma branch para a sua melhoria.
3. Adicione testes para o comportamento alterado.
4. Abra um pull request descrevendo:
   - o problema;
   - a solução;
   - o impacto esperado.

---

## 15. Licença

Este projeto utiliza a licença **MIT**.  
Consulte o arquivo `LICENSE`.

---

## 16. Checklist de entrega

- [ ] Subir este conteúdo para um repositório público no GitHub
- [ ] Confirmar compilação com `cargo build`
- [ ] Executar `cargo test`
- [ ] Executar `cargo bench`
- [ ] Atualizar o README com o link real do repositório
- [ ] Exportar ou anexar o PDF final da documentação
