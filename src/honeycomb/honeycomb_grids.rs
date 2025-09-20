use std::vec;

use crate::{
    system::{
    diag::{diag, SEudEnum}, model::System
}};
use crate::honeycomb::{
    util::{i_j_to_kk,cal_cell_area},
    setting::CalcSetting,
    cal_berry::calculate_berry_curvature_from_seud,
};

use nalgebra::{Complex, Vector2, Vector6};

pub struct Grids{
    pub u : Vec<Grid>,
    pub d : Vec<Grid>,
    pub system : System,
    pub calc_setting : CalcSetting,
}
impl Grids{
    pub fn to_iter(&self) -> [&Vec<Grid>;2]{
        [&self.u,&self.d]
    }
    pub fn index(&self, index : usize) -> &Vec<Grid>{
        match index {
            0 => &self.u,
            1 => &self.d,
            _ => panic!("index should be 0 or 1"),
        }
    }
    pub fn index_mut(&mut self, index : usize) -> &mut Vec<Grid>{
        match index {
            0 => &mut self.u,
            1 => &mut self.d,
            _ => panic!("index should be 0 or 1"),
        }
    }
    pub fn energy_range(&self) -> (f64,f64){
        let up_ground = self.u[0].energy_range().0;
        let up_heighest = self.u[self.system.size() - 1].energy_range().1;
        let down_ground = self.d[0].energy_range().0;
        let down_heighest = self.d[self.system.size() - 1].energy_range().1;

        (up_ground.min(down_ground), up_heighest.max(down_heighest))
    }
}

#[derive(Debug, Clone)]
pub struct Grid(pub Vec<Vec<BandInfo>>);

impl Grid{
    pub fn energy_range(&self) -> (f64,f64){
        let vecvec = &self.0;

        let mut heighest = 0.0;
        let mut ground = f64::MAX;

        for vec in vecvec{
            for band_info in vec{
                let energy = band_info.eigen;

                if energy > heighest{
                    heighest = energy;
                }
                if energy < ground{
                    ground = energy
                }
            }
        }

        (ground,heighest)
    }
}


#[derive(Debug, Clone, Copy)]
pub struct BandInfo{
    pub kk : Vector2<f64>,
    pub i : Option<usize>,
    pub j : Option<usize>,
    pub eigen : f64,
    pub eigen_vector : EigenVectorEnum,
    pub berry : Option<f64>,

}

impl BandInfo{
    pub fn ini() -> Self{
        BandInfo { 
            kk: Vector2::zeros(), 
            eigen: 0.0, 
            i: None,
            j: None,
            eigen_vector: EigenVectorEnum::None,
            berry: None,
        }
    }
    pub fn new(kk : Vector2<f64>, i : usize, j : usize, eigen: f64, eigen_vector: EigenVectorEnum)-> Self{
        BandInfo { kk, i : Some(i), j : Some(j), eigen , eigen_vector, berry : None }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EigenVectorEnum{
    EigenVector6(Vector6<Complex<f64>>),
    EigenVector2(Vector2<Complex<f64>>),
    None
}

impl EigenVectorEnum{
    pub fn is_2(&self) -> Vector2<Complex<f64>>{
        match self{
            EigenVectorEnum::EigenVector2(vec) => *vec,
            _ => panic!("size should be 2"),
        }
    }
    pub fn is_6(&self) -> Vector6<Complex<f64>>{
        match self{
            EigenVectorEnum::EigenVector6(vec) => *vec,
            _ => panic!("size should be 6"),
        }
    }
}

impl Grids{
    /// Gridsを構築し、各k点での対角化と同時にBerry曲率も計算する
    /// 
    /// この実装では、従来のcal_berry関数を呼び出す必要がありません。
    /// 各k点での対角化直後にBerry曲率を計算することで、
    /// ハミルトニアンの微分計算の重複を避け、効率的な実装となっています。
    /// 
    /// # Arguments
    /// * `calc_setting` - 計算設定（メッシュサイズなど）
    /// * `system` - システム情報
    /// 
    /// # Returns
    /// * `Self` - Berry曲率が計算済みのGrids
    pub fn build(
        calc_setting : CalcSetting,
        system : System
    ) -> Self{
        let mesh_kx = calc_setting.mesh_kx;
        let mesh_ky = calc_setting.mesh_ky;

        let size = system.size();

        let grid: Grid = Grid(vec![vec![BandInfo::ini();mesh_ky];mesh_kx]);

        let grid_u: Vec<Grid> = vec![grid.clone();size];
        let grid_d: Vec<Grid> = vec![grid;size];

        let mut grids = Grids { u: grid_u, d: grid_d, system, calc_setting };

        // セル面積を事前計算
        let cell_area = cal_cell_area(mesh_kx, mesh_ky, size);

        for i in 0..mesh_kx{
            for j in 0..mesh_ky{
                let kk = i_j_to_kk(i, j, mesh_kx, mesh_ky, false, system.size());

                let seud_enum = diag(&system,kk);

                // SEudEnumからBerry曲率を計算
                let berry_curvatures = calculate_berry_curvature_from_seud(&seud_enum, &system, kk, cell_area);

                match seud_enum{
                    SEudEnum::SEud2(seud) => {
                        for index in 0..2{
                            for band_num in 0..size{                             
                                grids.index_mut(index)[band_num].0[i][j] = {
                                    let eigen = seud.index(index).eigenvalues[band_num];
                                    let eigen_vector: Vector2<Complex<f64>> = seud.index(index).eigenvectors.column(band_num).into();
                                    let mut band_info = BandInfo::new(kk, i, j, eigen, EigenVectorEnum::EigenVector2(eigen_vector));
                                    // Berry曲率を設定
                                    band_info.berry = Some(berry_curvatures[index][band_num]);
                                    band_info
                                }
                            }
                        }
                    }
                    SEudEnum::SEud6(seud) => {
                        for index in 0..2{
                            for band_num in 0..size{                             
                                grids.index_mut(index)[band_num].0[i][j] = {
                                    let eigen = seud.index(index).eigenvalues[band_num];
                                    let eigen_vector: Vector6<Complex<f64>> = seud.index(index).eigenvectors.column(band_num).into();
                                    let mut band_info = BandInfo::new(kk, i, j, eigen, EigenVectorEnum::EigenVector6(eigen_vector));
                                    // Berry曲率を設定
                                    band_info.berry = Some(berry_curvatures[index][band_num]);
                                    band_info
                                }
                            }
                        }
                    }
                }
            }
        }

        grids
    }
}