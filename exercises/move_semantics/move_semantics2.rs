// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    //Type must be defined before being called
    
    
    /* 
        * Solution 1: Make a seperate version of 'vec0' to pass to 'fill_vec'
            let vec0 = Vec::<i32>::new();
            
            * Solution 2: Make `fill_vec` borrow its argument instead of taking ownership
            
            
            
            
    */
    let vec0 = Vec::<i32>::new();
    let temp_vec0 = Vec::new();

    let mut vec1 = fill_vec(temp_vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

//Solution 1:
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// Solution 2
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }
