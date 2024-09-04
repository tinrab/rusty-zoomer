use clap::Parser;
use image::ImageReader;
use skia_safe::{
    images::{self},
    surfaces, Color, Data, Font, FontMgr, FontStyle, ISize, ImageInfo, Paint, Point,
};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "result.png")]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let original_image = ImageReader::open(args.input)?.decode()?;
    let original_size = ISize::new(
        original_image.width() as i32,
        original_image.height() as i32,
    );

    let bytes = original_image.into_rgba8().into_vec();
    let image_info = ImageInfo::new_n32_premul((original_size.width, original_size.height), None);
    let row_bytes = original_size.width as usize * image_info.bytes_per_pixel();
    let original_image = unsafe {
        images::raster_from_data(&image_info, Data::new_bytes(&bytes), row_bytes).unwrap()
    };

    let fm = FontMgr::default();
    let tf = fm.legacy_make_typeface(None, FontStyle::normal()).unwrap();

    // let family_names = tf.new_family_name_iterator();
    // let mut any = false;
    // for name in family_names {
    //     println!("family: {}, language: {}", name.string, name.language);
    //     any = true
    // }
    // assert!(any);

    let mut font = Font::from_typeface(tf, 48.0f32);
    font.set_subpixel(true);

    let mut surface = surfaces::raster_n32_premul(original_size).expect("surface");
    let mut paint = Paint::default();

    let scale = 1.5f32;

    let now = std::time::Instant::now();
    {
        let canvas = surface.canvas();
        canvas.clear(Color::BLACK);

        canvas.save();
        canvas.scale((scale, scale));
        canvas.draw_image(&original_image, Point::new(0.0f32, 0.0f32), None);
        canvas.restore();

        paint.set_anti_alias(true).set_color(Color::BLUE);
        canvas.draw_str(
            "AAAAAAA! ".repeat(5),
            Point::new(100.0f32, 300.0f32),
            &font,
            &paint,
        );

        let result = surface.image_snapshot();
        let result_data = result.peek_pixels().unwrap().bytes().unwrap();
        image::save_buffer(
            args.output,
            &result_data,
            original_size.width as u32,
            original_size.height as u32,
            image::ColorType::Rgba8,
        )?;
    }
    println!(
        "Rendered in {:.2}ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );

    Ok(())
}
