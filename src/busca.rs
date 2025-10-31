use crate::indice::IndiceInvertido;

pub fn buscar_query(indice: &IndiceInvertido, consulta: &str) -> Vec<usize> {
    let mut resultados: Vec<usize> = Vec::new();

    for termo in consulta.split_whitespace() {
        if let Some(ids) = indice.buscar(termo) {
            resultados.extend(ids);
        }
    }

    resultados.sort();
    resultados.dedup();
    resultados
}

