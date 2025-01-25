#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::gm::GM;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl GM {
  pub fn add_edge(&mut self, u: usize, v: usize) {
    self.edges[u].push(v);
    self.edges[v].push(u);
  }

  pub fn has_edge(&self, u: usize, v: usize) -> bool {
    self.edges[u].iter().any(|&x| x == v)
  }
}
