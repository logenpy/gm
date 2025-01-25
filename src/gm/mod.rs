//! GM operations.

use std::iter;

use face::Face;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::land::Land;

pub mod edges;

#[cfg(target_arch = "wasm32")]
pub mod index;

pub mod face;

/// Game map.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct GM {
  #[wasm_bindgen(skip)]
  pub edges: Vec<Vec<usize>>,

  #[wasm_bindgen(skip)]
  pub lands: Vec<Land>,

  #[wasm_bindgen(skip)]
  pub faces: Vec<Face>
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl GM {
  /// Creates an empty [`GM`] with the given size.
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
  pub fn new(size: usize) -> Self {
    Self {
      lands: iter::repeat(Default::default()).take(size).collect(),
      edges: iter::repeat(Default::default()).take(size).collect(),
      faces: iter::repeat(Default::default()).take(size).collect()
    }
  }

  /// Loads [`GM::lands`] from lands.
  #[cfg(target_arch = "wasm32")]
  pub fn load(&mut self, lands: Vec<u32>) {
    self.lands = lands.into_iter().map(Into::into).collect();
  }

  /// Returns a pointer to [`GM::lands`] of the [`GM`].
  #[cfg(target_arch = "wasm32")]
  pub fn export_lands(&self) -> *const Land {
    self.lands.as_ptr()
  }
}
