#[allow(unused_imports)]
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

extern crate graphx;
use graphx::graph_x;

fn main() {
    let str = "SSSSSSS-dddd dfdfd";
    println!("Hello, world! {}", str);

    let xs = [1, 2, 3, 4];

    {xs[0] + 1};

    analyze_slice(&xs);   
    
    graph_x::create_graph();
    
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("value is {}", guess);

    let a = {
        let y = [2, 3, 4];
        y
    };

    println!("value is {:?}", a);
    
    let cnt = 18;

    match cnt{
        n @ 1 ... 12 => println!("between 1 and 12 {}", n),
        n @ 13 ... 23 => println!("between 13 and 23 {}", n),
        _ => println!("other")
    }

    let some_val = Some(3);
    match some_val{
        Some(x) => println!("{}", x),
        _ => println!("not found")
    }

}

#[allow(dead_code)]
fn recursion(x: i128) -> i128{
    println!("input value {}", x);
    if x == 0{
        return x;
    } else {
        recursion(x -1)
    }     
}