use std::collections::HashMap;

pub struct SimpleVectorMemory {
    vectors: HashMap<String, Vec<f32>>,
}

impl SimpleVectorMemory {
    pub fn new() -> Self {
        Self {
            vectors: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, vec: Vec<f32>) {
        self.vectors.insert(key, vec);
    }

    pub fn search(&self, query_vec: &[f32], top_k: usize) -> Vec<String> {
        let mut scored: Vec<(f64, &String)> = self
            .vectors
            .iter()
            .map(|(k, v)| (cosine_similarity(query_vec, v), k))
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored
            .into_iter()
            .take(top_k)
            .map(|(_, k)| k.clone())
            .collect()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    let dot: f64 = a.iter().zip(b).map(|(x, y)| *x as f64 * *y as f64).sum();
    let na: f64 = a.iter().map(|x| *x as f64 * *x as f64).sum::<f64>().sqrt();
    let nb: f64 = b.iter().map(|x| *x as f64 * *x as f64).sum::<f64>().sqrt();
    if na < 1e-8 || nb < 1e-8 {
        return 0.0;
    }
    dot / (na * nb)
}
