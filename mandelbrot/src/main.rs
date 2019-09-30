extern crate num;
extern crate image;
extern crate crossbeam;

use std::str::FromStr;
use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() == 1 {
    println!("Usage: mandelbrot FILE DIMENSIONS UPPERLEFT LOWERRIGHT");
    std::process::exit(1);
  }
  if args.len() != 5 {
    eprintln!("Need five arguments");
    std::process::exit(2);
  }
  let bounds = parse_pair(&args[2], 'x').expect("need sizes, e.g. 100x200");
  let upper_left = parse_complex(&args[3]).expect("need upper left e.g. 0.0,1.0");
  let lower_right = parse_complex(&args[4]).expect("need lower right e.g. 1.0,0.0");
  let mut pixels = vec![0; bounds.0 * bounds.1];

  let threads = 8;
  let rows_per_band = bounds.1 / threads + 1;
  {
    let bands: Vec<&mut[u8]> =
    pixels.chunks_mut(rows_per_band * bounds.0).collect();
    
    crossbeam::scope(|spawner| {
      for (i, band) in bands.into_iter().enumerate() {
        let top = rows_per_band * i;
        let height = band.len() / bounds.0;
        let band_bounds = (bounds.0, height);
        let band_upper_left =
          pixel_to_point(bounds, (0, top), upper_left, lower_right);
        let band_lower_right =
          pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
        
        spawner.spawn(move |_| {
          render(band, band_bounds, band_upper_left, band_lower_right);
        });
      }
    });
  }
  println!("pixels ready, will write");
  write_image(&args[1], &pixels, bounds).expect("could not find a PNG");
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
  let one = Complex::new(1.0, 0.0);

  let mut z = Complex::new(0.0, 0.0);
  for i in 0..limit {
    z = z * z + c;
    if z.norm_sqr() > 4.0 {
      return Some(i);
    }
  }
  None
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) -> Option<Complex<f64>> {
  let mut z = Complex::new(0.0, 0.0);
  loop {
    z = z * z + c;
    if z.norm_sqr() > 4.0 {
      return Some(c);
    }
  }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
  match parse_pair(s, ',') {
    Some((re, im)) => Some(Complex{re, im}),
    None => None
  }
}

fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
  match s.find(sep) {
    None => None,
    Some(i) => {
      match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
        (Ok(l), Ok(r)) => Some((l, r)),
        _ => None
      }
    }
  }
}
fn pixel_to_point(
  bounds: (usize, usize),
  pixel: (usize, usize),
  upper_left: Complex<f64>,
  lower_right: Complex<f64>
) -> Complex<f64>
{
  let (w, h) = (lower_right.re - upper_left.re,
                            upper_left.im - lower_right.im);
  Complex {
    re: upper_left.re + pixel.0 as f64 * w  / bounds.0 as f64,
    im: upper_left.im - pixel.1 as f64 * h / bounds.1 as f64
  }  
}

fn render(
  pixels: &mut[u8],
  bounds: (usize, usize),
  upper_left: Complex<f64>,
  lower_right: Complex<f64>) {
  assert!(pixels.len() == bounds.0 * bounds.1);
  
  for row in 0 .. bounds.1 {{
    for column in 0 .. bounds.0 {
      let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
      pixels[row * bounds.0 + column] =
        match escape_time(point, 255) {
          None => 0,
          Some(n) => 255 - n as u8
        }
    }
  }}
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>
{
  let out = File::create(filename)?;
  let encoder = PNGEncoder::new(out);
  encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
  Ok(())
}
#[test]
fn test_parse_pair() {
  assert_eq!(parse_pair::<i32>("", ','), None);
  assert_eq!(parse_pair::<i32>("10,", ','), None);
  assert_eq!(parse_pair::<i32>(",10", ','), None);
  assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
  assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
  assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
  assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {
  //-1.0,1.0
  assert_eq!(parse_complex("-1.0,1.0"), Some(Complex::new(-1.0, 1.0)));
  assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex::new(1.25, -0.0625)));
  assert_eq!(parse_complex(",-0.0625"), None);
}

#[test]
fn test_pixel_to_point() {
  let bounds = (100, 100);
  let ul =  Complex {re:-1.0, im:1.0};
  let lr = Complex {re:1.0, im:-1.0};
  assert_eq!(pixel_to_point(bounds,(0, 0),ul,lr),
             ul);
  assert_eq!(pixel_to_point(bounds,(100, 0),ul,lr),
             Complex{re:1.0, im:1.0});
  assert_eq!(pixel_to_point(bounds,(0, 100),ul,lr),
             Complex{re:-1.0, im:-1.0});
  assert_eq!(pixel_to_point(bounds,(100, 100),ul,lr),
             lr);
  assert_eq!(pixel_to_point(bounds,(25, 75),ul,lr),
             Complex{re:-0.5, im:-0.5});
}