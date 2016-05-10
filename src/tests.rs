use zipwith::ZipWith;
use zipwith::ZipWithT;
use std::iter::Iterator;
use std::vec::{Vec, IntoIter};
use std::ops::Fn;

fn add(x: usize, y: usize) -> usize {
    x + y
}

#[test]
fn test_zip_with() {
    //let add = |x: usize, y: usize| { x+y };
    let vec1: Vec<usize> = vec![0x01; 16];
    let vec2: Vec<usize> = vec![0x02; 16];
    let zipper//: ZipWith<F, IntoIter<usize>, IntoIter<usize>, IntoIter<usize>> =  
        = ZipWithT::zip_with(add, vec1, vec2);
    //let result   = zipper.collect::<Vec<usize>>();
    let expected = vec![0x03; 16];

    //assert_eq!(expected, result); 
}