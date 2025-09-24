use crate::honeycomb::{
        height_map::AllHeightMaps, 
        honeycomb_grids::Grids, 
        setting::CalcSetting,
        tanzaku::{Tanzakus,},
        util::{GridInfo,}
};

use crate::system::{model::System,};

use rayon::prelude::*;

pub fn parallel_calculate_tanzaku(
    calc_setting : CalcSetting,
    system : System,
) -> Tanzakus{

    let main_grid = calc_setting.main_mesh;

    //エネルギーの範囲を取得するための事前処理

    let grid_info = GridInfo::no_divide();
    let grids = Grids::build(calc_setting, system,grid_info);
    let energy_range = grids.energy_range();

    let final_tanzakus = (0..(main_grid * main_grid))
        .into_par_iter()
        // --- Mapフェーズ ---
        // 各スレッドは独立したTanzakusを計算して返す
        .map(|ij| {
            let i = ij % main_grid;
            let j = ij / main_grid;

            let grid_info = GridInfo::new_ijn(i, j, main_grid, main_grid, Some(energy_range));

            // ハニカム格子の構築
            let grids = Grids::build(calc_setting, system, grid_info);

            // 全バンドの等高線データを作成
            let all_height_maps = AllHeightMaps::build(&grids);

            // このスレッド専用のローカルなTanzakusを作成
            let mut partial_tanzakus = Tanzakus::new(calc_setting, system);
            partial_tanzakus.write_energy_n_bc_sum_to_tanzaku(&grids);
            partial_tanzakus.write_bcd_sum_to_tanzakus(&all_height_maps);

            // 計算結果を返す
            partial_tanzakus
        })
        // --- Reduceフェーズ ---
        // 2つのTanzakusを受け取り、1つにマージする
        .reduce(
            // 最初の要素がない場合の初期値を作成するクロージャ
            || Tanzakus::new(calc_setting, system),
            // 2つのTanzakus (t1, t2) をマージするクロージャ
            |mut t1, t2| {
                t1.merge(&t2);
                t1
            },
        );

    final_tanzakus
}