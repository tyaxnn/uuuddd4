use std::vec;

use crate::{
    honeycomb::setting, system::{
        diag::SEudEnum, hamiltonian::{hamiltonian_2_dxi,hamiltonian_6_dxi}, model::System
    }
};

//テンソル成分
pub enum Tensor{
    XX,
    XY,
    YY,
}

use nalgebra::{Complex, ComplexField, Vector2, Vector6};
/// SEudEnumからspin,bandごとのBerry曲率を効率的に計算する関数
/// 
/// この関数は対角化結果（SEudEnum）を不変借用し、Kubo公式に基づいてBerry曲率を計算します。
/// 従来のcal_berry関数とは異なり、各k点での対角化直後に計算を行うため、
/// ハミルトニアンの微分計算が重複せず、より効率的です。
/// 
/// # Arguments
/// * `seud_enum` - 対角化結果（固有値・固有ベクトル）
/// * `system` - システム情報
/// * `kk` - k点座標
/// * `cell_area` - ブリルアンゾーンのセル面積
/// 
/// # Returns
/// * `Vec<Vec<f64>>` - [spin][band]の形でBerry曲率を格納
/// 
/// # 計算式
/// Berry曲率は以下のKubo公式で計算されます：
/// Ω_n(k) = -2 * Im[Σ_{m≠n} <u_n|∂H/∂kx|u_m><u_m|∂H/∂ky|u_n> / (ε_n - ε_m)²]
pub fn calculate_berry_curvature_from_seud(
    seud_enum: &SEudEnum,
    system: &System,
    kk: Vector2<f64>,
    cell_area: f64,
    setting : &setting::CalcSetting,
) -> Vec<Vec<f64>> {
    let size = system.size();
    let mut berry_results = vec![vec![0.0; size]; 2]; // [spin][band]
    
    match seud_enum {
        SEudEnum::SEud2(seud) => {
            // ハミルトニアンの微分を一度だけ計算
            let dhdx_all = hamiltonian_2_dxi(system, kk, 0);
            let dhdy_all = hamiltonian_2_dxi(system, kk, 1);
            
            for spin in 0..2 {
                let dhdx = dhdx_all.index(spin);
                let dhdy = dhdy_all.index(spin);
                
                // 固有ベクトルを事前に取得
                let eigenvectors: Vec<Vector2<Complex<f64>>> = (0..size)
                    .map(|i| seud.index(spin).eigenvectors.column(i).into())
                    .collect();
                
                let eigenvalues = &seud.index(spin).eigenvalues;
                
                for ei in 0..size {
                    let mut berry = 0.0;
                    let u_ei = &eigenvectors[ei];
                    let eps_i = eigenvalues[ei];
                    
                    for ej in 0..size {
                        if ei != ej {
                            let u_ej = &eigenvectors[ej];
                            let eps_j = eigenvalues[ej];
                            
                            // Kubo公式の計算
                            let braket = (u_ei.adjoint() * dhdx * u_ej)[(0,0)] * (u_ej.adjoint() * dhdy * u_ei)[(0,0)];
                            let bunshi = braket.imaginary() * -2.0;
                            let bunbo = (eps_i - eps_j).powi(2);
                            
                            // 分母が0に近い場合は寄与を無視（数値安定性のため）
                            if bunbo.abs() > setting.threshold_berry {
                                berry += bunshi / bunbo * cell_area;
                            }
                        }
                    }
                    
                    berry_results[spin][ei] = berry;
                }
            }
        }
        SEudEnum::SEud6(seud) => {
            // ハミルトニアンの微分を一度だけ計算
            let dhdx_all = hamiltonian_6_dxi(system, kk, 0);
            let dhdy_all = hamiltonian_6_dxi(system, kk, 1);
            
            for spin in 0..2 {
                let dhdx = dhdx_all.index(spin);
                let dhdy = dhdy_all.index(spin);
                
                // 固有ベクトルを事前に取得
                let eigenvectors: Vec<Vector6<Complex<f64>>> = (0..size)
                    .map(|i| seud.index(spin).eigenvectors.column(i).into())
                    .collect();
                
                let eigenvalues = &seud.index(spin).eigenvalues;
                
                for ei in 0..size {
                    let mut berry = 0.0;
                    let u_ei = &eigenvectors[ei];
                    let eps_i = eigenvalues[ei];
                    
                    for ej in 0..size {
                        if ei != ej {
                            let u_ej = &eigenvectors[ej];
                            let eps_j = eigenvalues[ej];
                            
                            // Kubo公式の計算
                            let braket = (u_ei.adjoint() * dhdx * u_ej)[(0,0)] * (u_ej.adjoint() * dhdy * u_ei)[(0,0)];
                            let bunshi = braket.imaginary() * -2.0;
                            let bunbo = (eps_i - eps_j).powi(2);
                            
                            // 分母が0に近い場合は寄与を無視（数値安定性のため）
                            if bunbo.abs() > setting.threshold_berry {
                                berry += bunshi / bunbo * cell_area;
                            }
                        }
                    }
                    
                    berry_results[spin][ei] = berry;
                }
            }
        }
    }
    
    berry_results
}

pub fn calculate_quantum_metric_from_seud(
    seud_enum: &SEudEnum,
    system: &System,
    kk: Vector2<f64>,
    cell_area: f64,
    is_berry_curvature : bool,
    tensor : Tensor,
    setting : &setting::CalcSetting,
) -> Vec<Vec<f64>> {
    let size = system.size();
    let mut berry_results = vec![vec![0.0; size]; 2]; // [spin][band]
    
    match seud_enum {
        SEudEnum::SEud2(seud) => {
            // ハミルトニアンの微分を一度だけ計算
            let dhdx_all = hamiltonian_2_dxi(system, kk, 0);
            let dhdy_all = hamiltonian_2_dxi(system, kk, 1);
            
            for spin in 0..2 {
                let dhdx = dhdx_all.index(spin);
                let dhdy = dhdy_all.index(spin);
                
                // 固有ベクトルを事前に取得
                let eigenvectors: Vec<Vector2<Complex<f64>>> = (0..size)
                    .map(|i| seud.index(spin).eigenvectors.column(i).into())
                    .collect();
                
                let eigenvalues = &seud.index(spin).eigenvalues;
                
                for ei in 0..size {
                    let mut berry = 0.0;
                    let u_ei = &eigenvectors[ei];
                    let eps_i = eigenvalues[ei];
                    
                    for ej in 0..size {
                        if ei != ej {
                            let u_ej = &eigenvectors[ej];
                            let eps_j = eigenvalues[ej];
                            
                            // Kubo公式の計算
                            let braket = match tensor{
                                Tensor::XX => (u_ei.adjoint() * dhdx * u_ej)[(0,0)] * (u_ej.adjoint() * dhdx * u_ei)[(0,0)],
                                Tensor::XY => (u_ei.adjoint() * dhdx * u_ej)[(0,0)] * (u_ej.adjoint() * dhdy * u_ei)[(0,0)],
                                Tensor::YY => (u_ei.adjoint() * dhdy * u_ej)[(0,0)] * (u_ej.adjoint() * dhdy * u_ei)[(0,0)],
                            };
                            let bunshi = if is_berry_curvature {
                                //ベリー曲率の場合は-2xIm[Gij]
                                braket.imaginary() * -2.0
                            } else {
                                //量子幾何計量の場合はRe[Gij]
                                braket.real()
                            };
                            let bunbo = (eps_i - eps_j).powi(2);
                            
                            // 分母が0に近い場合は寄与を無視（数値安定性のため）
                            if bunbo.abs() > setting.threshold_berry {
                                berry += bunshi / bunbo * cell_area;
                            }
                        }
                    }
                    
                    berry_results[spin][ei] = berry;
                }
            }
        }
        SEudEnum::SEud6(seud) => {
            // ハミルトニアンの微分を一度だけ計算
            let dhdx_all = hamiltonian_6_dxi(system, kk, 0);
            let dhdy_all = hamiltonian_6_dxi(system, kk, 1);
            
            for spin in 0..2 {
                let dhdx = dhdx_all.index(spin);
                let dhdy = dhdy_all.index(spin);
                
                // 固有ベクトルを事前に取得
                let eigenvectors: Vec<Vector6<Complex<f64>>> = (0..size)
                    .map(|i| seud.index(spin).eigenvectors.column(i).into())
                    .collect();
                
                let eigenvalues = &seud.index(spin).eigenvalues;
                
                for ei in 0..size {
                    let mut berry = 0.0;
                    let u_ei = &eigenvectors[ei];
                    let eps_i = eigenvalues[ei];
                    
                    for ej in 0..size {
                        if ei != ej {
                            let u_ej = &eigenvectors[ej];
                            let eps_j = eigenvalues[ej];
                            
                            // Kubo公式の計算
                            let braket = match tensor{
                                Tensor::XX => (u_ei.adjoint() * dhdx * u_ej)[(0,0)] * (u_ej.adjoint() * dhdx * u_ei)[(0,0)],
                                Tensor::XY => (u_ei.adjoint() * dhdx * u_ej)[(0,0)] * (u_ej.adjoint() * dhdy * u_ei)[(0,0)],
                                Tensor::YY => (u_ei.adjoint() * dhdy * u_ej)[(0,0)] * (u_ej.adjoint() * dhdy * u_ei)[(0,0)],
                            };
                            let bunshi = if is_berry_curvature {
                                //ベリー曲率の場合は-2xIm[Gij]
                                braket.imaginary() * -2.0
                            } else {
                                //量子幾何計量の場合はRe[Gij]
                                braket.real()
                            };
                            let bunbo = (eps_i - eps_j).powi(2);

                            // 分母が0に近い場合は寄与を無視（数値安定性のため）
                            if bunbo.abs() > setting.threshold_berry {
                                berry += bunshi / bunbo * cell_area;
                            }
                        }
                    }
                    
                    berry_results[spin][ei] = berry;
                }
            }
        }
    }
    
    berry_results
}

//ある点でのあるバンドの異常速度を計算する関数
pub fn cal_anomaly_velocity(
    seud_enum: &SEudEnum,
    system: &System,
    kk: Vector2<f64>,
    spin : usize,
    band_num : usize,
) -> Vector2<f64> {
    match seud_enum {
        SEudEnum::SEud2(seud) => {
            let dhdx_all = hamiltonian_2_dxi(system, kk, 0);
            let dhdy_all = hamiltonian_2_dxi(system, kk, 1);

            let dhdx = dhdx_all.index(spin);
            let dhdy = dhdy_all.index(spin);

            let u_ei = &seud.index(spin).eigenvectors.column(band_num);

            let av_x = u_ei.adjoint() * dhdx * u_ei;
            let av_y = u_ei.adjoint() * dhdy * u_ei;
            
            Vector2::new(av_x[(0,0)].real(), av_y[(0,0)].real())
        }
        SEudEnum::SEud6(seud) => {
            let dhdx_all = hamiltonian_6_dxi(system, kk, 0);
            let dhdy_all = hamiltonian_6_dxi(system, kk, 1);

            let dhdx = dhdx_all.index(spin);
            let dhdy = dhdy_all.index(spin);

            let u_ei = &seud.index(spin).eigenvectors.column(band_num);

            let av_x = u_ei.adjoint() * dhdx * u_ei;
            let av_y = u_ei.adjoint() * dhdy * u_ei;
            
            Vector2::new(av_x[(0,0)].real(), av_y[(0,0)].real())
        }
    }

}