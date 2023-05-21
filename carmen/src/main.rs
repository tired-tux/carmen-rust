use std::io;

pub fn main() {
    println!("Welcome to Carmen Scrachiageo in Rust");
    let q = ["Question 1:\nWhat place is known as the city of love?\n 1. Germany\n 2. France\n 3. Russia\n 4. New York\n Please type your answer:","Question 2:\nWhat place was Nintendo Founded?\n 1. Japan\n 2. China\n 3. Korea\n 4. Hong Kong\n Please type your answer:"];
    let a = [2, 1];

    for i in 0..2 {
        println!("{}", q[i]);

        let mut h: String = String::new();

        io::stdin().read_line(&mut h).expect("Failed to read line");

        let h: usize = h.trim().parse().expect("Failed to parse input");

        if h == a[i] {
            println!("Correct! Let's move on");
        } else {
            println!("Incorrect!");
        }
    }
}
