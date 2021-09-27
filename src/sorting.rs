
#![allow(dead_code)] // do this to stop warnings
use std::borrow::BorrowMut;
pub fn mergesort(a: &mut [i32], len:usize)
{
    let mid = len/2;
    let (mut alow, mut ahigh) = split_array(&Vec::from(a.borrow_mut()), mid);
    domerge(&mut alow);
    domerge(&mut ahigh);
    let mut avec = Vec::from(a.borrow_mut());
    final_merge(&mut avec, &mut alow, &mut ahigh);
    for i in 0..avec.len()
    {
        a[i] = avec[i];
    }
}

fn domerge(a: &mut Vec<i32>)
{
    let len = a.len();
    if len<=2
    {
        if len==2 && a[0] > a[1]
        {
            swap_vec(a, 0, 1);
        }
        return;
    }
    else
    {
        let (mut alow, mut ahigh) = split_array(a, len/2);
        domerge(&mut alow);
        domerge( &mut ahigh);
        final_merge(a, &mut alow, &mut ahigh);
    }
}

fn final_merge(a: &mut Vec<i32>, alow: &mut Vec<i32>, ahigh: &mut Vec<i32>)
{
    ahigh.reverse();
    alow.reverse();
    
    let mut k = 0;
    while !alow.is_empty() && !ahigh.is_empty()
    {
        if alow.last().unwrap() <= ahigh.last().unwrap()
        {
            a[k] = alow.pop().unwrap();
        }
        else 
        {
            a[k] = ahigh.pop().unwrap();
        }
        k+=1;
    }

    while !alow.is_empty()
    {
        a[k] = alow.pop().unwrap();
        k+=1;
    }
    
    while !ahigh.is_empty()
    { 
        a[k] = ahigh.pop().unwrap();
        k+=1;
    }
}

fn split_array(a: &Vec<i32>, m: usize) -> (Vec<i32>, Vec<i32>)
{
    let (alow, ahigh) = a.split_at(m);
    (Vec::from(alow),Vec::from(ahigh))
}

pub fn bubblesort(a: &mut [i32], len:usize)
{
    let mut i:usize = 0;
    let mut sorted = true;
    loop 
    {
        if i == 0
        {
            sorted = true;
        }
        if a[i] > a[i+1]
        {
            sorted = false;
            swap_arr(a, i, i+1);
        } 
        i += 1;
        if i == len-1
        {
            i = 0;
        }
        if sorted{break;}
    }
}


pub fn quicksort(a: &mut [i32], lo: usize, hi: usize)
{
    if lo as i32 >= 0 && hi as i32 >= 0 && lo < hi
    {
        let p = partition(a, lo, hi);
        let phi =match p{
            0 => 0,
            _ => p-1   
        };
        let plo = p+1;
        quicksort(a, lo, phi);
        quicksort(a, plo, hi);
    }
}

//helper
fn partition(a: &mut [i32], lo:usize, hi:usize) -> usize
{
    let pivot = a[hi as usize];
    let mut p:i32 = lo as i32 - 1;
    for i in lo..hi+1
    {
        if a[i] <= pivot
        {
            p += 1;
            swap_arr(a, p as usize, i);
        }
    }
    p as usize
}

fn swap_arr(a:&mut [i32], i:usize, j:usize)
{
    let temp = a[i];
    a[i] = a[j];
    a[j] = temp;
}
fn swap_vec(a:&mut Vec<i32>, i:usize, j:usize)
{
    let temp = a[i];
    a[i] = a[j];
    a[j] = temp;
}

pub fn print_a(a: &[i32])
{
    for i in a.iter()
    {
        print!("{} ", i);
    }
    println!("");
}