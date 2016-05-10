use zipwith::ZipWith;
use std::iter::Iterator;
use std::ops::Fn;


#[test]
fn test_zip_with() {
    let add = |x: usize, y: usize| { x+y as usize };
    let vec1: Vec<usize> = vec![0x01; 16];
    let vec2: Vec<usize> = vec![0x02; 16];
    let zipper   = ZipWith::<&Fn(usize,usize)->usize,Vec<usize>,Vec<usize>,Vec<usize>>::zip_with(&add, vec1, vec2);
    let result   = zipper.collect::<Vec<usize>>();
    let expected = vec![0x03; 16];

    assert_eq!(expected, result); 
}