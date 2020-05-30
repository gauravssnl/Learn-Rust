// Lifetime Elision
// No need to specify lifettime annoation

fn first_word(s: &str) -> &str { 
// lifetime annoation : 
// 1.  input lifetimes : fn first_word(s: &'a str)
// 2. output lifetimes: fn first_word(s: &'a str) -> &'a str
// 3.&self or &mut self parameter check in method  =>  
// the lifetime of self is assigned to all output lifetime parameters

    let bytes = s.as_bytes();

    for(indx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..indx];
        }
    }
    &s
}