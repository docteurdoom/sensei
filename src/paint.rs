use colored::{ColoredString, Colorize};
use prisma::{Rgb, Hsv, FromColor};
extern crate angular_units as angle;
use angle::Deg;

// Function that sets the HSV hue from the given temperature
// and returns a string, that is a number of certain color.
pub fn apply(celsius: f64) -> ColoredString {
    let rgb = Rgb::new(0.0, 0.0, 1.0);
    let mut hsv = Hsv::from_color(&rgb);
    hsv.set_hue(Deg(temptoang(celsius)));
    let rgb = Rgb::from_color(&hsv);
    return celsius.round()
        .to_string()
        .truecolor((rgb.red() * 255.0).round() as u8,
                   (rgb.green() * 255.0).round() as u8,
                   (rgb.blue() * 255.0).round() as u8);
}

// HSV hue 240° is dark blue and 0° is red.
// This function maps temperature into HSV hue angle
// to create a temperature color gradient.
fn temptoang(celsius: f64) -> f64 {
    return if celsius < 35.0 {
        240 as f64
    }
    else if celsius >= 35.0
        && celsius <= 80.0 {
        let x = -16.0 / 3.0;
        let y = -1280.0 / 3.0;
        celsius * x - y
    }
    else {
        0 as f64
    }
}
