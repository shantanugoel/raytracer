use crate::{color::Color, matrix::Matrix};

pub struct Canvas {
    data: Matrix<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let data = Matrix::<Color>::new(height, width);
        Canvas { data }
    }

    pub fn height(self: &Self) -> usize {
        self.data.num_rows()
    }

    pub fn width(self: &Self) -> usize {
        self.data.num_cols()
    }

    pub fn write_pixel(self: &mut Self, x: usize, y: usize, color: Color) {
        self.data[y][x] = color;
    }

    #[allow(dead_code)]
    pub fn pixel_at(self: &Self, x: usize, y: usize) -> Color {
        self.data[y][x]
    }

    pub fn to_ppm(self: &Self) -> String {
        let mut ppm = format!(
            "P3\n{} {}\n255\n",
            self.width().to_string(),
            self.height().to_string(),
        );
        for row in self.data.iter() {
            for (column_index, column) in row.into_iter().enumerate() {
                ppm.push_str(column.to_string().as_str());
                if column_index < self.data.num_cols() - 1 {
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
        println!("{} {}", c.to_ppm().len(), expected.len());
        println!("{:?}", c.to_ppm());
        assert_eq!(*expected, c.to_ppm()[..expected.len()]);
    }
}
