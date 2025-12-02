use std::array;

use nalgebra::{Point, SMatrix, Vector4};

pub trait Blendable {
    fn blend(data: [&Self], weights: &[f32], scale: f32) -> Self;
}

impl Blendable for u32 {
    fn blend(data: [&u32], weights: &[f32], scale: f32) -> u32 {
        let mut result = Vector4::zeros();

        for i in 0..data.len() {
            let channels = Vector4::from_column_slice(
                &data[i].to_be_bytes().map(|channel| channel as f32 / 256.0),
            );

            result += channels * weights[i] * scale;
        }

        u32::from_be_bytes(array::from_fn(|i| (result[i] * 256.0) as u8))
    }
}

impl Blendable for f32 {
    fn blend(data: [&Self], weights: &[f32], scale: f32) -> Self {
        let mut result = 0.0;

        for i in 0..data.len() {
            result += data[i] * weights[i] * scale;
        }

        result
    }
}

impl<const R: usize, const C: usize> Blendable for SMatrix<f32, R, C> {
    fn blend(data: [&Self], weights: &[f32], scale: f32) -> Self {
        let mut result = Self::from_element(0.0);

        for i in 0..data.len() {
            result += data[i] * weights[i];
        }

        result * scale
    }
}

impl<const D: usize> Blendable for Point<f32, D> {
    fn blend(data: [&Self], weights: &[f32], scale: f32) -> Self {
        let mut result = Self::origin();

        for i in 0..data.len() {
            result += data[i].coords * weights[i];
        }

        result * scale
    }
}
