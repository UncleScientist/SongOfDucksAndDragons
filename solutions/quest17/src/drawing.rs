use macroquad::prelude::*;

pub async fn draw_x(width: usize, height: usize) {
    let (w, h) = (width as f32, height as f32);

    loop {
        let (sw, sh) = (screen_width(), screen_height());
        let (scaled_w, scaled_h) = (sw / w, sh / h);

        for row in 0..height {
            for col in 0..width {
                let (row, col) = (row as f32, col as f32);
                let rscale = row / h;
                let cscale = col / w;
                let color = match (rscale > cscale, rscale > 1.0 - cscale) {
                    (true, true) => BLUE,     // bottom
                    (true, false) => GREEN,   // left
                    (false, true) => RED,     // right
                    (false, false) => YELLOW, // top
                };
                draw_rectangle(
                    col * scaled_w,
                    row * scaled_h,
                    scaled_w - 1.0,
                    scaled_h - 1.0,
                    color,
                );
            }
        }

        next_frame().await;
    }
}
