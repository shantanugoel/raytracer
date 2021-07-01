use crate::color::Color;

pub struct Canvas {
    // width: usize, Probably not needed as can be derived from data
    // height: usize,Probably not needed as can be derived from data
    data: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let data: Vec<Vec<Color>> = vec![vec![Color::new(0.0, 0.0, 0.0); width]; height];
        Canvas { data }
    }

    pub fn write_pixel(self: &mut Self, x: usize, y: usize, color: Color) {
        self.data[y][x] = color;
    }

    pub fn pixel_at(self: &Self, x: usize, y: usize) -> Color {
        self.data[y][x]
    }

    pub fn to_ppm(self: &Self) -> String {
        let mut ppm = format!(
            "P3\n{} {}\n255\n",
            self.data[0].len().to_string(),
            self.data.len().to_string(),
        );
        // let data = self.data.iter().flatten().map(|x| format!("{}{}{}", x.))
        ppm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let c = Canvas::new(10, 20);
        let black = Color::new(0.0, 0.0, 0.0);
        for x in 0_usize..10_usize {
            println!("{}", x);
            for y in 0_usize..20_usize {
                assert_eq!(c.pixel_at(x, y), black);
            }
        }
    }

    #[test]
    fn test_write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(red, c.pixel_at(2, 3));
    }

    #[test]
    fn test_ppm_header() {
        let c = Canvas::new(5, 3);
        let expected = "P3\n5 3\n255\n";
        assert_eq!(expected, c.to_ppm().as_str());
    }
}