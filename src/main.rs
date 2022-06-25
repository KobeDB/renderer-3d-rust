mod vec4;
use vec4::Vec4;

mod figure;
use figure::Figure;

mod matrix4;

use bmp::*;

fn main() {
    println!("Hello, world!");

    let mut image = Image::new(100,100);

    for (x,y) in image.coordinates() {
        image.put_pixel(x, y, px!(255, 0, 255));
    }

    image.put_pixel(0,0, Pixel{r:255,g:255,b:255});
    image.put_pixel(0,image.get_height()-1, Pixel{r:0,g:255,b:255});

    image.save("siccimage.bmp").expect("writing to file failed");
}

// Is there a cleaner alternative to achieve this?
trait PutPixelWhereOriginIsBottomLeft {
    fn put_pixel(&mut self, x: u32, y:u32, p: Pixel);
}

impl PutPixelWhereOriginIsBottomLeft for Image {
    fn put_pixel(&mut self, x: u32, y: u32, p: Pixel) {
        self.set_pixel(x, self.get_height()-1-y, p);
    }
}