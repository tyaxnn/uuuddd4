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
    //--------------------------------------------------------------------
    Stable(Param),
    //--------------------------------------------------------------------
    FmKanemele(Param),
    One1Kanemele(Param),
    One2Kanemele(Param),
    TwinKanemele(Param),
    Tri1Kanemele(Param),
    Tri2Kanemele(Param),
    UuudddKanemele(Param),
    AfmKanemele(Param),
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
            //--------------------------------------------------------------------
            Self::Stable(_) => {panic!("Stable does not have a size");}
            //--------------------------------------------------------------------
            Self::FmKanemele(_) => {2},
            Self::One1Kanemele(_) => {6},
            Self::One2Kanemele(_) => {6},
            Self::TwinKanemele(_) => {6},
            Self::Tri1Kanemele(_) => {6},
            Self::Tri2Kanemele(_) => {6},
            Self::UuudddKanemele(_) => {6},
            Self::AfmKanemele(_) => {2},
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
            //--------------------------------------------------------------------
            Self::Stable(param) => param,
            Self::FmKanemele(param) => param,
            Self::One1Kanemele(param) => param,
            Self::One2Kanemele(param) => param,
            Self::TwinKanemele(param) => param,
            Self::Tri1Kanemele(param) => param,
            Self::Tri2Kanemele(param) => param,
            Self::UuudddKanemele(param) => param,
            Self::AfmKanemele(param) => param,
        }
    }
    pub fn tmd(&self) -> f64{
        match self{
            Self::Uuuddd(_) => 1.0,
            Self::Tmd(_) => 0.0,
            Self::Sato(_) => 1.0,
            //--------------------------------------------------------------------
            Self::FmTmd(_) => 0.0,
            Self::One1Tmd(_) => 0.0,
            Self::One2Tmd(_) => 0.0,
            Self::TwinTmd(_) => 0.0,
            Self::Tri1Tmd(_) => 0.0,
            Self::Tri2Tmd(_) => 0.0,
            Self::UuudddTmd(_) => 0.0,
            Self::SatoTmd(_) => 0.0,
            //--------------------------------------------------------------------
            Self::Stable(_) => 1.0,
            //--------------------------------------------------------------------
            Self::FmKanemele(_) => -1.0,
            Self::One1Kanemele(_) => -1.0,
            Self::One2Kanemele(_) => -1.0,
            Self::TwinKanemele(_) => -1.0,
            Self::Tri1Kanemele(_) => -1.0,
            Self::Tri2Kanemele(_) => -1.0,
            Self::UuudddKanemele(_) => -1.0, 
            Self::AfmKanemele(_) => -1.0,
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
            //--------------------------------------------------------------------
            Self::Stable(_) => "Stable",
            //--------------------------------------------------------------------
            Self::FmKanemele(_) => "FmKanemele",
            Self::One1Kanemele(_) => "One1Kanemele",
            Self::One2Kanemele(_) => "One2Kanemele",
            Self::TwinKanemele(_) => "TwinKanemele",
            Self::Tri1Kanemele(_) => "Tri1Kanemele",
            Self::Tri2Kanemele(_) => "Tri2Kanemele",
            Self::UuudddKanemele(_) => "UuudddKanemele",
            Self::AfmKanemele(_) => "AfmKanemele",
        };

        format!("{}_lambda{}_j{}", system_name, format!("{:.2}", self.param().lambda).replace('.', "p"), format!("{:.2}", self.param().jj).replace('.', "p"))
    }
    pub fn debug_only_name(&self) -> String{
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
            //--------------------------------------------------------------------
            Self::Stable(_) => "Stable",
            //--------------------------------------------------------------------
            Self::FmKanemele(_) => "FmKanemele",
            Self::One1Kanemele(_) => "One1Kanemele",
            Self::One2Kanemele(_) => "One2Kanemele",
            Self::TwinKanemele(_) => "TwinKanemele",
            Self::Tri1Kanemele(_) => "Tri1Kanemele",
            Self::Tri2Kanemele(_) => "Tri2Kanemele",
            Self::UuudddKanemele(_) => "UuudddKanemele",
            Self::AfmKanemele(_) => "AfmKanemele",
        };

        system_name.to_string()
    }
    pub fn spinseq(&self) -> SpinSeq6{
        match self {
            Self::UuudddTmd(_) | Self::Uuuddd(_) | Self::UuudddKanemele(_) => {
                SpinSeq6::uuuddd()
            }
            Self::One1Tmd(_) | Self::One1Kanemele(_) => {
                SpinSeq6::one1()
            }
            Self::One2Tmd(_) | Self::One2Kanemele(_) => {
                SpinSeq6::one2()
            }
            Self::TwinTmd(_) | Self::TwinKanemele(_) => {
                SpinSeq6::twin()
            }
            Self::Tri1Tmd(_) | Self::Tri1Kanemele(_) => {
                SpinSeq6::tri1()
            }
            Self::Tri2Tmd(_) | Self::Tri2Kanemele(_) => {
                SpinSeq6::tri2()
            }
            Self::SatoTmd(_) | Self::Sato(_) | Self::AfmKanemele(_) => {
                SpinSeq6::afm()
            }
            Self::FmTmd(_) | Self::FmKanemele(_) => {
                SpinSeq6::fm()
            }
            Self::Tmd(_) => {
                SpinSeq6::para()
            }
            Self::Stable(_) => {
                panic!("Stable does not have a spin sequence");
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