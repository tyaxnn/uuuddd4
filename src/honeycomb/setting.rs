#[derive(Clone, Copy)]
pub struct CalcSetting{
    pub mesh_kx : usize,
    pub mesh_ky : usize,
    pub height_map_div : usize,
    pub threshold_berry : f64,
    pub main_mesh : usize,
}

impl CalcSetting{
    pub fn meshes(&self) -> (usize,usize){
        (self.mesh_kx,self.mesh_ky)
    }
    pub fn debug(&self) -> String{
        format!("mesh_x{}_mesh_y{}_div{}_thresh10em{}_main_mesh{}", self.mesh_kx, self.mesh_ky, self.height_map_div, -self.threshold_berry.log10() as i32, self.main_mesh)
    }
}