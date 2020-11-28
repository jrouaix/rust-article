use image::Rgba;
use qrcode::QrCode;

fn main() {
  let code = QrCode::new(b"Eventually Consultant").unwrap();

  let image = code
    .render::<Rgba<u8>>()
    .light_color(image::Rgba([235, 237, 240, 1]))
    .dark_color(image::Rgba([33, 110, 57, 1]))
    .quiet_zone(false)
    .build();

  // Save the image.
  image.save("qrcode.jpg").unwrap();
}
