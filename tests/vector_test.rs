// TODO: write test for out of memory
use algorithms_and_data_structures::vector::Vector;

#[test]
fn create_empty_vector() {
    let array: [i32; 0] = [];
    let vec = Vector::new(&array);

    assert!(vec.is_empty());
    assert_eq!(vec.size(), 0);
}

#[test]
fn create_non_empty_vector_i32() {
    let array: [i32; 4] = [1, 2, 3, 4];
    let vec = Vector::new(&array);

    assert!(!vec.is_empty());
    assert_eq!(vec.size(), 4);
}

#[test]
fn create_non_empty_vector_bool() {
    let array: [bool; 4] = [false, true, true, false];
    let vec = Vector::new(&array);

    assert!(!vec.is_empty());
    assert_eq!(vec.size(), 4);
}

#[test]
fn get_capacity_empty() {
    let array_1: [i32; 0] = [];

    let vec_1 = Vector::new(&array_1);

    assert_eq!(vec_1.capacity(), 1);
}

#[test]
fn get_capacity_1() {
    let array_1: [i32; 1] = [0];
    let array_2: [i32; 2] = [0; 2];

    let vec_1 = Vector::new(&array_1);
    let vec_2 = Vector::new(&array_2);

    assert_eq!(vec_1.capacity(), 1);
    assert_ne!(vec_2.capacity(), 1);
}

#[test]
fn get_capacity_32() {
    let array_1: [i32; 17] = [0; 17];
    let array_2: [i32; 32] = [0; 32];
    let array_3: [i32; 33] = [0; 33];

    let vec_1 = Vector::new(&array_1);
    let vec_2 = Vector::new(&array_2);
    let vec_3 = Vector::new(&array_3);

    assert_eq!(vec_1.capacity(), 32);
    assert_eq!(vec_2.capacity(), 32);
    assert_ne!(vec_3.capacity(), 32);
}

#[test]
fn at_index() {
    let array: [i32; 4] = [2, 4, 1, 3];

    let vec = Vector::new(&array);

    assert_eq!(vec.at(0), Ok(2));
    assert_eq!(vec.at(1), Ok(4));
    assert_eq!(vec.at(2), Ok(1));
    assert_eq!(vec.at(3), Ok(3));
}

#[test]
fn at_index_out_of_range() {
    let array: [i32; 4] = [2, 4, 1, 3];

    let vec = Vector::new(&array);

    assert!(vec.at(4).is_err());
}

#[test]
fn push_item() {
    let array: [i32; 4] = [0; 4];

    let mut vec = Vector::new(&array);
    vec.push(1);

    assert_eq!(vec.size(), 5);
    assert_eq!(vec.at(4), Ok(1));
}

#[test]
fn push_item_empty_vec() {
    let array: [i32; 0] = [];

    let mut vec = Vector::new(&array);
    vec.push(1);

    assert!(!vec.is_empty());
    assert_eq!(vec.size(), 1);
    assert_eq!(vec.at(0), Ok(1));
}

#[test]
fn push_item_and_increase_capacity() {
    let array: [i32; 16] = [0; 16];

    let mut vec = Vector::new(&array);
    vec.push(1);

    assert_eq!(vec.capacity(), 32);
    assert_eq!(vec.size(), 17);
    assert_eq!(vec.at(16), Ok(1));
}

#[test]
fn prepend_item() {
    let array: [i32; 4] = [0; 4];

    let mut vec = Vector::new(&array);
    vec.prepend(1);

    assert_eq!(vec.size(), 5);
    assert_eq!(vec.at(0), Ok(1));
}

#[test]
fn prepend_item_empty_vec() {
    let array: [i32; 0] = [];

    let mut vec = Vector::new(&array);
    vec.prepend(1);

    assert!(!vec.is_empty());
    assert_eq!(vec.size(), 1);
    assert_eq!(vec.at(0), Ok(1));
}

#[test]
fn prepend_item_and_increase_capacity() {
    let array: [i32; 16] = [0; 16];

    let mut vec = Vector::new(&array);
    vec.prepend(1);

    assert_eq!(vec.capacity(), 32);
    assert_eq!(vec.size(), 17);
    assert_eq!(vec.at(0), Ok(1));
}

#[test]
fn insert_item() {
    let array: [i32; 4] = [0; 4];

    let mut vec = Vector::new(&array);
    let res = vec.insert(3, 1);

    assert!(res.is_ok());
    assert_eq!(vec.size(), 5);
    assert_eq!(vec.at(3), Ok(1));
}

#[test]
fn insert_item_at_end() {
    let array: [i32; 4] = [0; 4];

    let mut vec = Vector::new(&array);
    let res = vec.insert(4, 1);

    assert!(res.is_ok());
    assert_eq!(vec.size(), 5);
    assert_eq!(vec.at(4), Ok(1));
}

#[test]
fn insert_item_in_empty_vec() {
    let array: [i32; 0] = [];

    let mut vec = Vector::new(&array);
    let res = vec.insert(0, 1);

    assert!(res.is_ok());
    assert_eq!(vec.size(), 1);
    assert_eq!(vec.at(0), Ok(1));
}

#[test]
fn insert_item_out_of_bounds() {
    let array: [i32; 4] = [0; 4];

    let mut vec = Vector::new(&array);
    let res = vec.insert(6, 0);

    assert!(res.is_err());
    assert_eq!(vec.size(), 4);
}

#[test]
fn insert_item_and_increase_capacity() {
    let array: [i32; 16] = [0; 16];

    let mut vec = Vector::new(&array);
    let res = vec.insert(8, 1);

    assert!(res.is_ok());
    assert_eq!(vec.capacity(), 32);
    assert_eq!(vec.size(), 17);
    assert_eq!(vec.at(8), Ok(1));
}

#[test]
fn pop_item() {
    let array: [i32; 4] = [2, 3, 1, 4];

    let mut vec = Vector::new(&array);

    let mut res = vec.pop();
    assert_eq!(res, Ok(4));
    assert_eq!(vec.size(), 3);
    assert_eq!(vec.capacity(), 4);

    res = vec.pop();
    assert_eq!(res, Ok(1));
    assert_eq!(vec.size(), 2);
    assert_eq!(vec.capacity(), 2);

    res = vec.pop();
    assert_eq!(res, Ok(3));
    assert_eq!(vec.size(), 1);
    assert_eq!(vec.capacity(), 1);

    res = vec.pop();
    assert_eq!(res, Ok(2));
    assert_eq!(vec.size(), 0);
    assert_eq!(vec.capacity(), 1);
}

#[test]
fn pop_item_empty() {
    let array: [i32; 0] = [];

    let mut vec = Vector::new(&array);

    let res = vec.pop();
    assert!(res.is_err());
}

#[test]
fn pop_item_decrease_capacity() {
    let array: [i32; 17] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    let mut vec = Vector::new(&array);

    let res = vec.pop();
    assert_eq!(res, Ok(16));
    assert_eq!(vec.at(15), Ok(15));
    assert_eq!(vec.capacity(), 16);
}

#[test]
fn delete_item() {
    let array: [i32; 4] = [2, 3, 1, 4];

    let mut vec = Vector::new(&array);

    let res = vec.delete(2);
    assert!(res.is_ok());
}

#[test]
fn delete_item_out_of_range() {
    let array: [i32; 4] = [2, 3, 1, 4];

    let mut vec = Vector::new(&array);

    let res = vec.delete(4);
    assert!(res.is_err());
}

#[test]
fn delete_item_and_decrease_capacity() {
    let array: [i32; 17] = [0; 17];

    let mut vec = Vector::new(&array);

    let res = vec.delete(8);
    assert!(res.is_ok());
    assert_eq!(vec.capacity(), 16);
}

#[test]
fn remove_item() {
    let array: [i32; 4] = [3, 1, 3, 2];

    let mut vec = Vector::new(&array);

    vec.remove(3);
    assert_eq!(vec.size(), 2)
}

#[test]
fn remove_item_not_existing() {
    let array: [i32; 4] = [3, 1, 3, 2];

    let mut vec = Vector::new(&array);

    vec.remove(4);
    assert_eq!(vec.size(), 4)
}

#[test]
fn remove_item_and_decrease_capacity() {
    let array: [i32; 17] = [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4];

    let mut vec = Vector::new(&array);

    vec.remove(4);
    assert_eq!(vec.size(), 14);
    assert_eq!(vec.capacity(), 16);
}

#[test]
fn find_item() {
    let array: [i32; 4] = [3, 1, 4, 2];

    let vec = Vector::new(&array);
    let res = vec.find(4);

    assert_eq!(res, Ok(2));
}

#[test]
fn find_item_first_occurence() {
    let array: [i32; 5] = [4, 3, 1, 4, 2];

    let vec = Vector::new(&array);
    let res = vec.find(4);

    assert_eq!(res, Ok(0));
}

#[test]
fn find_item_not_existing() {
    let array: [i32; 4] = [3, 1, 4, 2];

    let vec = Vector::new(&array);
    let res = vec.find(5);

    assert!(res.is_err());
}
