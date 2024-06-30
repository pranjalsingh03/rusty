use std::io;

fn main() {
    let a = 1;
    if a !=0{
        println!("The value of a is not equal to 0.");
    }
    else{
        println!("The value of a is equal to 0.");
    }

    //Writing the program to check the eligibiltiy of voting.

    println!("Enter the age.");
    
    let mut age = String::new();

    io::stdin().read_line(&mut age).expect("Failed to read line.");
    
    let age: i32 = age.trim().parse().expect("Age entered was not found.");
    
    if age >= 18{
        println!("You are eligible for voting.");
    }
    else{
        println!("You are not eligible for voting.");
    }

    let condition = true;
    //Using the if conditon in the let statemnet 
    let number = if condition {1} else {0};  // this line will check if condition is true then number will be 1 and if condition is false then it will be 0.

    println!("Value of number is: {number}");


    //Loops

    loop{
        println!("loop statement press ctrl+c to cancel.")
    }
}
