# Sistema de Busca Otimizado - MegaStore

## Descrição do Projeto
O **MegaStore Search** é um sistema de busca desenvolvido em **Rust**, projetado para oferecer **consultas rápidas e precisas**.
O objetivo principal é demonstrar o uso de **estruturas de dados eficientes**, **algoritmos de busca** e boas práticas de **escalabilidade e desempenho**.

O sistema inclui:
- Uma **interface web** simples (usando HTML) para pesquisar e cadastrar produtos.  
- Um **motor de busca** implementado em Rust, utilizando **índice invertido** para buscas rápidas por palavras-chave.  
- Suporte a **armazenamento em memória**, com estrutura que pode ser modificada para armazenamento em banco de dados.

---

## Tecnologias Utilizadas

### Linguagem
- **Rust**

### Frameworks e Crates
- [`axum`](https://crates.io/crates/axum) → para criação da API e roteamento HTTP  
- [`tokio`](https://crates.io/crates/tokio) → runtime assíncrono  
- [`serde` / `serde_json`](https://crates.io/crates/serde) → serialização e desserialização de dados  
- [`tracing`](https://crates.io/crates/tracing) → logs e depuração  
- [`tower`](https://crates.io/crates/tower) → suporte adicional a middlewares  
- [`hyper`](https://crates.io/crates/hyper) → backend HTTP (interno ao Axum)

### Ferramentas
- **Cargo** → gerenciador de pacotes e build do Rust  
- **Git + GitHub** → controle de versão e hospedagem do código  
- **Vim** → ambiente de desenvolvimento recomendado  

---

##  Como Executar o Sistema

### 1️ Clonar o repositório
```bash
git clone git@github.com:rochxaa/MegaStore_Buscador.git
cd MegaStore_Buscador
