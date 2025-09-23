use uuuddd4::{
    honeycomb::{
        height_map::{AllHeightMaps,}, 
        honeycomb_grids::{Grids,}, 
        setting::CalcSetting,
        tanzaku::Tanzakus,
    }, system::{self, model::Param}
};


fn main() -> std::io::Result<()> {
    // 計算設定
    let calc_setting = CalcSetting{
        mesh_kx : 2000,
        mesh_ky : 2000,
        height_map_div : 300,   // 等高線の分割数
        threshold_berry : 1e-12, // Berry曲率計算の際の閾値
    };

    let system = system::model::System::UuudddTmd(Param::interesting());

    // ハニカム格子の構築
    let grids = Grids::build(calc_setting, system);

    // 全バンドの等高線データを作成
    let all_height_maps = AllHeightMaps::build(&grids);
    


    let tanzaku = {
        let mut tanzakus = Tanzakus::new(calc_setting,system);
        tanzakus.write_energy_n_bc_sum_to_tanzaku(&grids);
        tanzakus.write_bcd_sum_to_tanzakus(&all_height_maps);
        tanzakus
    };

    tanzaku.write_to_dat()?;
    Ok(())
}