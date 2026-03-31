# Implementação de um Sistema de Recomendação de Produtos Utilizando Grafos
## MegaStore - Relatório técnico do estudo de caso

**Disciplina:** Data Structures Strategy and Implementation  
**Linguagem:** Rust  
**Entregável preparado por:** Dexter  
**Data:** 30/03/2026

---

## Resumo executivo

A MegaStore enfrenta um problema clássico de e-commerce em larga escala: o catálogo cresce continuamente, mas a busca atual devolve resultados lentos, imprecisos e pouco relevantes. Isso impacta diretamente experiência do cliente, taxa de conversão e reputação da marca.

Para responder ao estudo de caso, foi proposta uma solução híbrida em Rust baseada em dois pilares:

1. **Índice invertido com tabelas hash (`HashMap`)**, responsável pela recuperação rápida dos produtos candidatos.
2. **Grafo de relacionamento entre produtos**, responsável por recomendação contextual e reforço de relevância.

A implementação entregue neste pacote inclui:

- código-fonte em Rust organizado em módulos;
- testes automatizados;
- benchmark com Criterion;
- README técnico;
- dados de exemplo para demonstração.

> Nota de transparência: o ambiente usado para montar esta entrega não possui `rustc`/`cargo`; por isso, os arquivos foram preparados, comentados e organizados para uso imediato, mas a compilação e a obtenção dos números finais de benchmark devem ser executadas localmente antes da submissão no GitHub.

---

## 1. Entendimento do desafio

O enunciado descreve uma empresa com milhões de produtos, em que a busca tradicional já não atende às necessidades do negócio. O trabalho pede uma solução em Rust que seja eficiente, escalável, segura e bem documentada.

Embora o texto do estudo de caso foque em **otimização da busca**, o próprio título pede **recomendação utilizando grafos**. Por isso, a proposta adotada não trata busca e recomendação como problemas isolados; ela combina as duas frentes em um único motor:

- **Busca** para localizar o produto certo.
- **Grafo** para sugerir complementos, produtos relacionados e aumentar a relevância.

Essa união é coerente com o cenário real de e-commerce, em que encontrar e sugerir caminham juntos.

---

## 2. Principais causas da lentidão e imprecisão do sistema atual

### 2.1. Lentidão

As causas mais prováveis são:

- busca por varredura linear do catálogo;
- ausência de índice invertido;
- filtros aplicados tardiamente;
- falta de normalização do texto;
- repetição de operações custosas a cada consulta.

### 2.2. Imprecisão

A baixa precisão costuma surgir de:

- comparação textual ingênua;
- falta de tratamento de acentos e ruídos;
- ausência de pesos por campo;
- ausência de sinônimos e contexto;
- inexistência de sinais comportamentais.

---

## 3. Quem é afetado e como é afetado

### Clientes
- demoram para encontrar o que precisam;
- abandonam a navegação;
- perdem confiança na loja.

### Equipe de produto
- encontra mais dificuldade para melhorar conversão;
- passa a depender de ajustes manuais pouco sustentáveis.

### Operação comercial
- perde oportunidades de cross-sell e upsell;
- vende menos itens por sessão.

### Marca
- sofre desgaste de reputação;
- passa a ser percebida como plataforma lenta ou desorganizada.

---

## 4. Soluções possíveis para otimizar a busca

### Alternativa A - Melhorar apenas filtros no banco relacional
**Prós**
- menor ruptura inicial.

**Contras**
- resolve pouco da relevância;
- piora com crescimento do catálogo;
- não é a melhor abordagem para busca textual.

### Alternativa B - Motor de busca dedicado com índice invertido
**Prós**
- grande ganho de velocidade;
- excelente aderência a texto;
- escalável.

**Contras**
- exige pipeline de indexação;
- adiciona complexidade operacional.

### Alternativa C - Motor híbrido com índice + grafo
**Prós**
- melhora busca e recomendação;
- aumenta contexto e personalização;
- amplia oportunidades comerciais.

**Contras**
- exige mais modelagem;
- demanda governança de dados relacionais.

### Alternativa D - Busca vetorial/semântica
**Prós**
- melhor compreensão semântica;
- útil para linguagem natural.

**Contras**
- maior custo;
- mais difícil de explicar;
- desnecessária como primeira etapa para este estudo.

**Escolha deste projeto:**  
A solução escolhida foi a **Alternativa C**, implementada de forma didática em Rust: índice invertido com tabelas hash + grafo de produtos.

---

## 5. Barreiras e desafios de implementação

- garantir atualização consistente do índice;
- normalizar dados de catálogo de múltiplas origens;
- definir pesos de relevância;
- manter o grafo atualizado;
- medir qualidade de busca com métricas reais;
- preparar a arquitetura para crescimento.

---

## 6. Como as tabelas hash em Rust contribuem para a solução

As tabelas hash são o núcleo da implementação.

### Aplicações no projeto

- `HashMap<String, HashSet<String>>` para índice invertido;
- `HashMap<String, HashSet<String>>` para marca;
- `HashMap<String, HashSet<String>>` para categoria;
- `HashMap<String, Vec<(String, f32, String)>>` para o grafo de relações.

### Benefícios

- recuperação rápida de candidatos;
- boa performance em memória;
- código legível e idiomático;
- aderência ao objetivo da disciplina.

---

## 7. Requisitos de hardware e software

### Software
- Rust toolchain
- Cargo
- sistema operacional Linux, macOS ou Windows
- Git para versionamento

### Hardware
Para fins acadêmicos e de protótipo:

- CPU de 4 núcleos
- 8 GB de RAM
- SSD recomendado

Para produção:
- instâncias escaláveis;
- armazenamento rápido;
- observabilidade;
- estratégia de replicação e particionamento.

---

## 8. Integração com a plataforma de e-commerce

A solução pode ser integrada ao ecossistema da MegaStore da seguinte forma:

1. **Ingestão**  
   Catálogo, preço, estoque e atributos entram no pipeline.

2. **Indexação**  
   O serviço de indexação atualiza os mapas hash e o grafo.

3. **Consulta**  
   A API de busca recebe a consulta do front-end.

4. **Recomendação**  
   O produto consultado ou clicado pode acionar sugestões relacionadas.

5. **Observabilidade**  
   Métricas de latência e relevância retroalimentam o ajuste dos pesos.

---

## 9. Indicadores de desempenho

### Técnicos
- latência p50, p95 e p99;
- throughput;
- uso de memória;
- tempo de reconstrução do índice;
- tempo de atualização incremental.

### De negócio
- CTR da busca;
- taxa de conversão após busca;
- receita por sessão;
- abandono após zero resultado;
- taxa de clique em recomendação.

---

## 10. Implicações de longo prazo se o problema não for resolvido

- aumento do abandono do site;
- perda de competitividade;
- queda de conversão;
- aumento de custo operacional;
- dependência de intervenção manual;
- dificuldade de escalar o e-commerce.

---

## 11. Solução proposta

A solução foi estruturada como um **motor híbrido de busca e recomendação**.

### 11.1. Busca

A busca utiliza:

- tokenização;
- normalização de texto;
- índice invertido;
- filtros por marca, categoria e faixa de preço;
- score por campo.

### 11.2. Recomendação

O módulo de recomendação utiliza um grafo em memória, em que:

- cada **produto** é um vértice;
- cada **relação** é uma aresta;
- o **peso** indica a força da ligação.

Exemplos de relações:
- comprado junto;
- semelhante;
- acessório;
- coleção;
- look esportivo.

### 11.3. Relevância híbrida

Quando o usuário consulta um produto-semente, o sistema pode:
- buscar por texto;
- aplicar um **boost de grafo** para itens relacionados ao contexto atual.

Isso torna a resposta mais útil sem abandonar a explicabilidade.

---

## 12. Estrutura do repositório entregue

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
├── Cargo.toml
├── LICENSE
└── README.md
```

---

## 13. Explicação dos módulos

### `models.rs`
Modelos de domínio: produto, aresta, consulta, resultado.

### `tokenizer.rs`
Normalização e tokenização.

### `index.rs`
Construção dos índices em hash e do grafo.

### `search.rs`
Recuperação de candidatos, filtros e ranking.

### `recommendation.rs`
Recomendações por vizinhança do grafo.

### `engine.rs`
Fachada do sistema.

### `main.rs`
CLI para demonstração e testes manuais.

---

## 14. Considerações sobre qualidade de código

O projeto foi preparado com foco em:

- comentários autoexplicativos;
- separação de responsabilidades;
- testes automatizados;
- facilidade de manutenção;
- legibilidade para correção acadêmica.

---

## 15. Testes e benchmark

### Testes incluídos
- busca por tokens do nome;
- filtro por categoria;
- recomendação por grafo.

### Benchmark incluído
Foi adicionado `benches/search_bench.rs` com Criterion.

### Situação dos resultados
Os números ainda devem ser coletados com:

```bash
cargo bench
```

Isso é necessário porque o ambiente de geração não possui compilador Rust instalado.

---

## 16. Exemplo de uso

### Buscar produtos
```bash
cargo run -- search --query "fone bluetooth" --category "Audio" --limit 5
```

### Recomendar produtos
```bash
cargo run -- recommend --product-id P1004 --limit 3
```

---

## 17. Exemplo concreto de mercado

Duas referências relevantes para o problema proposto são:

- **Elasticsearch/OpenSearch**, pelo uso de índice invertido e busca full-text;
- **Algolia**, pela relevância, autocomplete e personalização.

O projeto entregue não tenta reproduzir toda a complexidade dessas plataformas. Ele entrega uma **versão acadêmica, modular e explicável**, suficiente para demonstrar domínio de estruturas de dados, algoritmos e Rust.

---

## 18. Conclusão

A solução proposta atende ao estudo de caso porque:

- melhora a velocidade de recuperação com tabelas hash;
- aumenta a precisão com ranking por campo;
- adiciona recomendação com grafos;
- mantém o código organizado e testável;
- prepara a base para evolução futura.

Em termos didáticos, o projeto demonstra com clareza:
- uso de `HashMap` e `HashSet`;
- modelagem de grafo em memória;
- separação modular em Rust;
- preocupação com desempenho, escalabilidade e explicabilidade.

---

## 19. Referências consultadas

1. Rust Standard Library - `std::collections::HashMap`  
2. Criterion.rs Documentation  
3. Elastic Docs - Full-text search e inverted index  
4. Algolia - documentação e materiais de relevância/autocomplete  
5. Enunciado do estudo de caso da MegaStore

---

## 20. Próximos passos sugeridos

1. Subir o projeto para um repositório público no GitHub.
2. Validar compilação com `cargo build`.
3. Executar `cargo test`.
4. Executar `cargo bench`.
5. Atualizar o relatório com números reais de benchmark.
6. Opcionalmente, evoluir para:
   - sinônimos;
   - stemming;
   - autocomplete;
   - atualização incremental;
   - persistência dos índices;
   - API HTTP.
