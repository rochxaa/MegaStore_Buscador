use std::collections::HashMap;
use crate::produto::Produto;

pub struct IndiceInvertido {
    pub indice: HashMap<String, Vec<usize>>,
}

impl IndiceInvertido {
    pub fn novo() -> Self {
        Self { indice: HashMap::new() }
    }

    pub fn adicionar_produto(&mut self, produto: &Produto) {
        let texto = format!("{} {}", produto.nome, produto.descricao);
        for token in texto.split_whitespace() {
            let termo = token.to_lowercase();
            self.indice.entry(termo).or_default().push(produto.id);
        }
    }

    pub fn buscar(&self, termo: &str) -> Option<&Vec<usize>> {
        self.indice.get(&termo.to_lowercase())
    }
}

