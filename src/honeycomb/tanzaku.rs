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
    pub fn write_to_dat(&self,dir :  Option<&String>) -> std::io::Result<()>{

        let dir = match dir{
            Some(d) => d,
            None => &String::from("./out_tanzaku/data_qmd"),
        };
        // 出力ディレクトリを作成（存在しない場合）
        std::fs::create_dir_all(&dir)?;

        let system_str = self.system.debug();
        let path = format!("{}/data_{}_{}.dat", dir, system_str, self.setting.debug());

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

            let weight = 1.0 / (self.setting.main_mesh as f64).powi(2);
            
            self.data[div_index].n = n_electrons * weight;
            self.data[div_index].energy = energy;
            self.data[div_index].berry = berry_sum * weight;
        }
    }

    /// nに対して等間隔なデータセットに線形補間で変換する
    /// n_div: nの分割数
    pub fn interpolate_by_n(&self, n_div: usize) -> Self {
        if self.data.is_empty() {
            return self.clone();
        }

        // 新しいデータセットを作成
        let mut new_data = Vec::with_capacity(n_div);
        
        for i in 0..n_div {
            let target_n = 2.0 * (i as f64) / ((n_div) as f64);
            
            // 線形補間で各値を計算
            let interpolated = self.linear_interpolate_at_n(target_n);
            new_data.push(interpolated);
        }

        Self {
            data: new_data,
            setting: self.setting.clone(),
            system: self.system.clone(),
        }
    }

    /// 指定されたnの値で線形補間を行う
    fn linear_interpolate_at_n(&self, target_n: f64) -> Tanzaku {
        // nでソートされたデータを取得
        let mut sorted_data = self.data.clone();
        sorted_data.sort_by(|a, b| a.n.partial_cmp(&b.n).unwrap());

        // target_n が範囲外の場合の処理
        if target_n <= sorted_data[0].n {
            return sorted_data[0];
        }
        if target_n >= sorted_data[sorted_data.len() - 1].n {
            return sorted_data[sorted_data.len() - 1];
        }

        // 補間する2点を見つける
        for i in 0..sorted_data.len() - 1 {
            let t1 = &sorted_data[i];
            let t2 = &sorted_data[i + 1];
            
            if target_n >= t1.n && target_n <= t2.n {
                // 線形補間の重み計算
                let weight = (target_n - t1.n) / (t2.n - t1.n);
                
                // 各値を線形補間
                let energy = t1.energy + weight * (t2.energy - t1.energy);
                let berry = t1.berry + weight * (t2.berry - t1.berry);
                
                let bcd_x = t1.bcd.x + weight * (t2.bcd.x - t1.bcd.x);
                let bcd_y = t1.bcd.y + weight * (t2.bcd.y - t1.bcd.y);
                let bcd = Vector2::new(bcd_x, bcd_y);
                
                let qmd_x = t1.qmd.x + weight * (t2.qmd.x - t1.qmd.x);
                let qmd_y = t1.qmd.y + weight * (t2.qmd.y - t1.qmd.y);
                let qmd = Vector2::new(qmd_x, qmd_y);
                
                return Tanzaku::new(target_n, energy, berry, bcd, qmd);
            }
        }

        // ここには到達しないはずだが、念のため最初の要素を返す
        sorted_data[0]
    }
    pub fn merge(&mut self, other: &Tanzakus) {
        if self.data.len() != other.data.len() {
            panic!("Tanzakusのデータ長が一致しません");
        }


        for i in 0..self.data.len() {
            self.data[i].n += other.data[i].n;
            self.data[i].berry += other.data[i].berry;
            self.data[i].bcd += other.data[i].bcd;
            self.data[i].qmd += other.data[i].qmd;
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
