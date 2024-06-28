use std::io;

fn main() {
    // Collection of multiple values of same data-types 
    // Can store only values of same type , fixed length

    let a = [1,2,3,4];
    
    //the same can be declared in the different way for example: 
    let _b: [i32;5]=[1,2,3,4,5];   // i32 is type of element and after the semicolon number of elements.
    // using "_" of eliminate the warning
    
    // We can also initialise the arrays of same emelent using let a:[2;5] this array will contain 2 five times [2,2,2,2,2].
    
    // in this also positioning start from 0 position 
    println!("Value of first is: {}",a[0]);
    println!("Value of second is: {}",a[1]);

    // println!("Value of 6th is: {}",a[6]); // this will give error as the size of array is 4 and we are trying to access 6th element.

    println!("Enter the array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line.");

    let index: usize= index.trim().parse().expect("Index entered was not found.");

    let element = a[index];
    println!("The value at index {index} is: {element}");  // we can write this way also to print the element.

    // println!("The value at index {} is: {}",index,element);

    // The above code will throung the runtime error when we write the index out of range
}