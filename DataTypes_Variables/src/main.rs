fn main() {
    let x: i8 =5;  //i8 can store value from -128 to 127

    // Simailarly it goes upto i128 it is only use to define the size the the memory which the variable store in the memory
    
    let y: u8 =10;  //this is for the unsigned integer it cannot store negative values
    
    // Similary it also goes till u128
    
    //Basically it is umutable by defalult , We write mut to make it mutable 
    // In mutable we can update or change the value after the declaration.
    // And in unmutable we cannot change the value assigned to the variable
    
    let mut z: i8 = 20;
    
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);

    z= 25; // in this the value of z is update to 25 because we have make that mutable while decalaration of the varible.
    
    println!("{}",z);

    let a: f32 = -1323.0331;  //this is for storing the floating number . It can store both positve and negative number also

    println!("{}",a);
}
