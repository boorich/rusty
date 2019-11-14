#[derive(PartialEq,Debug)]
pub struct USD (i32); // this makes the struct a tuple whereas the value would be at position .0
#[derive(PartialEq,Debug)]
pub struct GBP (i32);
#[derive(PartialEq,Debug)]
pub struct CAD (i32);

// traits are rules that allow types to jointly use functions
pub trait ToUSDv<F> { // this trait is not impl'ed on a currency but on an exchange, <F> needs to exist
    fn to_uv(&self, f:F) -> f32; // &self is the exchange, F is a generic type for a currency
                                 // Returns a float
}

pub trait FromUSDv<F> {
    fn from_uv(&self, f:f32) -> F;
}

pub struct Ex {
    cad:f32,
    gbp:f32,
}

// impls are the actual functions that a trait implements (careful if you do this on generic types)
impl ToUSDv<GBP> for Ex { // here we add the implementation for GBP-type objects inside the scope of Ex
    fn to_uv(&self, g:GBP) -> f32 { 
        (g.0 as f32) * self.gbp
    }
}

impl FromUSDv<CAD> for Ex {
    fn from_uv(&self, f:f32) -> CAD {
        CAD((f / self.cad) as i32)
    }
}

pub trait Exchange<F,T> { // generic F from and T to types
    fn convert(&self, f:F) -> T;
}

// We know this is applied to Ex-type since Ex type impl's ToUSDv for GBP and FromUSDv for CAD and as a result Ex implemets
// Exchange on GBP -> CAD
impl<E,F,T> Exchange<F,T> for E // Exchange first needs all variables to exist <E,F,T> so i can be implemented for E
    where E:ToUSDv<F> + FromUSDv<T> { // here we limit the allowed types to the ones that implement required traits
    fn convert(&self, f:F)->T { // and implement the function to convert
        self.from_uv(self.to_uv(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = GBP(200);
        let ex = Ex{cad:0.7, gbp:1.3};
        // let c = ex.from_uv(ex.to_uv(g));
        let c:CAD = ex.convert(g); // currently there is only CAD implemented, so it would infer that 
        assert_eq!(c, CAD(371));
    }
}
