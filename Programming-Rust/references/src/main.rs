fn main() {
    let v = [10, 12, 1, 8, 7, 40, 30];
    println!("Smallest num: {:?}", smallest(&v));
}

fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}
