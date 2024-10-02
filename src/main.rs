mod noise;
use term_size;
use std::{thread, time::Duration};
use ansi_escapes;
use rand::random;
// fn noise_to_ascii(value: f64) -> char {
//     const ASCII_CHARS: &str = " .-=+*#%@";
//     let index = ((value + 1.0) / 2.0 * (ASCII_CHARS.len() - 1) as f64).round() as usize;
//     ASCII_CHARS.chars().nth(index).unwrap()
// }
//
// fn main() {
//     let width;
//     let height;
//
//     if let Some((w, h)) = term_size::dimensions() {
//         width = w;
//         height = h;
//     } else {
//         println!("Unable to get term size :(");
//         return;
//     }
//
//     let perlin = noise::PerlinNoise::new();
//     let scale = 0.1;
//
//     for y in 0..height {
//         for x in 0..width {
//             let value = perlin.noise(x as f64 * scale, y as f64 * scale, 0.0);
//             let ascii_char = noise_to_ascii(value);
//             print!("{}", ascii_char);
//         }
//         println!();
//     }
// }

const HEXAGON_STRING: [&str; 4] = [
"   ┏┛      ┗┓   ",
"━━━┫        ┣━━━",
"   ┗┓      ┏┛   ",
"    ┣━━━━━━┫    "
];

const SQUARES: [&str; 5] = [
"%───────",
"│       ",
"│       ",
"│       ",
"│       ",
];

const SQUARE_REMOVER: [&str; 5] = [
    "........",
    ".       ",
    ".       ",
    ".       ",
    ".       ",
];

const TRIANGLES: [&str; 3] = [
    " . ",
    ". .",
    ".. ",
];

fn hexagon(x: i16, y: i16) -> String {
    let xindex: usize = (x % 16) as usize;
    let yindex: usize = (y % 4) as usize;
    let c: String = HEXAGON_STRING[yindex].chars().nth(xindex).unwrap().into();
    // if c.is_none() {return ".".into();}
    if c == " " {return ansi_escapes::CursorMove::X(1).to_string()};
    return c;
}

fn squares(x: i16, y: i16) -> String {
    let xindex: usize = (x % 8) as usize;
    let yindex: usize = (y % 5) as usize;
    let c: String = SQUARES[yindex].chars().nth(xindex).unwrap().into();
    // if c.is_none() {return ".".into();}
    if c == " " {return ansi_escapes::CursorMove::X(1).to_string()};
    return c;
}
fn squares_2(x: i16, y: i16) -> String {
    let xindex: usize = (x % 8) as usize;
    let yindex: usize = (y % 5) as usize;
    let c: String = SQUARE_REMOVER[yindex].chars().nth(xindex).unwrap().into();
    // if c.is_none() {return ".".into();}
    if c == " " {return ansi_escapes::CursorMove::X(1).to_string()};
    if c == "." {return "".into()};
    return c;
}


fn triangles(x: i16, y: i16) -> String {
    let xindex: usize = (x % 3) as usize;
    let yindex: usize = (y % 3) as usize;
    let c: String = TRIANGLES[yindex].chars().nth(xindex).unwrap().into();
    // if c.is_none() {return ".".into();}
    if c == " " {return ansi_escapes::CursorMove::X(1).to_string()};
    return c;
}

// fn number(value: f64, x: i16, y: i16) -> String {
//     const TEXT: &str = " .-*%#";
//     let index: usize = ((value + 1.0) / 2.0 * (TEXT.len() - 1) as f64).round() as usize;
//     return TEXT.chars().nth(index).unwrap().into();
//
// }




fn noise_to_ascii(value: f64, x: i16, y:i16) -> String {
    const options_length: i32 = 6;
    let index = ((value + 1.0) / 2.0 * (options_length - 1) as f64).round() as usize;
    match index {
        0 => return hexagon(x, y),
        1 => return hexagon(x, y),
        2 => return triangles(x,y),
        3 => return squares(x, y),
        4 => return squares_2(x, y),
        5 => return squares_2(x, y),
        _ => {return "e".into() }
    }


}

fn abs(value: f64) -> f64 {
    if(value < 0.){
        return -value;
    }
    return value;
}

fn sign(value: f64) -> i8 {
    if(value > 0.){
        return 1;
    }else if(value < 0.){
        return -1;
    }
    return 0;
}

fn randInc() -> i8{
    let tmp:f64 = (random::<f64>()*2.)-1.;
    if abs(tmp) > 0.95 {
        return sign(tmp);
    }
    return 0
}

fn main() {
    let width;
    let height;

    if let Some((w, h)) = term_size::dimensions() {
        width = w;
        height = h;
    } else {
        println!("Unable to get term size :(");
        return;
    }

    // for y in 0 ..height {
    //     for x in 0..width {
    //     }
    //     println!();
    // }
    let perlin = noise::PerlinNoise::new();
    let scale = 0.1;
    let time_scale = 0.02;
    let mut time = 0.0;

    let mut xoffset:i16 = 0;
    let mut yoffset:i16 = 0;

    loop {
        // print!("{}", ansi_escapes::ClearScreen);
        // print!("{}", ansi_escapes::CursorTo::AbsoluteXY(0, 0));
        // xoffset += randInc() as i16;
        // yoffset += randInc() as i16;

        // println!("{}", ((2. * random::<f64>() - 1.)).round());

        for y in 0..height {
            print!("{}", ansi_escapes::CursorTo::AbsoluteXY(0, y as u16));
            for x in 0..width {
                let value = perlin.noise(x as f64 * scale, y as f64 * scale, time);
                print!("{}", noise_to_ascii(value, (x as i16 + xoffset) % width as i16, (y as i16 + yoffset) % height as i16));


            }
            // println!();
        }

        time += time_scale;
        thread::sleep(Duration::from_millis(50));
    }
}