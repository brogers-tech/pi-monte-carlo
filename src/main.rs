use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::Write;
use std::io;

use nalgebra::Vector2;
use rayon::prelude::*;

fn get_input() -> io::Result<usize> {
    let mut userin = String::new();
    print!("Number of darts to throw: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut userin)?;
    Ok(userin.trim().parse::<usize>().unwrap_or(0))
}

fn throw_darts(max_darts: usize) -> usize {
    let mut darts_that_hit_sphere: AtomicUsize = AtomicUsize::new(0);
    (1..=max_darts).into_par_iter().for_each(|_| {
        let dart: Vector2<f32> = Vector2::new_random();
        if dart.magnitude() <= 1.0 {
            darts_that_hit_sphere.fetch_add(1, Ordering::Relaxed);
        }
    });

    *darts_that_hit_sphere.get_mut()
}

fn main() -> io::Result<()> {
    let max_darts = get_input().unwrap();
    if max_darts > 0 {
        println!("Throwing {max_darts} darts");
        let darts_that_hit = throw_darts(max_darts);    

        let ratio = darts_that_hit as f64 / max_darts as f64;
        let pi = (4.0) * ratio;

        println!("pi = {pi}");
    } else {
        println!("No darts thrown.");
    }

    Ok(())
}
