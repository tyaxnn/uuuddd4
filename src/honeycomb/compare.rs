use crate::honeycomb::{
    parallelization::parallel_calculate_tanzaku, setting::CalcSetting, tanzaku::Tanzakus, util::GridInfo
};

use crate::system::{model::Param,model::System};

pub fn compare_6_spinmodel(param : Param) {
    // 計算設定
    let calc_setting = CalcSetting{
        mesh_kx : 400,
        mesh_ky : 400,
        height_map_div : 307,   // 等高線の分割数
        threshold_berry : 1e-12, // Berry曲率計算の際の閾値
        main_mesh : 8,
    };

    let systems = vec![
        System::FmTmd(param),
        System::One1Tmd(param),
        System::One2Tmd(param),
        System::TwinTmd(param),
        System::Tri1Tmd(param),
        System::UuudddTmd(param),
        System::Tri2Tmd(param),
        System::SatoTmd(param),
    ];

    let n_div = 300;

    let cal_e_vs_ns : Vec<Vec<f64>> = systems.iter().map(|system|{
        cal_e_vs_n(system, 100, n_div)
    }).collect();

    let tanzakuss : Vec<Tanzakus> = systems.iter().map(|system|{
        // ハニカム格子の構築
        let tanzakus = parallel_calculate_tanzaku(calc_setting, *system);
        tanzakus.interpolate_by_n(n_div)
    }).collect();

    let mut tanzakus_most_stable = Tanzakus::new(calc_setting,System::Stable(param));

    for i in 0..n_div{
        tanzakus_most_stable.data[i] = {
            let mut tanzaku = tanzakuss[0].data[i];
            let mut min_energy = cal_e_vs_ns[0][i];

            for j in 1..tanzakuss.len(){
                if cal_e_vs_ns[j][i] < min_energy{
                    tanzaku = tanzakuss[j].data[i];
                    min_energy = cal_e_vs_ns[j][i];
                }
            }
            tanzaku
        }
    }

    //出力
    let dir = "./out_tanzaku/compare_6_spinmodel".to_string();
    for tanzakus in tanzakuss{
        tanzakus.write_to_dat(Some(&dir)).unwrap();
    }
    tanzakus_most_stable.write_to_dat(Some(&dir)).unwrap();
}

use crate::{
    honeycomb::util::i_j_to_kk,
    system::diag::diag
};


pub fn cal_e_vs_n(system : &System, graph_mesh: usize, n_div: usize) -> Vec<f64> {
    let mesh_kx = graph_mesh;
    let mesh_ky = graph_mesh;

    let mut all_eigens = Vec::new();

    for i in 0..mesh_kx {
        for j in 0..mesh_ky {
            let kk = i_j_to_kk(i, j, mesh_kx, mesh_ky, false, system.size(),GridInfo::no_divide());
            let eigens = diag(system,kk,true).is_6().eigenvalues();
            all_eigens.extend_from_slice(&eigens);
        }
    }

    all_eigens.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let total_kpoints = mesh_kx * mesh_ky;
    let total_states = all_eigens.len();
    let bands_per_k = total_states / total_kpoints;

    assert_eq!(
        total_states,
        total_kpoints * bands_per_k,
        "固有値数とバンド数が一致しません"
    );

    let mut energy_vs_n = Vec::new();

    for step in 0..=n_div {
        let n = step as f64 / n_div as f64 * 2.0; // nは0から2まで変化

        // 電子数に対応する占有状態数（スピン縮重なしの場合）
        let num_occupied_states = ((n * total_states as f64 * 0.5).round()) as usize;

        let energy_sum: f64 = all_eigens
            .iter()
            .take(num_occupied_states)
            .sum();

        energy_vs_n.push(energy_sum / mesh_kx as f64 / mesh_ky as f64);
    }

    energy_vs_n
}