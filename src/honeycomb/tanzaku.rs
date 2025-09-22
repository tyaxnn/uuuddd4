use crate::honeycomb::{
    height_map::AllHeightMaps, 
    honeycomb_grids::Grids, setting::CalcSetting
};

use crate::system::{model::System,};
use nalgebra::Vector2;
use std::{io::Write,};

#[derive(Clone)]
pub struct Tanzakus{
    pub data : Vec<Tanzaku>,
    pub setting : CalcSetting,
    pub system : System,
}

impl Tanzakus{
    pub fn new(setting: CalcSetting, system : System) -> Self{
        let div = setting.height_map_div;
        Self{
            data : vec![Tanzaku::new(0.0,0.0,0.0,Vector2::new(0.0,0.0), Vector2::new(0.0,0.0));div],
            setting,
            system
        }
    }
    pub fn write_to_dat(&self) -> std::io::Result<()>{
        // 出力ディレクトリを作成（存在しない場合）
        std::fs::create_dir_all("./out_tanzaku/data_qmd")?;  

        let system_str = self.system.debug();
        let path = format!("./out_tanzaku/data_qmd/data_{}_{}.dat", system_str, self.setting.debug());

        let mut file = std::fs::File::create(path)?;
        writeln!(file, "# n,energy,berry,bcd_x,bcd_y,qmd_x,qmd_y")?;
        for tanzaku in &self.data{
            writeln!(file, "{},{},{},{},{},{},{}",tanzaku.n,tanzaku.energy,tanzaku.berry,tanzaku.bcd.x,tanzaku.bcd.y,tanzaku.qmd.x,tanzaku.qmd.y)?;
        }
        Ok(())
    }
    pub fn write_bcd_sum_to_tanzakus(&mut self, all_heights_maps : &AllHeightMaps) {

        let div = all_heights_maps.calc_setting.height_map_div;

        // 各エネルギーレベルごとに処理
        for energy_index in 0..div {
            let mut total_bcd_x = 0.0;
            let mut total_bcd_y = 0.0;

            let mut total_qmd_x = 0.0;
            let mut total_qmd_y = 0.0;
            
            // 全スピン、全バンドのBCD,QMDを合計
            for spin in 0..2 {
                let height_maps = all_heights_maps.index(spin);
                
                for height_map in height_maps.iter() {
                    if energy_index < height_map.contents.len() {
                        let height_map_level = &height_map.contents[energy_index];
                        
                        // このエネルギーレベルの全ラインのBCDを合計
                        for line in &height_map_level.0 {
                            if let (Some(berry), Some(anomaly_velocity), Some(gm_xx), Some(gm_xy), Some(gm_yy)) = (line.berry, line.anomaly_velocity, line.gm_xx, line.gm_xy, line.gm_yy) {
                                let bcd_x = anomaly_velocity.x * berry * line.length();
                                let bcd_y = anomaly_velocity.y * berry * line.length();

                                let qmd_x = (anomaly_velocity.y * gm_xx - anomaly_velocity.x * gm_xy) * line.length();
                                let qmd_y = (anomaly_velocity.x * gm_yy - anomaly_velocity.y * gm_xy) * line.length();

                                total_bcd_x += bcd_x;
                                total_bcd_y += bcd_y;
                                total_qmd_x += qmd_x;
                                total_qmd_y += qmd_y;
                            }
                        }
                    }
                }
            }

            self.data[energy_index].bcd = Vector2::new(total_bcd_x, total_bcd_y);
            self.data[energy_index].qmd = Vector2::new(total_qmd_x, total_qmd_y);
        }
        
    }
    pub fn write_energy_n_bc_sum_to_tanzaku(&mut self, grids: &Grids) {
        let (mesh_kx, mesh_ky) = grids.calc_setting.meshes();
        let div = grids.calc_setting.height_map_div;
        let size = grids.system.size();
        
        // エネルギー範囲を取得
        let (ground_energy, highest_energy) = grids.energy_range();
        
        // 全k点での状態を収集（エネルギーとBerry曲率のペア）
        let mut all_states = Vec::new();
        
        for i in 0..mesh_kx {
            for j in 0..mesh_ky {
                for spin in 0..2 {
                    for band_num in 0..size {
                        let band_info = &grids.index(spin)[band_num].0[i][j];
                        if let Some(berry) = band_info.berry {
                            all_states.push((band_info.eigen, berry));
                        }
                    }
                }
            }
        }
        
        // エネルギーでソート
        all_states.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        
        let total_k_points = mesh_kx * mesh_ky;
        
        for div_index in 0..div {
            let energy = ground_energy + (highest_energy - ground_energy) * (div_index as f64) / (div as f64);
            
            // このエネルギー以下の状態数をカウント
            let mut states_below_energy = 0;
            let mut berry_sum = 0.0;
            
            for (state_energy, state_berry) in &all_states {
                if *state_energy <= energy {
                    states_below_energy += 1;
                    berry_sum += state_berry;
                } else {
                    break; // ソート済みなので、これ以降は全て energy より大きい
                }
            }
            
            // 電子フィリング n を計算
            // 規約: 全充填時 n=2, 半充填時 n=1
            let n_electrons = (states_below_energy as f64) / (total_k_points as f64) / size as f64;
            
            self.data[div_index].n = n_electrons;
            self.data[div_index].energy = energy;
            self.data[div_index].berry = berry_sum;
        }
    }

}

#[derive(Clone,Copy)]
pub struct Tanzaku {
    pub n : f64,
    pub energy : f64,
    pub berry : f64,
    pub bcd : Vector2<f64>,
    pub qmd : Vector2<f64>,
}

impl Tanzaku{
    pub fn new(n:f64,energy:f64,berry:f64,bcd:Vector2<f64>,qmd:Vector2<f64>) -> Self{
        Self{
            n,
            energy,
            berry,
            bcd,
            qmd
        }
    }
}
