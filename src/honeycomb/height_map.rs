use crate::honeycomb::{
    honeycomb_grids::{BandInfo, Grid, Grids},
    cal_berry::{calculate_berry_curvature_from_seud,cal_anomaly_velocity},
    setting::CalcSetting,
    util::cal_cell_area,
};

use crate::system::{
    model::System,
    diag::{diag,}
};

use nalgebra::{Vector2};
use std::{fs::File,};
use std::io::{Write, Result as IoResult};

pub struct AllHeightMaps{
    pub u : Vec<HeightMaps>,
    pub d : Vec<HeightMaps>,
    pub calc_setting : CalcSetting,
}

impl AllHeightMaps{
    pub fn build(
        grids : &Grids
    ) -> Self{
        let size = grids.system.size();
        let (mesh_kx,mesh_ky) = grids.calc_setting.meshes();
        let div = grids.calc_setting.height_map_div;

        let (highest_energy, ground_energy) = grids.energy_range();

        let mut out = Self::ini(grids.calc_setting);

        for ud in 0..2{
            for band_num in 0..size{
                let grid = grids.index(ud)[band_num].clone();
                let mut height_map = HeightMaps::initialize(ground_energy, highest_energy, div);

                for i in 0..mesh_kx-1{
                    for j in 0..mesh_ky-1{
                        //ここでHeightMapを作る
                        let cell = Cell::from_grid(&grid, i, j, mesh_kx, mesh_ky);

                        let (min_index, max_index) = cell.index_range(&height_map);

                        for index in min_index..=max_index{
                            let enerygy = height_map.index_2_energy(&index);

                            let ab = div_internal(cell.a, cell.b, enerygy);
                            let bc = div_internal(cell.b, cell.c, enerygy); 
                            let cd = div_internal(cell.c, cell.d, enerygy);
                            let da = div_internal(cell.d, cell.a, enerygy);
                            let bd = div_internal(cell.b, cell.d, enerygy);
                            
                            //先ず、三角形ABDについて等高線を引く
                            if let Some(line) = create_triangle_line(ab, bd, da) {
                                let mut  calced_line = line;
                                //line上でのBerry曲率と異常速度を計算してセットする
                                calced_line.set_berry_and_anomaly_velocity(&grids.calc_setting, &grids.system, ud, band_num);
                                height_map.contents[index].0.push(calced_line);
                            }
                            //次に、三角形BCDについて等高線を引く
                            if let Some(line) = create_triangle_line(bc, cd, bd) {
                                let mut  calced_line = line;
                                //line上でのBerry曲率と異常速度を計算してセットする
                                calced_line.set_berry_and_anomaly_velocity(&grids.calc_setting, &grids.system, ud, band_num);
                                height_map.contents[index].0.push(calced_line);
                            }
                            

                        }
                    }
                }

                out.index_mut(ud).push(height_map);
            }
        }

        out
    }
    pub fn ini(calc_setting: CalcSetting) -> Self{
        AllHeightMaps { u: Vec::new(), d: Vec::new(), calc_setting }
    }
    pub fn index(&self, index : usize) -> &Vec<HeightMaps>{
        match index {
            0 => &self.u,
            1 => &self.d,
            _ => panic!("index should be 0 or 1"),
        }
    }
    pub fn index_mut(&mut self, index : usize) -> &mut Vec<HeightMaps>{
        match index {
            0 => &mut self.u,
            1 => &mut self.d,
            _ => panic!("index should be 0 or 1"),
        }
    }

    /// AllHeightMapsをPythonで可視化できる形式で.datファイルに出力する
    /// 出力形式: energy kx ky line_start_kx line_start_ky line_end_kx line_end_ky spin band_index
    pub fn write_to_dat(&self, filename: &str) -> IoResult<()> {
        let mut file = File::create(filename)?;
        
        // ヘッダーを書き込み
        writeln!(file, "# energy,line_start_kx,line_start_ky,line_end_kx,line_end_ky,bcd_x,bcd_y,spin,band_index")?;
        
        // スピンごと（u=0, d=1）
        for spin in 0..2 {
            let height_maps = self.index(spin);
            
            // バンドごと
            for (band_index, height_map) in height_maps.iter().enumerate() {
                
                // エネルギーレベルごと
                for (energy_index, height_map_level) in height_map.contents.iter().enumerate() {
                    let energy = height_map.index_2_energy(&energy_index);
                    
                    // 各ラインを出力
                    for line in &height_map_level.0 {
                        let bcd_x = line.anomaly_velocity.unwrap().x * line.berry.unwrap() * line.length();
                        let bcd_y = line.anomaly_velocity.unwrap().y * line.berry.unwrap() * line.length();
                        writeln!(
                            file,
                            "{:.6},{:.6},{:.6},{:.6},{:.6},{:.12},{:.12},{},{}",
                            energy,
                            line.ini.x, line.ini.y,
                            line.end.x, line.end.y,
                            bcd_x, bcd_y,
                            spin,
                            band_index
                        )?;
                    }
                }
            }
        }
        
        Ok(())
    }

    /// outputディレクトリに等高線データを出力する便利メソッド
    pub fn write_to_output_dir(&self) -> IoResult<()> {
        // outputディレクトリを作成
        std::fs::create_dir_all("./output")?;
        
        // メインの等高線データを出力
        self.write_to_dat("./output/contour_lines.dat")?;
        
        println!("Contour data written to ./output/contour_lines.dat");
        println!("Use Python scripts in ./output/ to visualize the data");
        
        Ok(())
    }

    }



#[derive(Debug, Clone)]
pub struct HeightMaps{
    pub ground_energy : f64,
    pub highest_energy : f64,
    pub div : usize,
    pub contents: Vec<HeightMap>
}

impl HeightMaps{
    pub fn initialize(ground_energy: f64, highest_energy: f64, div: usize) -> Self{
        HeightMaps {
            ground_energy,
            highest_energy,
            div,
            contents: vec![HeightMap::ini(); div],
        }
    }
    pub fn index_2_energy(&self, index : &usize) -> f64{
        let range = self.highest_energy - self.ground_energy;
        self.ground_energy + range * (*index as f64) / (self.div as f64)
    }
    pub fn energy_2_index(&self, energy : &f64) -> usize{
        let range = self.highest_energy - self.ground_energy;
        let frac = (energy - self.ground_energy) / range;
        let index = (frac * (self.div as f64)).floor() as usize;

        if index >= self.div{
            self.div - 1
        } else{
            index
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeightMap(pub Vec<Line>);

impl HeightMap {
    pub fn ini() -> Self{
        HeightMap(Vec::new())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line{
    pub ini : Vector2<f64>,
    pub end : Vector2<f64>,
    pub berry : Option<f64>,
    pub anomaly_velocity : Option<Vector2<f64>>,
}

impl Line{
    pub fn new(ini : Vector2<f64>, end : Vector2<f64>) -> Self{
        Line { ini, end, berry: None, anomaly_velocity: None }
    }
    pub fn length(&self) -> f64{
        let diff = self.end - self.ini;
        diff.norm()
    }
    pub fn center(&self) -> Vector2<f64>{
        (self.ini + self.end) / 2.0
    }
    pub fn set_berry_and_anomaly_velocity(&mut self, calc_setting: &CalcSetting, system : &System, ud : usize, band_num : usize){
        let kk = self.center();
        let seud = diag(system,kk);
        let cell_area = cal_cell_area(calc_setting.mesh_kx, calc_setting.mesh_ky, system.size());
        self.berry = Some(calculate_berry_curvature_from_seud(&seud, system, kk,cell_area)[ud][band_num] / cell_area);
        self.anomaly_velocity = Some(cal_anomaly_velocity(&seud, system, kk, ud, band_num));
    }
}

pub struct Cell{
    pub a : BandInfo,
    pub b : BandInfo,
    pub c : BandInfo,
    pub d : BandInfo,
}

impl Cell{
    pub fn from_grid(
        grid : &Grid,
        i : usize,
        j : usize,
        mesh_kx : usize,
        mesh_ky : usize
    ) -> Self{
        let a = &grid.0[i][j];
        let b = &grid.0[(i+1)%mesh_kx][j];
        let c = &grid.0[(i+1)%mesh_kx][(j+1)%mesh_ky];
        let d = &grid.0[i][(j+1)%mesh_ky];

        Cell {
            a: *a,
            b: *b,
            c: *c,
            d: *d,
        }
    }
    pub fn index_range(&self, height_maps : &HeightMaps) -> (usize,usize){
        let indices = vec![
            height_maps.energy_2_index(&self.a.eigen),
            height_maps.energy_2_index(&self.b.eigen),
            height_maps.energy_2_index(&self.c.eigen),
            height_maps.energy_2_index(&self.d.eigen)
        ];

        let max = *indices.iter().max().unwrap();
        let min = *indices.iter().min().unwrap();
        (min, max)
    }
}

fn div_internal(p1 : BandInfo, p2 : BandInfo, energy : f64) -> Option<Vector2<f64>>{
    let (e1,e2) = (p1.eigen, p2.eigen);

    if (e1 - energy) * (e2 - energy) >= 0.0{
        None
    } else{
        let frac = (energy - e1) / (e2 - e1);
        let diff = p2.kk - p1.kk;
        Some(p1.kk + diff * frac)
    }
}

fn create_triangle_line(
    edge1: Option<Vector2<f64>>, 
    edge2: Option<Vector2<f64>>, 
    edge3: Option<Vector2<f64>>
) -> Option<Line> {
    if edge1.is_some() && edge2.is_some() && edge3.is_none() {
        Some(Line::new(edge1.unwrap(), edge2.unwrap()))
    }
    else if edge1.is_some() && edge2.is_none() && edge3.is_some() {
        Some(Line::new(edge3.unwrap(), edge1.unwrap()))
    }
    else if edge1.is_none() && edge2.is_some() && edge3.is_some() {
        Some(Line::new(edge2.unwrap(), edge3.unwrap()))
    }
    else if edge1.is_none() && edge2.is_none() && edge3.is_none() {
        // 等高線は横切らなかった
        None
    }
    else {
        println!("等高線はあり得ない横切り方をしている {} {} {}", edge1.is_some(), edge2.is_some(), edge3.is_some());
        None
    }
}