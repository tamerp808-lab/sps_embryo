use hnsw_rs::prelude::*;
use std::sync::Mutex;

pub struct VectorIndex {
    hnsw: Mutex<Hnsw<'static, f32, f32>>,
    id_map: Mutex<Vec<String>>,
}

impl VectorIndex {
    pub fn new(dim: usize) -> Self {
        let hnsw = Hnsw::new(16, 100, dim, EuclideanDistance {});
        Self {
            hnsw: Mutex::new(hnsw),
            id_map: Mutex::new(Vec::new()),
        }
    }

    /// إضافة متجه مع معرف نصي
    pub fn insert(&self, id: String, vector: Vec<f32>) -> Result<(), String> {
        let mut hnsw = self.hnsw.lock().map_err(|e| e.to_string())?;
        let mut id_map = self.id_map.lock().map_err(|e| e.to_string())?;
        
        let idx = id_map.len();
        id_map.push(id);
        hnsw.insert((vector, idx));
        
        Ok(())
    }

    /// البحث عن أقرب المتجهات
    pub fn search(&self, query: &[f32], top_k: usize) -> Result<Vec<(String, f32)>, String> {
        let hnsw = self.hnsw.lock().map_err(|e| e.to_string())?;
        let id_map = self.id_map.lock().map_err(|e| e.to_string())?;
        
        if !hnsw.get_nb_inserted() > 0 {
            return Ok(vec![]);
        }
        
        let results = hnsw.neighbour(query, top_k, 16);
        let mut output = Vec::new();
        
        for (neighbor, dist) in results {
            if neighbor.d_id < id_map.len() {
                output.push((id_map[neighbor.d_id].clone(), dist));
            }
        }
        
        Ok(output)
    }

    pub fn len(&self) -> usize {
        self.id_map.lock().map(|m| m.len()).unwrap_or(0)
    }
}
