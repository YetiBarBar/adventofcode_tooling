#[derive(Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct Matrix2D<T: Clone> {
    pub width: usize,
    pub height: usize,
    pub values: Vec<T>,
}

impl<T: Clone> Matrix2D<T> {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            values: Vec::<T>::new(),
        }
    }

    #[must_use]
    pub fn row(&self, idx: usize) -> Vec<T> {
        self.values[idx * self.width..self.width * (idx + 1)].to_vec()
    }

    #[must_use]
    pub fn col(&self, idx: usize) -> Vec<T> {
        self.values
            .iter()
            .skip(idx)
            .step_by(self.width)
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn cols(&self) -> Vec<Vec<T>> {
        (0..self.width).map(|idx| self.col(idx)).collect()
    }

    #[must_use]
    pub fn rows(&self) -> Vec<Vec<T>> {
        (0..self.height).map(|idx| self.row(idx)).collect()
    }

    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Option<T>{
        if x < self.width && y < self.height {
            Some(self.values[x + y * self.width].clone())
        }
        else {
            None
        }
    }

    #[must_use]
    pub fn transpose(&self) -> Self {
        let values: Vec<T> = self.cols().iter().flat_map(|v| v.iter()).cloned().collect();
        Self {
            width: self.height,
            height: self.width,
            values,
        }
    }
}
