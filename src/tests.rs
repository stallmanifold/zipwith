use zipwith::ZipWith;
use std::iter::Iterator;
use std::ops::Fn;
use std::collections::linked_list::LinkedList;


#[test]
fn test_zip_with() {
    let add = |x: usize, y: usize| { x + y as usize };
    let vec1: Vec<usize> = vec![0x01; 16];
    let vec2: Vec<usize> = vec![0x02; 16];
    let zipper   = ZipWith::<&Fn(usize,usize)->usize,Vec<usize>,Vec<usize>,usize>::zip_with(&add, vec1, vec2);
    let result   = zipper.collect::<Vec<usize>>();
    let expected = vec![0x03; 16];

    assert_eq!(expected, result); 
}

#[test]
fn test_zip_with_output_length_should_be_identical_to_shortest_vector() {
    let add = |x: usize, y: usize| { x + y as usize };
    let vec1: Vec<usize> = vec![0x01; 16];
    let vec2: Vec<usize> = vec![0x02; 20];
    let zipper   = ZipWith::<&Fn(usize,usize)->usize,Vec<usize>,Vec<usize>,usize>::zip_with(&add, vec1, vec2);
    let result   = zipper.collect::<Vec<usize>>();
    let expected = vec![0x03; 16];

    assert_eq!(expected.len(), result.len());
}

#[test]
fn test_zip_with_having_one_empty_input() {
    let add              = |x: usize, y: usize| { x + y as usize };
    let vec1: Vec<usize> = vec![];
    let vec2: Vec<usize> = vec![0x02; 16];
    let zipper           = ZipWith::<&Fn(usize,usize)->usize,Vec<usize>,Vec<usize>,usize>::zip_with(&add, vec1, vec2);
    let result           = zipper.collect::<Vec<usize>>();
    let expected: Vec<usize> = vec![];

    assert_eq!(expected.len(), result.len());
    assert_eq!(expected, result);
}

#[test]
fn test_zip_with_having_two_empty_inputs() {
    let add              = |x: usize, y: usize| { x + y as usize };
    let vec1: Vec<usize> = vec![];
    let vec2: Vec<usize> = vec![];
    let zipper           = ZipWith::<&Fn(usize,usize)->usize,Vec<usize>,Vec<usize>,usize>::zip_with(&add, vec1, vec2);
    let result           = zipper.collect::<Vec<usize>>();

    assert_eq!(result.len(), 0);
}

#[test]
fn test_zip_with_iterator() {
    let xor              = |x: usize, y: usize| { x ^ y as usize };
    let vec1: Vec<usize> = vec![0xFF; 16];
    let vec2: Vec<usize> = vec![0xF0; 16];
    let zipper           = ZipWith::<&Fn(usize,usize)->usize,Vec<usize>,Vec<usize>,usize>::zip_with(&xor, vec1, vec2);

    for word in zipper {
        assert_eq!(word, 0x0F);
    } 
}

#[test]
fn test_zip_with_sized_iterator() {
    let xor              = |x: &usize, y: &usize| { *x ^ *y as usize };
    let vec1: Vec<usize> = vec![0xFF; 16];
    let vec2: Vec<usize> = vec![0xF0; 20];
    let slice1: &[usize] = vec1.as_slice();
    let slice2: &[usize] = vec2.as_slice();
    let zipper           = ZipWith::<&Fn(&usize,&usize)->usize,&[usize],&[usize],usize>::zip_with(&xor, slice1, slice2);
    
    assert_eq!(zipper.len(), vec1.len());
}

#[test]
fn test_zip_with_should_work_with_mixed_types() {
    let xor                         = |x: &usize, y: &usize| { *x ^ *y as usize };
    let vec: Vec<usize>             = vec![0xFF; 16];
    let mut list: LinkedList<usize> = LinkedList::new();

    for item in &vec { // Borrow vec instead of moving it.
        list.push_back(item.clone());
    }

    let slice: &[usize]      = vec.as_slice();
    let zipper = ZipWith::<&Fn(&usize,&usize)->usize,&[usize],&LinkedList<usize>,usize>::zip_with(&xor, slice, &list);
    let result: Vec<usize>   = zipper.collect::<Vec<usize>>();
    let expected: Vec<usize> = vec![0x00; 16];

    assert_eq!(result.len(), expected.len());
    assert_eq!(result, expected);

}
