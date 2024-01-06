use image::{GenericImageView, ImageBuffer, Rgb};

fn main() {
    // 画像の読み込み
    let baseimage = image::open("icons/test-icon.png").unwrap();

    // 画像の次元
    let (width, height) = baseimage.dimensions();

    // 青い画像を生成
    let mut img = baseimage.to_rgb8();

    // オーバーレイ
    for (x, y, pxl) in img.enumerate_pixels_mut() {
        if pxl == &Rgb([u8::MAX, u8::MAX, u8::MAX]) {
            *pxl = Rgb([0x0, 0xc0, 0xc0]);
        }
    }

    // 白い部分を青で埋める
    img.save("icons/test-save.png").unwrap();
}
