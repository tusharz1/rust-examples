use std::iter::*;

fn is_prime(n: i32) -> bool {
    for i in 2..=(n.isqrt()) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn add_one(arr: &[i32]) {
    let y = arr.into_iter().map(|x| x + 1);
    println!("{:?}", y);
}

fn lazy_prime_gen() {
    let mut x = (1..).into_iter().filter(|&x| is_prime(x));
    println!("{}", x.next().unwrap());
    println!("{}", x.next().unwrap());
    println!("{}", x.next().unwrap());
}

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    println!("Hello world {:?}", arr);
    lazy_prime_gen();
    add_one(arr.as_slice());

    let strs : [String; 2] = ["hello".to_string(), "world".to_string()];
    let str_lengths = strs.each_ref().map(|x| x.len());
    println!("{:?}", str_lengths);
    let mut arr2 = arr.clone();
    println!("arr2 {:?}", arr2);
    arr2[0] = 23;
    println!("arr2 {:?}", arr2);
    println!("arr {:?}", arr);
}
