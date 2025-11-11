#[derive(Clone, Debug)]
pub struct Kernel {
    pub width: usize,
    pub height: usize,
    pub data: Vec<i32>,
    pub divider: i32,
}

impl Kernel {
    pub fn new(size: usize) -> Self {
        Self {
            width: size,
            height: size,
            data: vec![0; size * size],
            divider: 0,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> i32 {
        self.data[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: i32) {
        self.data[y * self.width + x] = value;
    }

    pub fn create_blur(size: usize) -> Self {
        let mut kernel = Self::new(size);
        for y in 0..size {
            for x in 0..size {
                kernel.set(x, y, 1);
                kernel.divider += 1;
            }
        }
        kernel
    }

    pub fn create_sharpen() -> Self {
        let mut kernel = Self::new(3);
        let data = [
            0, -1, 0,
            -1, 5, -1,
            0, -1, 0,
        ];
        kernel.data.copy_from_slice(&data);
        kernel.divider = data.iter().sum();
        kernel
    }

    pub fn create_uncanny_sharpen() -> Self {
        let mut kernel = Self::new(3);
        let data = [
            -1, -1, -1,
            -1, 9, -1,
            -1, -1, -1,
        ];
        kernel.data.copy_from_slice(&data);
        kernel.divider = data.iter().sum();
        kernel
    }

    pub fn create_directional_weird() -> Self {
        let mut kernel = Self::new(3);
        let data = [
            2, 0, -2,
            1, 0, -1,
            0, 0, 0,
        ];
        kernel.data.copy_from_slice(&data);
        kernel.divider = 1;
        kernel
    }

    pub fn create_basic() -> Self {
        let mut kernel = Self::new(3);
        let data = [
            0, 0, 0,
            0, 1, 0,
            0, 0, 0,
        ];
        kernel.data.copy_from_slice(&data);
        kernel.divider = data.iter().sum();
        kernel
    }
}
