use uuuddd4::{
    honeycomb::{
        compare::{compare_6_spinmodel, compare_6_spinmodel_kanemele}, height_map::{self, AllHeightMaps}, honeycomb_grids::Grids, parallelization::parallel_calculate_tanzaku, setting::CalcSetting, tanzaku::Tanzakus, util::GridInfo
    }, system::{self, model::Param}
};

use std::time::Instant;


fn main() -> std::io::Result<()> {
    //----------------------------------------------------------------------------------

    // // フェルミ面の等高線データを出力するコード例
    //     // 計算設定
    //     let calc_setting = CalcSetting{
    //         mesh_kx : 100,
    //         mesh_ky : 100,
    //         height_map_div : 39,   // 等高線の分割数
    //         threshold_berry : 1e-12, // Berry曲率計算の際の閾値
    //         main_mesh : 1,
    //     };

    //     let system = system::model::System::Uuuddd(Param::interesting());

    //     let grid_info = GridInfo::no_divide();

    //     let grids = Grids::build(calc_setting, system, grid_info);
    //     let height_map = AllHeightMaps::build(&grids);

    //     let _ = height_map.write_to_dat("./output/contour_lines_uuuddd.dat", 6);

    //----------------------------------------------------------------------------------

    // for i in 0..50 {
    //     let start = Instant::now();
    //     let jj = i as f64 * 0.01;

    //     compare_6_spinmodel(Param::new(0.3,jj),10);

    //     let duration = start.elapsed();

    //     println!("並列計算実行時間: {:?}, j = {}", duration,jj);
    // }

    //----------------------------------------------------------------------------------
    
    let system = system::model::System::UuudddKanemele(Param::interesting());
    let calc_setting = CalcSetting{
        mesh_kx : 100,
        mesh_ky : 100,
        height_map_div : 307,   // 等高線の分割数
        threshold_berry : 1e-12, // Berry曲率計算の際の閾値
        main_mesh : 10,
    };

    let tanzaku = parallel_calculate_tanzaku(calc_setting, system);

    tanzaku.interpolate_by_n(3000).write_to_dat(None,false)?;


    let tanzaku = parallel_calculate_tanzaku(calc_setting, system);

    tanzaku.interpolate_by_n(3000).write_to_dat(None,false)?;

    //----------------------------------------------------------------------------------

    // let start = Instant::now();

    // compare_6_spinmodel_kanemele(Param::new(0.1,0.1),1);

    // let duration = start.elapsed();

    // println!("並列計算実行時間: {:?}", duration);

    Ok(())
}