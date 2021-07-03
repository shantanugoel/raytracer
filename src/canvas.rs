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
        for row in &self.data {
            for (column_index, column) in row.iter().enumerate() {
                ppm.push_str(column.to_string().as_str());

                // Don't add a space if it's the last color in the row
                if column_index < row.len() - 1 {
                    ppm.push(' ');
                }
            }
            ppm.push('\n');
        }
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
        assert_eq!(*expected, c.to_ppm()[..expected.len()]);
    }

    #[test]
    fn test_ppm() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let expected = "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
        assert_eq!(*expected, c.to_ppm()[..expected.len()]);
    }
}
