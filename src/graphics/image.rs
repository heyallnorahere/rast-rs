use std::iter;
use std::mem;

pub struct Image<T: Sized> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,

    pub color: Vec<Image<u32>>,
    pub depth: Option<Image<f32>>,
}

impl<T: Sized + Default> Image<T> {
    pub fn new(width: usize, height: usize) -> Image<T> {
        let total_pixels = width * height;

        Image {
            data: Vec::from_iter(iter::repeat_with(|| T::default()).take(total_pixels)),
            width: width,
            height: height,
        }
    }
}

impl<T : Sized> Image<T> {
    fn index_of(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(y * self.width + x)
        }
    }

    pub fn at<'a>(&'a self, x: usize, y: usize) -> Option<&'a T> {
        self.index_of(x, y).map(|index| &self.data[index])
    }

    pub fn exchange(&mut self, x: usize, y: usize, value: T) -> Option<T> {
        self.index_of(x, y).map(|index| {
            let mut other = value;
            mem::swap(&mut other, &mut self.data[index]);

            other
        })
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl Framebuffer {
    fn validate_attachment<T: Sized>(&self, attachment: &Image<T>) -> Result<(), String> {
        if self.width != attachment.width || self.height != attachment.height {
            Err(format!(
                "Size mismatch on framebuffer attachment! ({0}x{1} vs {2}x{3})",
                self.width, self.height, attachment.width, attachment.height
            ))
        } else {
            Ok(())
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        for color in &self.color {
            self.validate_attachment(color)?;
        }

        match &self.depth {
            Some(depth) => self.validate_attachment(depth),
            None => Ok(())
        }?;

        Ok(())
    }
}
