fn main() {
    let mut x = 5;
    { // Create a new scope to limit the mutable borrow
        let y = &mut x;
        *y = 10; 
    }
    let z = &mut x;
    *z = 15; 
} 