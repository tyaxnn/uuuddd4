use crate::system::model::{System,};
use nalgebra::{Complex, Const, SymmetricEigen, Vector2, DimMin, Dim, OMatrix, OVector};
use crate::system::hamiltonian::{self, HamiltonianEnum};

//----------------------------------------------------------------
// 対角化後の固有値、固有ベクトルを格納する構造体
//----------------------------------------------------------------
#[derive(Clone,Debug)]
pub struct SEud<const N: usize> {
    pub u: SymmetricEigen<Complex<f64>, Const<N>>,
    pub d: SymmetricEigen<Complex<f64>, Const<N>>,
}

impl<const N: usize> SEud<N>
where
    Const<N>: Dim + DimMin<Const<N>, Output = Const<N>>,
{
    pub fn new(
        u: SymmetricEigen<Complex<f64>, Const<N>>,
        d: SymmetricEigen<Complex<f64>, Const<N>>,
    ) -> Self {
        SEud { u, d }
    }

    pub fn sort(self) -> Self {
        SEud::new(
            sort_symmetric_eigen_ascending(self.u),
            sort_symmetric_eigen_ascending(self.d),
        )
    }

    pub fn eigenvalues(&self) -> Vec<f64> {
        let mut eigens = Vec::with_capacity(N * 2);
        eigens.extend_from_slice(self.u.eigenvalues.as_slice());
        eigens.extend_from_slice(self.d.eigenvalues.as_slice());
        eigens
    }
    pub fn index(&self, index : usize) -> &SymmetricEigen<Complex<f64>, Const<N>>{
        match index {
            0 => &self.u,
            1 => &self.d,
            _ => panic!("index should be 0 or 1"),
        }
    }
}

#[derive(Clone,Debug)]
pub enum SEudEnum{
    SEud2(SEud<2>),
    SEud6(SEud<6>),
}

impl SEudEnum{
    pub fn sort(self) -> Self{
        match self{
            SEudEnum::SEud2(seud) => {
                SEudEnum::SEud2(seud.sort())
            }
            SEudEnum::SEud6(seud) => {
                SEudEnum::SEud6(seud.sort())
            }
        }
    }
    pub fn is_2(&self) -> &SEud<2>{
        match self {
            SEudEnum::SEud2(seud) => seud,
            _ => panic!("is not 2")
        }
    }
    pub fn is_6(&self) -> &SEud<6>{
        match self {
            SEudEnum::SEud6(seud) => seud,
            _ => panic!("is not 6")
        }
    }
    pub fn get_cont(op_seud: &Option<Self>) -> &Self {
        match op_seud {
            Some(seudenum) => seudenum,
            None => panic!("no seud"),
        }
    }
}

//----------------------------------------------------------------
// 固有値の昇順に並び替える関数
//----------------------------------------------------------------

pub fn sort_symmetric_eigen_ascending<const N: usize>(
    eigen: SymmetricEigen<Complex<f64>, Const<N>>,
) -> SymmetricEigen<Complex<f64>, Const<N>>
where
    Const<N>: Dim + DimMin<Const<N>, Output = Const<N>>,
{
    let mut indexed_eigenvalues: Vec<(usize, f64)> = eigen
        .eigenvalues
        .iter()
        .enumerate()
        .map(|(i, &val)| (i, val))
        .collect();

    indexed_eigenvalues.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // サイズ N のベクトルと行列を作成
    let mut sorted_eigenvalues = OVector::<f64, Const<N>>::zeros();
    let mut sorted_eigenvectors = OMatrix::<Complex<f64>, Const<N>, Const<N>>::zeros();

    for (new_index, (old_index, _)) in indexed_eigenvalues.iter().enumerate() {
        sorted_eigenvalues[new_index] = eigen.eigenvalues[*old_index];
        sorted_eigenvectors.set_column(new_index, &eigen.eigenvectors.column(*old_index));
    }

    SymmetricEigen {
        eigenvalues: sorted_eigenvalues,
        eigenvectors: sorted_eigenvectors,
    }
}

//----------------------------------------------------------------
// 対角化
//----------------------------------------------------------------

pub fn diag(system: &System, kk: Vector2<f64>,force_6 : bool) -> SEudEnum {

    let hamiltonian_enum = hamiltonian::hamiltonian_from_system(system, kk,force_6);

    match hamiltonian_enum {
        HamiltonianEnum::H2(hamiltonian) => {

            let seud_u = SymmetricEigen::new(hamiltonian.u);
            let seud_d = SymmetricEigen::new(hamiltonian.d);

            let unsorted = SEud::new(seud_u, seud_d);

            SEudEnum::SEud2(unsorted.sort())
        }
        HamiltonianEnum::H6(hamiltonian) => {
            let seud_u = SymmetricEigen::new(hamiltonian.u);
            let seud_d = SymmetricEigen::new(hamiltonian.d);

            let unsorted = SEud::new(seud_u, seud_d);

            SEudEnum::SEud6(unsorted.sort())
        }
    }
}