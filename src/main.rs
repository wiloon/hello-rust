use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use std::time::Duration;
use std::thread;

fn main() {
    println!("Hello, world!");

    let mut number = 1;
    while number != 3 {
        println!("{}", number);
        number += 1;
        thread::sleep(Duration::from_millis(1000));
    }
    println!("exit");

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();


}
