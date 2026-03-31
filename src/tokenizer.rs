/// Conjunto reduzido de utilitários de tokenização.
///
/// O objetivo aqui é manter a solução simples, didática e 100% em Rust padrão,
/// removendo ruído textual e normalizando acentos.

/// Converte texto para um formato mais amigável à indexação:
/// - minúsculas;
/// - remoção de acentos mais comuns em português;
/// - substituição de pontuação por espaço.
pub fn normalize(text: &str) -> String {
    text.chars()
        .map(fold_char)
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect::<String>()
}

/// Divide o texto em tokens úteis à busca.
pub fn tokenize(text: &str) -> Vec<String> {
    normalize(text)
        .split_whitespace()
        .filter(|token| token.len() > 1)
        .map(|token| token.to_string())
        .collect()
}

/// Faz um "fold" simples de acentos.
///
/// Essa abordagem é suficiente para a proposta acadêmica e evita dependências
/// externas só para normalização Unicode.
fn fold_char(c: char) -> char {
    match c {
        'á' | 'à' | 'â' | 'ã' | 'ä' | 'Á' | 'À' | 'Â' | 'Ã' | 'Ä' => 'a',
        'é' | 'è' | 'ê' | 'ë' | 'É' | 'È' | 'Ê' | 'Ë' => 'e',
        'í' | 'ì' | 'î' | 'ï' | 'Í' | 'Ì' | 'Î' | 'Ï' => 'i',
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' | 'Ó' | 'Ò' | 'Ô' | 'Õ' | 'Ö' => 'o',
        'ú' | 'ù' | 'û' | 'ü' | 'Ú' | 'Ù' | 'Û' | 'Ü' => 'u',
        'ç' | 'Ç' => 'c',
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize, tokenize};

    #[test]
    fn normalize_removes_accents_and_symbols() {
        let value = normalize("Câmera, Áudio & Vídeo!");
        assert_eq!(value, "camera  audio   video ");
    }

    #[test]
    fn tokenize_discards_short_noise() {
        let tokens = tokenize("tv 4k e som");
        assert_eq!(tokens, vec!["tv", "4k", "som"]);
    }
}
