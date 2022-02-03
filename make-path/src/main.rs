use clap::Parser;
use std::fmt::Write;

#[derive(Parser, Debug)]
#[clap(author, about, long_about = None)]
struct Args {
    width: i32,
    height: i32,
    x_radius_percentage: f32,
    y_radius_percentage: Option<f32>,
}

fn main() {
    let args = Args::parse();
    let radii: [i32; 2] = [
        percentage_of(args.x_radius_percentage, args.width),
        match args.y_radius_percentage {
            Some(y) => {
                percentage_of(y, args.height)
            },
            None => {
                percentage_of(args.x_radius_percentage, args.width)
            }
        }
    ];
    let mut path = String::new();
    
    write!(path, "M {},{} ", args.width / 2, args.height / 2).unwrap();
    write!(path, "m {},0 ", -radii[0]).unwrap();
    write!(path, "a {},{} 0 0,1 {},0 ", radii[0], radii[1], 2 * radii[0]).unwrap();
    write!(path, "a {},{} 0 0,1 {},0 ", radii[0], radii[1], -2 * radii[0]).unwrap();
    
    println!("{}", path);
}

fn percentage_of(p: f32, v: i32) -> i32 {
    (p * v as f32  / 2.0) as i32
}