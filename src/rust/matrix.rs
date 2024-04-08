use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, PartialEq)]
struct Vector {
    pub data: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq)]
struct Matrix {
    pub axes: Vec<Vector>,
    pub shape: (usize, usize),
}

impl Vector {
    pub fn new(data: Vec<f32>) -> Self {
        Self { data }
    }
}

impl Matrix {
    pub fn new(axes: Vec<Vector>, shape: (usize, usize)) -> Self {
        Self { axes, shape }
    }

    pub fn get_row(&self, index: usize) -> &Vector {
        self.axes.get(index).unwrap()
    }

    pub fn get_column(&self, index: usize) -> Vector {
        let mut column = vec![];

        for i in 0..self.shape.0 {
            let row = self.get_row(i);
            column.push(*row.data.get(index).unwrap());
        }

        Vector::new(column)
    }
}

///
/// Addition and subtraction
///

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        if self.data.len() != rhs.data.len() {
            panic!(
                "lhs and rhs vector are not the same length {} != {}",
                self.data.len(),
                rhs.data.len()
            );
        }

        Vector::new(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(l, r)| l + r)
                .collect(),
        )
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Self::Output {
        if self.data.len() != rhs.data.len() {
            panic!(
                "lhs and rhs vector are not the same length {} != {}",
                self.data.len(),
                rhs.data.len()
            );
        }

        Vector::new(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(l, r)| l - r)
                .collect(),
        )
    }
}

impl Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        if self.shape != rhs.shape {
            panic!(
                "lhs and rhs matrix shape are not the same length {:?} != {:?}",
                self.shape, rhs.shape
            );
        }

        Matrix::new(
            self.axes
                .iter()
                .zip(rhs.axes.iter())
                .map(|(l, r)| l + r)
                .collect(),
            self.shape,
        )
    }
}

impl Sub<&Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        if self.shape != rhs.shape {
            panic!(
                "lhs and rhs matrix shape are not the same length {:?} != {:?}",
                self.shape, rhs.shape
            );
        }

        Matrix::new(
            self.axes
                .iter()
                .zip(rhs.axes.iter())
                .map(|(l, r)| l - r)
                .collect(),
            self.shape,
        )
    }
}

///
/// Hadamard product
///

impl Mul<&Vector> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        if self.data.len() != rhs.data.len() {
            panic!(
                "lhs and rhs vector are not the same length {} != {}",
                self.data.len(),
                rhs.data.len()
            );
        }

        Vector::new(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(l, r)| l * r)
                .collect(),
        )
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        if self.shape != rhs.shape {
            panic!(
                "lhs and rhs matrix shape are not the same length {:?} != {:?}",
                self.shape, rhs.shape
            );
        }

        Matrix::new(
            self.axes
                .iter()
                .zip(rhs.axes.iter())
                .map(|(left, right)| left * right)
                .collect(),
            self.shape,
        )
    }
}

///
/// Multiplication
///

impl Vector {
    pub fn dot(&self, rhs: &Vector) -> f32 {
        if self.data.len() != rhs.data.len() {
            panic!(
                "lhs and rhs vector are not the same length {} != {}",
                self.data.len(),
                rhs.data.len()
            );
        }

        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(left, right)| left * right)
            .fold(0., |acc, v| acc + v)
    }
}

impl Matrix {
    #[allow(unused)]
    pub fn dot(&self, rhs: &Matrix) -> Matrix {
        if self.shape.1 != rhs.shape.0 {
            panic!(
                "lhs and rhs matrix shape are not compatible for matrix multiplication {:?} != {:?}",
                self.shape, rhs.shape
            );
        }

        let mut axes = vec![];
        let shape = (self.shape.0, rhs.shape.1);

        for row in self.axes.iter() {
            let mut vector = vec![];

            for column_index in 0..rhs.shape.1 {
                let column = rhs.get_column(column_index);

                vector.push(row.dot(&column))
            }

            axes.push(Vector::new(vector));
        }

        Matrix::new(axes, shape)
    }
}

///
/// Display
///

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, data) in self.data.iter().enumerate() {
            if i == self.data.len() - 1 {
                write!(f, "{}", data)?;
                continue;
            }

            write!(f, "{}, ", data)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, row) in self.axes.iter().enumerate() {
            if i == 0 {
                writeln!(f, "{}, ", row)?;
                continue;
            }

            if i == self.axes.len() - 1 {
                write!(f, " {}", row)?;
                continue;
            }

            writeln!(f, " {}, ", row)?;
        }

        write!(f, "]")?;

        Ok(())
    }
}

///
/// macro
///

macro_rules! matrix {
    ($([$($elem:expr),+]),+) => {
        {
            let mut axes = Vec::new();
            let mut shape = (0, 0);

            $(
                let vector_data = vec![$($elem as f32),+];
                let vector = Vector::new(vector_data.clone());

                axes.push(vector);
                shape.0 += 1;

                if shape.1 == 0 {
                    shape.1 = vector_data.len();
                } else {
                    assert_eq!(shape.1,
                        vector_data.len(),
                        "All rows must have the same length"
                    );
                }
            )+

            Matrix::new(axes, shape)
        }
    };
}

///
/// testing
///

fn test_matrix_multiplication() {
    {
        // 2x2 * 2x1
        let m1 = matrix![[3, 2], [1, 1]];
        let m2 = matrix![[5], [3]];
        let m3 = matrix![[21], [8]];

        assert_eq!(m1.dot(&m2), m3);
    }

    {
        // 4x2 * 2x1
        let m1 = matrix![[1, 2], [3, 4], [5, 6], [7, 8]];
        let m2 = matrix![[1], [2]];
        let m3 = matrix![[5], [11], [17], [23]];

        assert_eq!(m1.dot(&m2), m3);
    }

    {
        // 3x5 * 5x4
        let m1 = matrix![[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]];
        let m2 = matrix![
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
            [17, 18, 19, 20]
        ];
        let m3 = matrix![
            [175, 190, 205, 220],
            [400, 440, 480, 520],
            [625, 690, 755, 820]
        ];

        assert_eq!(m1.dot(&m2), m3);
    }

    {
        // 2x3 * 3x3
        let m1 = matrix![[1, 2, 3], [4, 5, 6]];
        let m2 = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let m3 = matrix![[30, 36, 42], [66, 81, 96]];

        assert_eq!(m1.dot(&m2), m3);
    }
}

fn test_matrix_add_sub() {
    {
        // 2x4 * 2x4
        let m1 = matrix![[5, 1, 3, 4], [8, 5, 3, 3]];
        let m2 = matrix![[4, 3, 3, 2], [5, 3, 3, 2]];
        let m3 = matrix![[9, 4, 6, 6], [13, 8, 6, 5]];

        assert_eq!(&m1 + &m2, m3);
    }

    {
        // 2x4 * 2x4
        let m1 = matrix![[5, 1, 3, 4], [8, 5, 3, 3]];
        let m2 = matrix![[4, 3, 3, 2], [5, 3, 3, 2]];
        let m3 = matrix![[1, -2, 0, 2], [3, 2, 0, 1]];

        assert_eq!(&m1 - &m2, m3);
    }

    {
        // 3x2 * 3x2
        let m1 = matrix![[4, 2], [3, 5], [3, 4]];
        let m2 = matrix![[2, 3], [3, 4], [4, 5]];
        let m3 = matrix![[6, 5], [6, 9], [7, 9]];

        assert_eq!(&m1 + &m2, m3);
    }
    {
        // 3x2 * 3x2
        let m1 = matrix![[4, 2], [3, 5], [3, 4]];
        let m2 = matrix![[2, 3], [3, 4], [4, 5]];
        let m3 = matrix![[2, -1], [0, 1], [-1, -1]];

        assert_eq!(&m1 - &m2, m3);
    }
}

fn test_matrix_hadamard_product() {
    {
        // 2x4 * 2x4
        let m1 = matrix![[5, 1, 3, 4], [8, 5, 3, 3]];
        let m2 = matrix![[4, 3, 3, 2], [5, 3, 3, 2]];
        let m3 = matrix![[20, 3, 9, 8], [40, 15, 9, 6]];

        assert_eq!(&m1 * &m2, m3);
    }

    {
        // 3x2 * 3x2
        let m1 = matrix![[4, 2], [3, 5], [3, 4]];
        let m2 = matrix![[2, 3], [3, 4], [4, 5]];
        let m3 = matrix![[8, 6], [9, 20], [12, 20]];

        assert_eq!(&m1 * &m2, m3);
    }
}

fn main() {
    test_matrix_multiplication();
    test_matrix_add_sub();
    test_matrix_hadamard_product();
}
