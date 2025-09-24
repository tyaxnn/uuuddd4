use crate::consts::*;
use crate::honeycomb::dv2::DV2;

use nalgebra::{Vector2,};


//------------------------------------------------------------------------
// 平行四辺形の格子の開始点と平行四辺形を構成するベクトルを格納する構造体
//------------------------------------------------------------------------
#[derive(Clone,Debug,Copy)]
pub struct GridInfo{
    pub ini : Vector2<f64>,
    pub dxy  : Vector2<f64>,
    pub energy_range : Option<(f64,f64)>,
}

impl GridInfo{
    pub fn new(ini_x : f64, ini_y : f64, delta_x : f64, delta_y : f64, energy_range : Option<(f64,f64)>) -> Self{
        let ini = Vector2::new(ini_x,ini_y);
        let dxy = Vector2::new(delta_x,delta_y);

        GridInfo { ini , dxy, energy_range  }
    }
    pub fn new_ijn(i : usize, j : usize, ni : usize, nj : usize, energy_range : Option<(f64,f64)>) -> Self{

        let ini_x = i as f64 / ni as f64;
        let ini_y = j as f64 / nj as f64;
        let delta_x = 1.0 / ni as f64;
        let delta_y = 1.0 / nj as f64;

        GridInfo::new(ini_x, ini_y, delta_x, delta_y, energy_range)
    }
    pub fn no_divide() -> Self{
        GridInfo::new_ijn(0,0,1,1,None)
    }
}

pub fn i_j_to_kk(
    i : usize, j : usize, 
    mesh_kx : usize, mesh_ky : usize, 
    hex : bool, 
    size : usize,
    grid_info : GridInfo,
) -> Vector2<f64>{
    let dv2_k1 = DV2::from_car(kpp(size),size) - DV2::from_car(-k(size),size);
    let dv2_k2 = DV2::from_car(kp(size),size) - DV2::from_car(-k(size),size);

    let ini= dv2_k1 * grid_info.ini.x + dv2_k2 * grid_info.ini.y + DV2::from_car(-k(size),size);
    let dv2_k1_grid = dv2_k1 * grid_info.dxy.x;
    let dv2_k2_grid = dv2_k2 * grid_info.dxy.y;

    let if64 = i as f64 / mesh_kx as f64;
    let jf64 = j as f64 / mesh_ky as f64;

    let mut kk_dv2 = dv2_k1_grid * if64 + dv2_k2_grid * jf64 + ini;

    if hex{
        if point_in_triangle_simple(kk_dv2.to_car(size), k(size), 2. * k(size), kpp(size)){
            kk_dv2 = kk_dv2 - dv2_k1;
        }
        else if point_in_triangle_simple(kk_dv2.to_car(size), k(size), 2. * k(size), kp(size)){
            kk_dv2 = kk_dv2 - dv2_k2;
        }
    }

    let kx = kk_dv2.to_car(size).x;
    let ky = kk_dv2.to_car(size).y;

    Vector2::new(kx,ky)
}

pub fn point_in_triangle_simple(
    x: Vector2<f64>,
    a: Vector2<f64>,
    b: Vector2<f64>,
    c: Vector2<f64>,
) -> bool {
    const EPSILON: f64 = 0.;

    let cross_ab_ax = (b.x - a.x) * (x.y - a.y) - (b.y - a.y) * (x.x - a.x);
    let cross_bc_bx = (c.x - b.x) * (x.y - b.y) - (c.y - b.y) * (x.x - b.x);
    let cross_ca_cx = (a.x - c.x) * (x.y - c.y) - (a.y - c.y) * (x.x - c.x);

    let all_non_negative = cross_ab_ax >= -EPSILON 
                        && cross_bc_bx >= -EPSILON 
                        && cross_ca_cx >= -EPSILON;

    let all_non_positive = cross_ab_ax <= EPSILON 
                        && cross_bc_bx <= EPSILON 
                        && cross_ca_cx <= EPSILON;

    all_non_negative || all_non_positive
}

pub fn cal_cell_area(mesh_kx : usize, mesh_ky : usize, size : usize) -> f64{
    let dv2_k1 = DV2::from_car(kpp(size),size) - DV2::from_car(-k(size),size);
    let dv2_k2 = DV2::from_car(kp(size),size) - DV2::from_car(-k(size),size);

    let lattice_1_len = dv2_k1.to_car(size).norm() / mesh_kx as f64;
    let lattice_2_len = dv2_k2.to_car(size).norm() / mesh_ky as f64;

    let cell_area = lattice_1_len * lattice_2_len * SQRT_3 * 0.5;

    cell_area
}