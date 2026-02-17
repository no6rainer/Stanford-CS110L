use linked_list::{ComputeNorm, LinkedList};
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    let display_ok = list.to_string().trim() == "10 9 8 7 6 5 4 3 2 1";
    println!("Display: {}", if display_ok { "PASS" } else { "FAIL" });

    let mut list_clone = list.clone();
    let original_size = list.get_size();
    let cloned_size = list_clone.get_size();
    let _ = list_clone.pop_front();
    let clone_ok = list.get_size() == original_size && list_clone.get_size() + 1 == cloned_size;
    println!("Clone: {}", if clone_ok { "PASS" } else { "FAIL" });

    let mut eq_a: LinkedList<i32> = LinkedList::new();
    eq_a.push_front(1);
    eq_a.push_front(2);
    let mut eq_b: LinkedList<i32> = LinkedList::new();
    eq_b.push_front(1);
    eq_b.push_front(2);
    let mut neq: LinkedList<i32> = LinkedList::new();
    neq.push_front(1);
    neq.push_front(3);
    let partial_eq_ok = eq_a == eq_b && eq_a != neq;
    println!("PartialEq: {}", if partial_eq_ok { "PASS" } else { "FAIL" });

    let mut string_list: LinkedList<String> = LinkedList::new();
    string_list.push_front("world".to_string());
    string_list.push_front("hello".to_string());
    let generic_display_ok = string_list.to_string().trim() == "hello world";
    println!(
        "Generic String + Display: {}",
        if generic_display_ok { "PASS" } else { "FAIL" }
    );

    // If you implement iterator trait:
    for val in &list {
        println!("{}", val);
    }

    let mut owned_iter_list: LinkedList<i32> = LinkedList::new();
    owned_iter_list.push_front(1);
    owned_iter_list.push_front(2);
    owned_iter_list.push_front(3);
    let consumed_values: Vec<i32> = owned_iter_list.into_iter().collect();
    let owned_into_iter_ok = consumed_values == vec![3, 2, 1];
    println!(
        "IntoIterator (owned): {}",
        if owned_into_iter_ok { "PASS" } else { "FAIL" }
    );

    let mut shared_iter_list: LinkedList<i32> = LinkedList::new();
    shared_iter_list.push_front(4);
    shared_iter_list.push_front(5);
    shared_iter_list.push_front(6);
    let referenced_values: Vec<i32> = (&shared_iter_list).into_iter().copied().collect();
    let shared_into_iter_ok = referenced_values == vec![6, 5, 4] && shared_iter_list.get_size() == 3;
    println!(
        "IntoIterator (&): {}",
        if shared_into_iter_ok { "PASS" } else { "FAIL" }
    );

    let mut mutable_iter_list: LinkedList<i32> = LinkedList::new();
    mutable_iter_list.push_front(7);
    mutable_iter_list.push_front(8);
    mutable_iter_list.push_front(9);
    for value in &mut mutable_iter_list {
        *value += 10;
    }
    let mutated_values: Vec<i32> = (&mutable_iter_list).into_iter().copied().collect();
    let mut_into_iter_ok = mutated_values == vec![19, 18, 17];
    println!(
        "IntoIterator (&mut): {}",
        if mut_into_iter_ok { "PASS" } else { "FAIL" }
    );

    let mut norm_list: LinkedList<f64> = LinkedList::new();
    norm_list.push_front(4.0);
    norm_list.push_front(3.0);
    let norm = norm_list.compute_norm();
    let compute_norm_ok = (norm - 5.0).abs() < 1e-10;
    println!(
        "ComputeNorm: {}",
        if compute_norm_ok { "PASS" } else { "FAIL" }
    );
}
