use rand::Rng;

use serenity::utils::Color;

pub fn random() -> Color {
    let mut rng = rand::thread_rng();

    let r = rng.gen_range(0..255);
    let g = rng.gen_range(0..255);
    let b = rng.gen_range(0..255);

    Color::from_rgb(r, g, b)
}