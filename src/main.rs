use nalgebra::Vector2;
use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;

const MAX_DARTS: usize = 100_000_000;

fn main() {

    let mut darts_that_hit_sphere: AtomicUsize = AtomicUsize::new(0);
    (1..=MAX_DARTS).into_par_iter().for_each(|_| {
        let dart: Vector2<f32> = Vector2::new_random();
        if dart.magnitude() <= 1.0 {
            darts_that_hit_sphere.fetch_add(1, Ordering::Relaxed);
        }
    });

    let ratio = *darts_that_hit_sphere.get_mut() as f64 / MAX_DARTS as f64;
    let pi = (4.0) * ratio;

    println!("pi = {pi}");
}
