use crate::consts::*;
use nalgebra::{Complex, Matrix6, Matrix2};

pub struct SpinSeq6 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl SpinSeq6 {
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        SpinSeq6 { a, b, c, d, e, f }
    }
    pub fn diag_matrix6(&self , j : f64) -> Matrix6<Complex<f64>> {

        let jj = j * ONE;

        Matrix6::new(
            self.a * jj, ZERO, ZERO, ZERO, ZERO, ZERO,
            ZERO, self.b * jj, ZERO, ZERO, ZERO, ZERO,
            ZERO, ZERO, self.c * jj, ZERO, ZERO, ZERO,
            ZERO, ZERO, ZERO, self.d * jj, ZERO, ZERO,
            ZERO, ZERO, ZERO, ZERO, self.e * jj, ZERO,
            ZERO, ZERO, ZERO, ZERO, ZERO, self.f * jj,
        )
    }
    pub fn diag_matrix2(&self , j : f64) -> Matrix2<Complex<f64>> {

        let jj = j * ONE;

        Matrix2::new(
            self.a * jj,ZERO,
            ZERO, self.b * jj,
        )
    }
    pub fn fm() -> Self{
        SpinSeq6::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0)
    }
    pub fn afm() -> Self{
        SpinSeq6::new(1.0, -1.0, 1.0, -1.0, 1.0, -1.0)
    }
    pub fn uuuddd() -> Self{
        SpinSeq6::new(1.0, 1.0, 1.0, -1.0, -1.0, -1.0)
    }
    pub fn one1() -> Self{
        SpinSeq6::new(1.0, 1.0, 1.0, 1.0, 1.0, -1.0)
    }
    pub fn one2() -> Self{
        SpinSeq6::new(1.0, 1.0, 1.0, 1.0, -1.0, 1.0)
    }
    pub fn twin() -> Self{
        SpinSeq6::new(1.0, 1.0, 1.0, 1.0, -1.0, -1.0)
    }
    pub fn tri1() -> Self{
        SpinSeq6::new(1.0, 1.0, 1.0, -1.0, 1.0, -1.0)
    }
    pub fn tri2() -> Self{
        SpinSeq6::new(1.0, 1.0, -1.0, 1.0, -1.0, 1.0)
    }
    pub fn para() -> Self{
        SpinSeq6::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }
}