use rand::Rng; //Use to generate the random number
use std::cmp::Ordering;  // Used for comparing two numbers
use std::io; //To take the input from the user 


fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(0..100); //thread will be give the particular random which we are going to use

    println!("The secret number is: {}", secret_number);

    //implemted loop for several checks
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); //input

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,  //to check the invalid inputs
                Err(_) => continue,
            }; //used trim to eliminate the white spaces

        println!("Your Guessed Number is {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
