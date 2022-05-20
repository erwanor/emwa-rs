use emwa_rs::*;

fn main() {
    println!("this is a test");
    let mut risk = EMWA::new(1 as f64, Alpha::Static);
    for i in 1..100 {
        risk.add(i as f64).unwrap();
    }

    println!("value: {}", risk.value())
}
