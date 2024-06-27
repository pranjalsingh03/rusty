fn main() {
    let x: i8 =5;  //i8 can store value from -128 to 127

    // Simailarly it goes upto i128 it is only use to define the size the the memory which the variable store in the memory
    
    let y: u8 =10;  //this is for the unsigned integer it cannot store negative values
    
    // Similary it also goes till u128
    
    //Basically it is umutable by defalult , We write mut to make it mutable 
    // In mutable we can update or change the value after the declaration.
    // And in unmutable we cannot change the value assigned to the variable
    
    let mut z: i8 = 20;
    
    println!("Value of x inital decleration is: {}",x);
    println!("Value of y inital decleration is: {}",y);
    println!("Value of z inital decleration is: {}",z);

    z= 25; // in this the value of z is update to 25 because we have make that mutable while decalaration of the varible.
    
    println!("Value of z are changing the value throug mut is: {}",z);

    let a: f32 = -1323.0331;  //this is for storing the floating number . It can store both positve and negative number also

    println!("Value of a floating number is: {}",a);

    const SECONDS_IN_MIN: u32 =60;
    //IN rust the naming of contants are always in capital letters
    
    //Constants are not only inmutable by default. But they are always inmutable.

    println!("Value of the constant is: {}",SECONDS_IN_MIN);

    //Shadowing

    //Previously x was declare as 5 so we are going to use that same in shadowing also
    let x = x + 2;  //increment of x by 2
    {
        let x = x * 2; //value of the x is updated and stored in x will be accessed by this bolck only the value can not be accessed form the outside.
        println!("Value of x in the inner block is: {}",x);
    }
    println!("Value of x outside the block is: {}",x);

    //isize and usize type variable size depends upon the computer architecture which you are working on.

    //64 bits if your computer is 64-bit architecture and 32 if your computer is 32-bit architecture.

    // in the declaration of any number we can also use "_" between them to make them easier to read.
    //For example: 1_000 is same as 1000.

    let ex: isize = 10_000;
    println!("Value of ex is: {}",ex);  //output will be 10000 only.

    //Numeric operators

    let sum = 1+2;

    let difference = 345.2 - 34.8;

    let multiplicaton = 3 * 9;

    let division = 25/5;

    let remainder = 21 % 4;

    println!("Sum of two numbers is: {}",sum);
    println!("Difference of two numbers is: {}",difference);
    println!("Multiplication of two numbers is: {}",multiplicaton);
    println!("Division of two numbers is: {}",division);
    println!("Remainder of two numbers is: {}",remainder);

    //Boolean 

    let t:bool = true;
    let f:bool = false; //with explict type annotation

    //the above declaration will give waring

    // Character type

    let ch: char = 'a';
    let emoji = '😻';
    println!("Value of ch is: {}",ch);
    println!("Value of emoji is: {}",emoji);

}
