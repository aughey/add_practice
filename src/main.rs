use rand::{self, Rng};

fn main() {
    loop {
        make_practice(3);
    }
}

fn make_practice(count: usize) {
    let mut rng = rand::thread_rng();
    let numbers = (0..count).map(|_| rng.gen_range(0..=99)).collect::<Vec<usize>>();
    // join numbers with " + "
    println!("{}", numbers.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" + "));
    let sum = numbers.into_iter().sum::<usize>();

    loop {
        // read a value from the console
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if let Ok(value) = input.parse::<usize>() {
            if value == sum {
                println!("Correct!");
                break;
            } else {
                println!("Wrong! Try again.");
            }
        } else {
            println!("Please input a number ({}).",input);
        }
    }
}
