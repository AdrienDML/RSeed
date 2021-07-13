pub trait Asset {

    type D;

    fn name(&self) -> String;
    
    fn data(&self) -> &Self::D;
} 



