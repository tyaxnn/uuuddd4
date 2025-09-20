use crate::consts::T;
use crate::system::spinseq::SpinSeq6;

#[derive(Debug, Clone, Copy)]
pub enum System{
    Uuuddd(Param),
    Sato(Param),
    Tmd(Param),    
    //--------------------------------------------------------------------
    FmTmd(Param),           //DDDDDD+TMD
    One1Tmd(Param),         //UDDDDD+TMD
    One2Tmd(Param),         //DUDDDD+TMD
    TwinTmd(Param),         //UUDDDD+TMD
    Tri1Tmd(Param),         //UDUDDD+TMD
    UuudddTmd(Param),       //UUUDDD+TMD
    Tri2Tmd(Param),         //DUDUDD+TMD
    SatoTmd(Param),         //UDUDUD+TMD
}

impl System{
    pub fn size(&self) -> usize{
        match self{
            Self::Uuuddd(_) => {6},
            Self::Tmd(_) => {2}
            Self::Sato(_) => {2}
            //--------------------------------------------------------------------
            Self::FmTmd(_) => {2}
            Self::One1Tmd(_) => {6}
            Self::One2Tmd(_) => {6}
            Self::TwinTmd(_) => {6}
            Self::Tri1Tmd(_) => {6}
            Self::Tri2Tmd(_) => {6}
            Self::UuudddTmd(_) => {6}
            Self::SatoTmd(_) => {2}
        }
    }
    pub fn param(&self) -> &Param{
        match self{
            Self::Uuuddd(param) => param,
            Self::Tmd(param) => param,
            Self::Sato(param) => param,
            //--------------------------------------------------------------------
            Self::FmTmd(param) => param,
            Self::One1Tmd(param) => param,
            Self::One2Tmd(param) => param,
            Self::TwinTmd(param) => param,
            Self::Tri1Tmd(param) => param,
            Self::Tri2Tmd(param) => param,
            Self::UuudddTmd(param) => param,
            Self::SatoTmd(param) => param,
        }
    }
    pub fn tmd(&self) -> bool{
        match self{
            Self::Uuuddd(_) => false,
            Self::Tmd(_) => true,
            Self::Sato(_) => false,
            //--------------------------------------------------------------------
            Self::FmTmd(_) => true,
            Self::One1Tmd(_) => true,
            Self::One2Tmd(_) => true,
            Self::TwinTmd(_) => true,
            Self::Tri1Tmd(_) => true,
            Self::Tri2Tmd(_) => true,
            Self::UuudddTmd(_) => true,
            Self::SatoTmd(_) => true,
        }
    }
    pub fn debug(&self) -> String{
        let system_name = match self {
            Self::Uuuddd(_) => "Uuuddd",
            Self::Tmd(_) => "Tmd",
            Self::Sato(_) => "Sato",
            //--------------------------------------------------------------------
            Self::FmTmd(_) => "FmTmd",
            Self::One1Tmd(_) => "One1Tmd",
            Self::One2Tmd(_) => "One2Tmd",
            Self::TwinTmd(_) => "TwinTmd",
            Self::Tri1Tmd(_) => "Tri1Tmd",
            Self::Tri2Tmd(_) => "Tri2Tmd",
            Self::UuudddTmd(_) => "UuudddTmd",
            Self::SatoTmd(_) => "SatoTmd",
        };

        format!("{}_lambda{}_j{}", system_name, self.param().lambda.to_string().replace('.', "p"), self.param().jj.to_string().replace('.', "p"))
    }
    pub fn spinseq(&self) -> SpinSeq6{
        match self {
            Self::UuudddTmd(_) | Self::Uuuddd(_)=> {
                SpinSeq6::uuuddd()
            }
            Self::One1Tmd(_) => {
                SpinSeq6::one1()
            }
            Self::One2Tmd(_) => {
                SpinSeq6::one2()
            }
            Self::TwinTmd(_) => {
                SpinSeq6::twin()
            }
            Self::Tri1Tmd(_) => {
                SpinSeq6::tri1()
            }
            Self::Tri2Tmd(_) => {
                SpinSeq6::tri2()
            }
            Self::SatoTmd(_) | Self::Sato(_) => {
                SpinSeq6::afm()
            }
            Self::FmTmd(_) => {
                SpinSeq6::fm()
            }
            Self::Tmd(_) => {
                SpinSeq6::para()
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Param{
    pub lambda : f64,
    pub jj : f64,
}

impl Param{
    pub fn new(lambda: f64, jj: f64) -> Self{
        Param { lambda, jj }
    }
    pub fn interesting() -> Self{
        let lambda = 0.3 * T;
        let jj = 0.25;

        Param { lambda, jj }
    }
}