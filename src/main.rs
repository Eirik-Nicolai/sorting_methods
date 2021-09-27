mod sorting;
mod trees;
use std::{borrow::BorrowMut, iter::Inspect, ptr::null, time::{self, Instant}};
use trees::Tree;

fn main() 
{
    let b = false;
    if b
    {
        let mut times = Vec::new();
    
        println!("-------- quicksort --------");
        let mut t = time::Instant::now();
        for _i in 0..100
        {
            let mut a = [5, 3, 6, 10, 7, 1, 8, 11, 4];
            let lo:usize = 0;
            let hi:usize = 9-1;
            sorting::print_a(&a);
            sorting::quicksort(&mut a, lo, hi);
            sorting::print_a(&a);
        }
        
        times.push(t.elapsed().as_millis());
        
        println!("");
        println!("-------- bubblesort --------");
        t = Instant::now();
        for _i in 0..100
        {
            let mut b = [5, 3, 6, 10, 7, 1, 8, 11, 4];    
            sorting::print_a(&b);
            sorting::bubblesort(&mut b, 9-1);
            sorting::print_a(&b);
        }
        times.push(t.elapsed().as_millis());
        
        println!("");
        println!("-------- mergesort --------");
        t = Instant::now();
        
        for _i in 0..100
        {
            let mut c = [5, 3, 6, 10, 7, 1, 8, 11, 4];    
            sorting::print_a(&c);
            sorting::mergesort(&mut c, 9);
            sorting::print_a(&c);
        }
        times.push(t.elapsed().as_millis());
    
        println!("");
        println!("Times taken:");
        println!("quicks: {}, bubblesort: {}, mergesort: {}", times[0], times[1], times[2]);
    }



    // ------------------------------
    // -- CONSTRUCTING BINARY TREE --
    // ------------------------------

    //                    1
    //                  /   \
    //                2       7
    //              /   \    / \
    //             3     6  8   9
    //           /   \
    //          4     5
    //

    let mut t:Tree = Tree::new(
        Tree::new(
            Tree::new(
                Tree::leaf(4, String::from("Dennis")), 
                Tree::leaf(5, String::from("Eirik")), 
                3,
                String::from("Carl")
            ), 
            Tree::leaf(6, String::from("Fredrik")), 
            2,
            String::from("Bjarne")
        ),
        Tree::new(
            Tree::leaf(8, String::from("Harry")), 
            Tree::leaf(9,String::from("Ida")), 
            7,
            String::from("Geir")
        ),
        1,
        String::from("Aleks")
    );
    
    //print!("inorder: ");
    //trees::inorder_traverse(&t, &printval);
    //print!("\npreorder: ");
    //trees::preorder_traverse(&t, &printval);
    //print!("\npostorder: ");
    //trees::postorder_traverse(&t, &printval);

    let val = 4;
    let mut found: Option<Tree> = None;
    let mut start = time::Instant::now();
    trees::depth_first_search(&t, val, found.borrow_mut());
    let end = start.elapsed();
    let end_mil = end.as_millis();
    let end_mic = end.as_micros();
    match found
    {
        Some(t) => println!("{} has value {}", t.name, val),
        None => println!("Couldn't find value {}", val)
    };
    println!("DFS took {} milliseconds ({} microseconds)", end_mil, end_mic);
}

fn printval(val:i32)
{
    print!("{} ", val);
}