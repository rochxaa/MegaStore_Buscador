use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    Router, routing::get, response::{Html, Json}, extract::State
};
use tokio::fs;
use std::collections::HashMap;

mod produto;
mod indice;
mod busca;
mod utils;

use produto::Produto;
use indice::IndiceInvertido;
use busca::buscar_query;
use axum::extract::Json as AxumJson;
use serde::Deserialize;

#[derive(Deserialize)]
struct NovoProduto {
    nome: String,
    descricao: String,
    preco: f64,
    categoria: String,
}

#[derive(serde::Serialize)]
struct Resultado {
    id: usize,
    nome: String,
    preco: f64,
    categoria: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Produtos iniciais
    let produtos = vec![
        Produto::novo(1, "Notebook Gamer", "Desempenho extremo com RTX 4060", 5999.90, "Informática"),
        Produto::novo(2, "Mouse sem fio", "Ergonômico e rápido", 129.90, "Periféricos"),
        Produto::novo(3, "Smartphone Mega X", "Tela AMOLED e bateria de longa duração", 2999.00, "Celulares"),
        Produto::novo(4, "Fone Bluetooth", "Som estéreo e cancelamento de ruído", 399.00, "Áudio"),
    ];

    let mut indice = IndiceInvertido::novo();
    for produto in &produtos {
        indice.adicionar_produto(produto);
    }

    let estado = Arc::new((RwLock::new(indice), produtos));

    let app = Router::new()
        .route("/", get(home))
        .route("/search", get(buscar_handler))
        .route("/cadastro", get(cadastro_page))
        .route("/cadastro", axum::routing::post(cadastro_handler))
        .with_state(estado);

    println!("🚀 Servidor rodando em http://localhost:8080");

    use tokio::net::TcpListener;
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ========================
// Página inicial HTML
// ========================
async fn home() -> Html<String> {
    let html = fs::read_to_string("templates/index.html").await.unwrap();
    Html(html)
}

// ========================
// Página de cadastro HTML
// ========================
async fn cadastro_page() -> Html<String> {
    let html = fs::read_to_string("templates/cadastro.html").await.unwrap();
    Html(html)
}

// ========================
// Handler de busca
// ========================
async fn buscar_handler(
    State(estado): State<Arc<(RwLock<IndiceInvertido>, Vec<Produto>)>>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> Json<Vec<Resultado>> {
    let query = params.get("q").unwrap_or(&"".to_string()).to_lowercase();
    let (indice, produtos) = &*estado;
    let indice = indice.read().await;

    let ids = buscar_query(&indice, &query);

    let resultados: Vec<Resultado> = produtos.iter()
        .filter(|p| ids.contains(&p.id))
        .map(|p| Resultado {
            id: p.id,
            nome: p.nome.clone(),
            preco: p.preco,
            categoria: p.categoria.clone(),
        })
        .collect();

    Json(resultados)
}

// ========================
// Handler de cadastro POST
// ========================
async fn cadastro_handler(
    State(estado): State<Arc<(RwLock<IndiceInvertido>, Vec<Produto>)>>,
    AxumJson(produto): AxumJson<NovoProduto>,
) -> &'static str {
    let (indice, produtos) = &*estado;

    let mut indice = indice.write().await;
    let mut produtos = produtos.clone();
    let novo_id = produtos.len() + 1;

    let novo = Produto::novo(
        novo_id,
        &produto.nome,
        &produto.descricao,
        produto.preco,
        &produto.categoria,
    );

    produtos.push(novo.clone());
    indice.adicionar_produto(&novo);

    "Produto cadastrado com sucesso!"
}

