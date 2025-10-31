pub fn normalizar_texto(texto: &str) -> String {
    texto.to_lowercase()
        .replace(",", "")
        .replace(".", "")
        .replace("!", "")
        .replace("?", "")
}

