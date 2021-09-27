#![allow(dead_code)]

use std::{borrow::BorrowMut, collections::{HashMap, HashSet}}; //do this to stop warnings
#[derive(Hash, PartialEq, Eq,Clone)]
pub struct Tree
{
    pub name: String,
    pub val: i32,
    l: Option<Box<Tree>>,
    r: Option<Box<Tree>>
}
impl Tree {
    pub fn new(l: Tree, r: Tree, val:i32, name:String) -> Tree
    {
        Tree{l:Some(Box::new(l)),r:Some(Box::new(r)),val:val, name:name}
    }
    pub fn leaf(val: i32, name:String) -> Tree
    {
        Tree{l:None,r:None, val:val, name:name}
    }
} 

pub fn inorder_traverse(t:&Tree, f: &dyn Fn(i32))
{
    match &t.l 
    {
        Some(l) => inorder_traverse(&*l, f),
        _ => {}
    }
    f(t.val);
    match &t.r 
    {
        Some(r) => inorder_traverse(&*r, f),
        _ => {}
    }
}

pub fn preorder_traverse(t:&Tree, f: &dyn Fn(i32))
{
    f(t.val);
    match &t.l 
    {
        Some(l) => inorder_traverse(&*l, f),
        _ => {}
    }
    match &t.r 
    {
        Some(r) => inorder_traverse(&*r, f),
        _ => {}
    }
}

pub fn postorder_traverse(t:&Tree, f: &dyn Fn(i32))
{
    match &t.l 
    {
        Some(l) => inorder_traverse(&*l, f),
        _ => {}
    }
    match &t.r 
    {
        Some(r) => inorder_traverse(&*r, f),
        _ => {}
    }
    f(t.val);
}

pub fn depth_first_search(t:&Tree, searched_val:i32, found:&mut Option<Tree>)
{
    let mut hset:HashSet<Tree> = HashSet::new();
    dfs_util(t,searched_val,hset.borrow_mut(), found.borrow_mut());
}

fn dfs_util(t:&Tree, searched_val:i32, visited:&mut HashSet<Tree>, found:&mut Option<Tree>)
{
    println!("Checking {} for value {}...", t.name, searched_val);
    if t.val == searched_val
    {
        *found = Some(t.clone());
        return;
    }
    match &t.l 
    {
        Some(l) => 
        {
            if !visited.contains(&*l) && found.is_none()
            {
                visited.insert(*l.to_owned());
                dfs_util(&*l, searched_val,visited.borrow_mut(), found);
            }
        }
        _ => {}
    }
    match &t.r
    {
        Some(r) => 
        {
            if !visited.contains(&*r) && found.is_none()
            {
                visited.insert(*r.to_owned());
                dfs_util(&*r, searched_val,visited.borrow_mut(),found);
            }
        }
        _ => {}
    }

}