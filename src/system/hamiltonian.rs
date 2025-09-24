use crate::{consts::*,};
use crate::system::model::{System,};
use nalgebra::{Complex, Const, Matrix2, Matrix6, Vector2, DimMin, Dim};

//----------------------------------------------------------------
// ハミルトニアンを格納する構造体
//----------------------------------------------------------------
#[derive(Clone,Debug)]
pub struct Hamiltonian<const N: usize>{
    pub u: nalgebra::Matrix<Complex<f64>, Const<N>, Const<N>, nalgebra::ArrayStorage<Complex<f64>, N, N>>,
    pub d: nalgebra::Matrix<Complex<f64>, Const<N>, Const<N>, nalgebra::ArrayStorage<Complex<f64>, N, N>>,
}

impl<const N: usize> Hamiltonian<N>
where
    Const<N>: Dim + DimMin<Const<N>, Output = Const<N>>,
{
    pub fn index(&self, index : usize) -> &nalgebra::Matrix<Complex<f64>, Const<N>, Const<N>, nalgebra::ArrayStorage<Complex<f64>, N, N>>{
        match index {
            0 => &self.u,
            1 => &self.d,
            _ => panic!("index should be 0 or 1"),
        }
    }
}

#[derive(Clone,Debug)]
pub enum HamiltonianEnum{
    H2(Hamiltonian<2>),
    H6(Hamiltonian<6>),
}

//----------------------------------------------------------------
// システムから対角化前のハミルトニアンを生成する関数
//----------------------------------------------------------------
pub fn hamiltonian_from_system(system: &System, kk: Vector2<f64>,force_6 : bool) -> HamiltonianEnum{
    match system.size(){
        2 => {
            if force_6{
                HamiltonianEnum::H6(hamiltonian_6(system, kk))
            }
            else{
                HamiltonianEnum::H2(hamiltonian_2(system, kk))
            }

        }
        6 => {
            HamiltonianEnum::H6(hamiltonian_6(system, kk))
        }
        _ => panic!("system size should be 2 or 6"),
    }
} 

//----------------------------------------------------------------
// 具体的なハミルトニアンの形はここで定義している
//----------------------------------------------------------------

//2x2 のハミルトニアン
pub fn hamiltonian_2(system : &System, kk : Vector2<f64>) -> Hamiltonian<2>{
    let param = system.param();
    let spin_seq = system.spinseq();
    let tmd = if system.tmd(){0.0}else{1.0};

    let lambda = param.lambda;
    let jj = param.jj;

    let diag = {
        2. * lambda * (
            kk.dot(&A1).sin() +
            kk.dot(&A2).sin() +
            kk.dot(&A3).sin() 
        )
    } * ONE;

    let off_diag = {
        Complex::exp( I * kk.dot(&D1)) +
        Complex::exp( I * kk.dot(&D2)) +
        Complex::exp( I * kk.dot(&D3))
    } * -T;

    let hamiltonian_u: nalgebra::Matrix<Complex<f64>, Const<2>, Const<2>, nalgebra::ArrayStorage<Complex<f64>, 2, 2>> = Matrix2::new(
        diag,off_diag,
        off_diag.conj(),diag * tmd
    ) + spin_seq.diag_matrix2(jj);
    let hamiltonian_d = Matrix2::new(
        -diag,off_diag,
        off_diag.conj(),-diag * tmd
    ) - spin_seq.diag_matrix2(jj);

    Hamiltonian{
        u: hamiltonian_u,
        d: hamiltonian_d,
    }
}

//6x6 のハミルトニアン
pub fn hamiltonian_6(system : &System, kk : Vector2<f64>) -> Hamiltonian<6>{
    let param = system.param();
    let spin_seq = system.spinseq();
    let tmd = if system.tmd(){0.0}else{1.0};

    let ed1p = Complex::exp( I * kk.dot(&D1)) * -T;
    let ed1m = Complex::exp(-I * kk.dot(&D1)) * -T;
    let ed2p = Complex::exp( I * kk.dot(&D2)) * -T;
    let ed2m = Complex::exp(-I * kk.dot(&D2)) * -T;
    let ed3p = Complex::exp( I * kk.dot(&D3)) * -T;
    let ed3m = Complex::exp(-I * kk.dot(&D3)) * -T;

    let lambda = param.lambda;
    let jj = param.jj;

    let plu = {
        Complex::exp( I * kk.dot(&A1)) +
        Complex::exp( I * kk.dot(&A2)) +
        Complex::exp( I * kk.dot(&A3))
    } * I * lambda;
    let mnu = {
        Complex::exp(-I * kk.dot(&A1)) +
        Complex::exp(-I * kk.dot(&A2)) +
        Complex::exp(-I * kk.dot(&A3))
    } * I * lambda;

    let (hamiltonian_u, hamiltonian_d) = {
        let (u_pre,d_pre) = hamiltonian_6_box(
            ed1p, ed1m,
            ed2p, ed2m,
            ed3p, ed3m,
            plu, mnu,
            tmd
        );

        let diag = spin_seq.diag_matrix6(jj);

        (
            u_pre + diag,
            d_pre - diag
        )
    };

    Hamiltonian{
        u: hamiltonian_u,
        d: hamiltonian_d,
    }

    
}

//2x2 pdv(H,k_x_i)
pub fn hamiltonian_2_dxi(system : &System, kk : Vector2<f64>, xindex : usize) -> Hamiltonian<2>{
    let param = system.param();
    let tmd = if system.tmd(){0.0}else{1.0};

    let lambda = param.lambda;

    let diag = {
        2. * lambda * (
            kk.dot(&A1).cos() * A1[xindex] +
            kk.dot(&A2).cos() * A2[xindex] +
            kk.dot(&A3).cos() * A3[xindex] 
        )
    } * ONE;

    let off_diag = {
        Complex::exp( I * kk.dot(&D1)) * I * D1[xindex] +
        Complex::exp( I * kk.dot(&D2)) * I * D2[xindex] +
        Complex::exp( I * kk.dot(&D3)) * I * D3[xindex]
    } * -T;

    let hamiltonian_u: nalgebra::Matrix<Complex<f64>, Const<2>, Const<2>, nalgebra::ArrayStorage<Complex<f64>, 2, 2>> = Matrix2::new(
        diag,off_diag,
        off_diag.conj(),diag * tmd
    ) ;
    let hamiltonian_d = Matrix2::new(
        -diag,off_diag,
        off_diag.conj(),-diag * tmd
    );

    Hamiltonian{
        u: hamiltonian_u,
        d: hamiltonian_d,
    }
}

//6x6 pdv(H,k_x_i)
pub fn hamiltonian_6_dxi(system : &System, kk : Vector2<f64>, xindex : usize) -> Hamiltonian<6>{
    let param = system.param();
    let tmd = if system.tmd(){0.0}else{1.0};    

    let ed1p = Complex::exp( I * kk.dot(&D1)) * -T;
    let ed1m = Complex::exp(-I * kk.dot(&D1)) * -T;
    let ed2p = Complex::exp( I * kk.dot(&D2)) * -T;
    let ed2m = Complex::exp(-I * kk.dot(&D2)) * -T;
    let ed3p = Complex::exp( I * kk.dot(&D3)) * -T;
    let ed3m = Complex::exp(-I * kk.dot(&D3)) * -T;

    let lambda = param.lambda;

    let pludx = {
        Complex::exp( I * kk.dot(&A1)) * I * A1[xindex] +
        Complex::exp( I * kk.dot(&A2)) * I * A2[xindex] +
        Complex::exp( I * kk.dot(&A3)) * I * A3[xindex]
    } * I * lambda;
    let mnudx = {
        Complex::exp(-I * kk.dot(&A1)) * -I * A1[xindex] +
        Complex::exp(-I * kk.dot(&A2)) * -I * A2[xindex] +
        Complex::exp(-I * kk.dot(&A3)) * -I * A3[xindex] 
    } * I * lambda;

    let (hamiltonian_u, hamiltonian_d) = 
    hamiltonian_6_box(
        ed1p * I * D1[xindex], ed1m * -I * D1[xindex],
        ed2p * I * D2[xindex], ed2m * -I * D2[xindex],
        ed3p * I * D3[xindex], ed3m * -I * D3[xindex],
        pludx, mnudx,
        tmd
    );

    Hamiltonian{
        u: hamiltonian_u,
        d: hamiltonian_d,
    }

    
}

//----------------------------------------------------------------
// 6x6 の重複部分を関数にしたもの
//----------------------------------------------------------------
fn hamiltonian_6_box(
    ed1p : Complex<f64>, ed1m : Complex<f64>,
    ed2p : Complex<f64>, ed2m : Complex<f64>, 
    ed3p : Complex<f64>, ed3m : Complex<f64>,
    plu  : Complex<f64>, mnu  : Complex<f64>,
    tmd  : f64
) -> (Matrix6<Complex<f64>>, Matrix6<Complex<f64>>){
    let hamiltonian_u = Matrix6::new(
        ZERO  ,ed1p,-plu*tmd,ed3p, mnu*tmd,ed2p,
        ed1m,ZERO  ,ed2m,-plu,ed3m, mnu,
        mnu*tmd,ed2p,ZERO ,ed1p,-plu*tmd, ed3p,
        ed3m, mnu,ed1m,ZERO  ,ed2m,-plu,
        -plu*tmd,ed3p, mnu*tmd,ed2p,ZERO  ,ed1p,
        ed2m,-plu,ed3m, mnu,ed1m,ZERO
    );
    let hamiltonian_d = Matrix6::new(
        ZERO  ,ed1p, plu*tmd,ed3p,-mnu*tmd,ed2p,
        ed1m,ZERO  ,ed2m, plu,ed3m,-mnu,
        -mnu*tmd,ed2p,ZERO  ,ed1p, plu*tmd, ed3p,
        ed3m,-mnu,ed1m,ZERO  ,ed2m, plu,
        plu*tmd,ed3p,-mnu*tmd,ed2p,ZERO  ,ed1p,
        ed2m, plu,ed3m,-mnu,ed1m,ZERO
    );
    (hamiltonian_u, hamiltonian_d)
}


