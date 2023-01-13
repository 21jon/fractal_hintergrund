mod picture_rendering_fractals;

use crate::picture_rendering_fractals::FractalPicture;

fn main() {
    let mut picture = FractalPicture::new(
        (7680, 4320),
        (-0.8, 0.156),
        "test.png".to_string(),
        (0, 0),
        1.0,
        ((0, 0, 255), (0, 255, 0)),
    );

    picture.render();

    picture.save();
}

//230, 114, 5
//207, 76, 6
