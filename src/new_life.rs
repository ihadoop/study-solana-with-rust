pub mod new_life{
    #[derive(Debug)]
struct Foo;

pub impl Foo {
    pub fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}


pub fn test(){
    
}

}