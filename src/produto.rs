#[derive(Debug, Clone, serde::Serialize)]
pub struct Produto {
    pub id: usize,
    pub nome: String,
    pub descricao: String,
    pub preco: f64,
    pub categoria: String,
}

impl Produto {
    pub fn novo(id: usize, nome: &str, descricao: &str, preco: f64, categoria: &str) -> Self {
        Self {
            id,
            nome: nome.to_string(),
            descricao: descricao.to_string(),
            preco,
            categoria: categoria.to_string(),
        }
    }
}

