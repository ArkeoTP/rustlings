// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec0 : Vec<i32> = Vec::new();

    // 1 - copy outside
    // let mut vec0_copy : Vec<i32> = Vec::new();
    // for elem in &vec0 {
    //     vec0_copy.push(*elem);
    // }
    // let mut vec1 = fill_vec(vec0_copy);

    // 2 - copy inside
    //let mut vec1 = fill_vec_ref(&vec0);
    // copy outside
    //let mut vec1 = vec0;
    fill_vec_mut_ref(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    //vec1.push(88);
    vec0.push(88);

    //println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

// 1 - copy outside
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// 2 - copy inside
fn fill_vec_ref(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec2 = Vec::new();

    for elem in vec {
        vec2.push(*elem);
    }

    vec2.push(22);
    vec2.push(44);
    vec2.push(66);

    vec2
}

fn fill_vec_mut_ref(vec: &mut Vec<i32>) -> () {

    vec.push(22);
    vec.push(44);
    vec.push(66);
}