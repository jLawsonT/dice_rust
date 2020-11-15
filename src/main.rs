use std::io;
use rand::Rng;

fn randomNumber(num: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let num1 = rng.gen_range(0, num);

    num1  
}



fn main() {
    println!("How many sides does your dice have? ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: u32 = n.trim().parse().expect("invalid input");

    println!("How many dice would you like to roll? ");
    let mut n1 = String::new();
    io::stdin()
        .read_line(&mut n1)
        .expect("failed to read input.");
    let n1: u32 = n1.trim().parse().expect("invalid input");

    println!("Loaded- 1   or   unloaded- 2 ");
    let mut n2 = String::new();
    io::stdin()
        .read_line(&mut n2)
        .expect("failed to read input.");
    let n2: u32 = n2.trim().parse().expect("invalid input");

    if n2 == 1 {
        println!("What number would you like to load? ");
        let mut n3 = String::new();
        io::stdin()
            .read_line(&mut n3)
            .expect("failed to read input.");
        let n3: u32 = n3.trim().parse().expect("invalid input");

        for i in 0..n1 {
            if randomNumber(2) == 1 {
                println!("{:?}", n3);
            }
            else {
                let number = randomNumber(n+1);
                println!("{:?}", number);
            }
        }
    }
    else {
        for i in 0..n1 {
            let number = randomNumber(n+1);
            println!("{:?}", number);
        }
    }
}
