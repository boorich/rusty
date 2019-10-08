// Vectors are re-sizable arrays. Like slices, their size is not known at compile time, but they can
// grow or shrink at any time. A vector is represented using 3 parameters:
// pointer to the data, length, capacity
// whereas the more memory get allocated when capacity threshold needs to be surpassed. 

fn main() {
    loop_over_vec()
}

// manual way to inititalize a vector
fn loop_over_vec() {
    let mut v = Vec::new();
    v.push(4);
    v.push(7);
    v.push(9);

    // macro to initialize a vector 
    let mut v = vec![3, 5, 8, 9, 0];

    v.push(57);
    v.push(39);
    v.push(138);
    //v.pop();  // would remove the last element

    // println!("The input vector is {:#?}", v);

    'outer_loop: for i in 0..10 {   // 'outer_loop is a lable that can be addressed at break
        for elem in &v {
            if elem % 2 == 0 {      // if even
               continue;            // carry on with next element
            }
            if *elem + i == 64 {    // if 64
                break 'outer_loop;  // leave loop immideately
            }
             println!("{} {}", i, elem);
        }
    // println!("{}", i);
    }
}
