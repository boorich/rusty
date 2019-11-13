// traits allow to pass implementations on to other traits
// in this case it allows to convert any type to any type as long as they have a conversion to USD and the other has a from USD 

#[derive(PartialEq,Debug)]
pub struct USD (i32); // this makes the struct a tuple whereas the value would be at position .0
#[derive(PartialEq,Debug)]
pub struct GBP (i32);
#[derive(PartialEq,Debug)]
pub struct CAD (i32);

pub trait ToUSD {
    fn to_usd(&self) -> USD;                // in order to satisfy a trait, everything that ends with a semicolon needs to implemented
                                            // Why Trait method instead of directly implementing it to type?
    fn convert<T:FromUSD>(&self) -> T {     // convert() takes a generic T (type of any kind) that implemets traif FromUSD and a &self and returns whatever it took in
                                            // every time this gets called a separate function will be generated
        T::from_usd(&self.to_usd())
    }
}

/*
The impl keyword is primarily used to define implementations on types. Inherent implementations are
standalone, while trait implementations are used to implement traits for types, or other traits.

Functions and consts can both be defined in an implementation. A function defined in an impl block
can be standalone, meaning it would be called like Foo::bar(). If the function takes self, &self,
or &mut self as its first argument, it can also be called using method-call syntax, a familiar feature
to any object oriented programmer, like foo.bar().
*/ 

impl ToUSD for GBP {
    fn to_usd(&self)-> USD {
        USD((self.0 * 130) / 100) // this needs to happen in order to make a conversion so output can be USD
    }
} // as of here GBP will have GBP::ToUSD

pub trait FromUSD {
    fn from_usd(u: &USD) -> Self; // takes a ref to USD so it does not consume it and returns the type of the caller
}


impl FromUSD for CAD {
    fn from_usd(u: &USD) -> Self {
        CAD((u.0 * 130) / 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;  // this module is using everything from the module that contains it
    #[test]
    fn it_works() {
        let g = GBP(200);        // take 200 GBP
        let u = g.to_usd();      // convert them to USD using a ref 
        assert_eq!(u, USD(260)); // check if converting 200 GBP actually yields 260 USD

        let c = CAD::from_usd(&u);  // take the amount of USD inside u and convert to CAD
        assert_eq!(c, CAD(338));    // check if this is equal to 338 CAD

        let c2:CAD = g.convert();   // convert was given to g by implementing the trait
        assert_eq!(c2, c);       // is c2 equal to c
    }
}
