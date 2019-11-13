#[derive(PartialEq,Debug)]
pub struct USD (i32); // this makes the struct a tuple whereas the value would be at position .0
#[derive(PartialEq,Debug)]
pub struct GBP (i32);
#[derive(PartialEq,Debug)]
pub struct CAD (i32);


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
