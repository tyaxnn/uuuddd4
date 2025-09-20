use nalgebra::{Complex,Vector2,};

//--------------------------------------------------------------------
//                          定数の定義                    
//--------------------------------------------------------------------

pub const SQRT_3 : f64 = 1.732050807568877293527446341505872367_f64;
pub const PI     : f64 = std::f64::consts::PI;

pub const I      : Complex<f64> = Complex::new(0.,1.);
pub const ZERO   : Complex<f64> = Complex::new(0.,0.);
pub const ONE    : Complex<f64> = Complex::new(1.,0.);

//--------------------------------------------------------------------
//                          実格子のベクトルを定義                    
//--------------------------------------------------------------------

//格子空間の次近接ベクトル
pub const A1 :  Vector2<f64> = Vector2::new(-0.5 * SQRT_3, SQRT_3 / 2. * SQRT_3);
pub const A2 :  Vector2<f64> = Vector2::new(-0.5 * SQRT_3, -1. * SQRT_3 / 2. * SQRT_3);
pub const A3 :  Vector2<f64> = Vector2::new(SQRT_3,0.);

//格子空間の最近接ベクトル
pub const D1 :  Vector2<f64> = Vector2::new(SQRT_3 / 2.,0.5);
pub const D2 :  Vector2<f64> = Vector2::new(-1. * SQRT_3 / 2.,0.5);
pub const D3 :  Vector2<f64> = Vector2::new(0.,-1.);

//--------------------------------------------------------------------
//                          逆格子のベクトルを定義                    
//--------------------------------------------------------------------

const TRI : f64 = 4. * PI / 9.;

//k空間における特徴点
pub const KP_KS6 : Vector2<f64> = Vector2::new(TRI * SQRT_3 / 2.,TRI * 0.5 );
pub const KPPKS6 : Vector2<f64> = Vector2::new(0.,-TRI);
pub const GAMMA6 : Vector2<f64> = Vector2::new(0.,0.);
pub const KINKS6 : Vector2<f64> = Vector2::new(TRI * SQRT_3 / 2.,-TRI * 0.5 );
pub const MINKS6 : Vector2<f64> = Vector2::new(TRI * SQRT_3 / 2.,0. );
//pub const MP_KS6 : Vector2<f64> = Vector2::new(-TRI * SQRT_3 / 2.,0. );

pub const KP_KS2 : Vector2<f64> = Vector2::new(4. * SQRT_3 * PI / 9. * 0.5,4. * SQRT_3 * PI / 9. * SQRT_3 * 0.5);
pub const KPPKS2 : Vector2<f64> = Vector2::new(4. * SQRT_3 * PI / 9. * 0.5,-4. * SQRT_3 * PI / 9. * SQRT_3 * 0.5);
pub const GAMMA2 : Vector2<f64> = Vector2::new(0.,0.);
pub const KINKS2 : Vector2<f64> = Vector2::new(4. * SQRT_3 * PI / 9.,0.);
pub const MINKS2 : Vector2<f64> = Vector2::new(SQRT_3 * PI / 3.,PI / 3.);

pub fn kp(size: usize) -> Vector2<f64> {
    match size {
        2 => KP_KS2,
        6 => KP_KS6,
        _ => panic!("ks should be 2 or 6"),
    }
}

pub fn kpp(size: usize) -> Vector2<f64> {
    match size {
        2 => KPPKS2,
        6 => KPPKS6,
        _ => panic!("ks should be 2 or 6"),
    }
}

pub fn gamma(size: usize) -> Vector2<f64> {
    match size {
        2 => GAMMA2,
        6 => GAMMA6,
        _ => panic!("ks should be 2 or 6"),
    }
}

pub fn k(size: usize) -> Vector2<f64> {
    match size {
        2 => KINKS2,
        6 => KINKS6,
        _ => panic!("ks should be 2 or 6"),
    }
}

pub fn m(size: usize) -> Vector2<f64> {
    match size {
        2 => MINKS2,
        6 => MINKS6,
        _ => panic!("ks should be 2 or 6"),
    }
}

//--------------------------------------------------------------------
//                          ハミルトニアンのパラメーター                    
//--------------------------------------------------------------------

//hopping
pub const T      : f64 = 1.0;