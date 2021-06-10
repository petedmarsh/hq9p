fn h() {
    println!("Hello, world!")
}

fn q(source: &str) {
    println!("{}", source)
}

fn _9() {
    for n in (2..99).rev() {
        println!("{} bottles of beer on the wall,", n);
        println!("{} bottles of beer.", n);
        println!("Take one down, pass it around,");
        println!("{} bottles of beer on the wall.", n);
        println!("")
    }
    println!("1 bottle of beer on the wall,");
    println!("1 bottle of beer.");
    println!("Take one down, pass it around,");
    println!("No more bottles of beer on the wall.");
}

fn p() {
    // do nothing
}

pub fn run(source: &str) {
    for c in source.chars() {
        match c {
            'H' => h(),
            'Q' => q(source),
            '9' => _9(),
            '+' => p(),
            // in any case do nothing
            _ => (),
        }
    }
}