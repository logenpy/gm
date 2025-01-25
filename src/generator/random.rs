use std::f32::consts::PI;

use rand::seq::SliceRandom;
use rand::Rng;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::gm::GM;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl GM {
  pub fn random(size: usize, players: usize) -> Self {
    let mut gm = GM::new(size);

    let mut rng = rand::thread_rng();

    for face in &mut gm.faces {
      face.position.x = rng.gen_range(0.0..50.0);
      face.position.y = rng.gen_range(0.0..30.0);
      face.position.z = rng.gen_range(0.0..50.0);

      let theta = rng.gen_range(0.0..PI * 2.0); // Angle in [0, 2π]
      let phi = rng.gen_range(0.0..PI); // Angle in [0, π]

      // Convert spherical coordinates to Cartesian coordinates
      face.nomal.x = phi.sin() * theta.cos();
      face.nomal.y = phi.sin() * theta.sin();
      face.nomal.z = phi.cos();

      face.radius = 1.0;
      face.sides = *[3, 4, 6].choose(&mut rng).unwrap();
    }

    for land in &mut gm.lands {
      land.set_amount(10u32.pow(rng.gen_range(0..=6)) * rng.gen_range(1..=9));
      land.set_color(rng.gen_range(0..=players as u32));
      land.set_type(rng.gen());
    }

    gm
  }
}
