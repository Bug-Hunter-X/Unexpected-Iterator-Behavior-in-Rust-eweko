fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    if let Some(val) = iter.next() { println!("{:?}", val); }
    if let Some(val) = iter.next() { println!("{:?}", val); }
    if let Some(val) = iter.next() { println!("{:?}", val); } else { println!("Iterator exhausted"); }
} 