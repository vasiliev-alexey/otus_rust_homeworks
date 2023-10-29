use hw11::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push_back(2);
    list.push_front(1);
    list.push_back(3);

    println!("initial list: {}", list);

    list.change_value_by_index(2, 4);

    println!("{}", list);

    list.push_back(5);
    list.push_back(6);
    println!("{}", list);
    let list_split_part = list.split_at(2);
    println!("list_split_part: {}", list_split_part.unwrap());
    println!("original list after split: {}", list);
}
