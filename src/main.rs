use ha5::hit::{Hit, HitList};
use ha5::image::{Color, Image};
use ha5::ray::Ray;
use ha5::sphere::Sphere;
use ha5::vec3::Vec3;
use rand::distributions::{Distribution, UnitSphereSurface};
use rand::Rng;
use std::env;
use std::f32;
use std::fs::File;
use std::io::{self, Write};

pub fn color(r: &Ray, world: &HitList) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.0, f32::MAX) {
        let sphere = UnitSphereSurface::new();
        let [x, y, z] = sphere.sample(&mut rand::thread_rng());
        let random_in_unit_sphere = Vec3::new(x as f32, y as f32, z as f32);
        let target = rec.p + rec.normal + random_in_unit_sphere;
        color(&Ray::new(rec.p, target - rec.p), world) * 0.5
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

pub fn create_image() -> Image {
    let mut image: Image = Image::new(200, 100);

    let ns = 100;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut list = HitList::default();

    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5);
    let sphere3 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5);
    let sphere4 = Sphere::new(Vec3::new(0.0, 100.5, -1.0), 100.0);

    list.push(&sphere1);
    list.push(&sphere2);
    list.push(&sphere3);
    list.push(&sphere4);

    let mut rng = rand::thread_rng();

    for y in 0..image.height {
        for x in 0..image.width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (x as f32 + rng.gen_range(0.0, 1.0)) / image.width as f32;
                let v = (y as f32 + rng.gen_range(0.0, 1.0)) / image.height as f32;
                let r = Ray::new(
                    origin,
                    lower_left_corner + horizontal * u + vertical * v - origin,
                );

                col += color(&r, &list);
            }
            col /= ns as f32;
            image[(x, y)] = Color::from(col.sqrt());
        }
    }

    image
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdout = io::stdout();

    let exit_code = match args.len() {
        1 | 2 => {
            let writer: io::Result<Box<Write>> = if args.len() == 1 {
                Ok(Box::new(stdout.lock()))
            } else {
                match File::create(&args[1]) {
                    Ok(file) => Ok(Box::new(file)),
                    Err(err) => Err(err),
                }
            };

            match writer {
                Ok(mut handle) => {
                    let image = create_image();
                    match image.write_ppm(&mut handle) {
                        Ok(ok) => ok,
                        Err(err) => {
                            eprintln!("Tried to write image but there was a problem: {}", err);
                            err.raw_os_error().unwrap()
                        }
                    }
                }
                Err(err) => {
                    println!(
                        "Tried to create file {:?} but there was a problem: {}",
                        &args[1], err
                    );
                    err.raw_os_error().unwrap()
                }
            }
        }
        _ => {
            eprintln!(
                "Program takes 1 or 2 arguments but {:?} were given",
                args.len() - 1
            );
            64
        }
    };

    std::process::exit(exit_code)
}
