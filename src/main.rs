use uuuddd4::{
    honeycomb::{
        height_map::{AllHeightMaps,}, 
        honeycomb_grids::{Grids,}, 
        setting::CalcSetting,
        tanzaku::Tanzakus,
        compare::{compare_6_spinmodel},
        parallelization::{parallel_calculate_tanzaku,}
    }, system::{self, model::Param}
};

use std::time::Instant;


fn main() -> std::io::Result<()> {
    // 計算設定
    let calc_setting = CalcSetting{
        mesh_kx : 400,
        mesh_ky : 400,
        height_map_div : 301,   // 等高線の分割数
        threshold_berry : 1e-12, // Berry曲率計算の際の閾値
        main_mesh : 35,
    };

    let system = system::model::System::Tri1Tmd(Param::interesting());

    let start = Instant::now();

    let tanzaku = parallel_calculate_tanzaku(calc_setting, system);

    let duration = start.elapsed();

    println!("並列計算実行時間: {:?}", duration);

    tanzaku.interpolate_by_n(3000).write_to_dat(None)?;

    // compare_6_spinmodel(Param::new(0.3,0.1));
    Ok(())
}