pub fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> (u8, u8, u8) {
    // Convert hue, saturation, and lightness to their corresponding ranges
    let hue = hue / 360.0;
    let saturation = saturation / 100.0;
    let lightness = lightness / 100.0;

    // Calculate intermediate values
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let hue_prime = hue * 6.0;
    let x = chroma * (1.0 - ((hue_prime % 2.0) - 1.0).abs());
    let m = lightness - chroma / 2.0;

    // Calculate RGB values
    let (red, green, blue) = if hue_prime < 1.0 {
        (chroma, x, 0.0)
    } else if hue_prime < 2.0 {
        (x, chroma, 0.0)
    } else if hue_prime < 3.0 {
        (0.0, chroma, x)
    } else if hue_prime < 4.0 {
        (0.0, x, chroma)
    } else if hue_prime < 5.0 {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };

    let red = ((red + m) * 255.0) as u8;
    let green = ((green + m) * 255.0) as u8;
    let blue = ((blue + m) * 255.0) as u8;

    (red, green, blue)
}
