use std::fmt::Display;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = ": ".to_string();

        let mut current = self.head.as_ref();
        for ind in 0..=(self.size) {
            if let Some(node) = current {
                if ind == 0 {
                    res.push_str(&node.value.to_string());
                }
                if ind != self.size - 1 {
                    res.push_str(" => ");
                }
                current = node.next.as_ref();
                if current.is_some() {
                    res.push_str(&current.as_ref().unwrap().value.to_string());
                }
            } else {
                break;
            }
        }
        write!(f, "LinkedList: Size: {}; Chain {}  ", self.size, res)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            current: self.head.as_deref(),
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });

        let mut last = self.head.as_mut();
        for _ in 0..(self.size) {
            if last.is_none() || last.as_ref().unwrap().next.is_none() {
                break;
            }
            last = last.unwrap().next.as_mut();
        }

        if let Some(last) = last {
            last.next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
        self.size += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn insert_after(&mut self, index: usize, value: T) {
        if index == 0 {
            self.push_front(value);
            return;
        }

        let mut current = self.head.as_deref_mut();
        for _ in 0..index {
            if let Some(node) = current {
                current = node.next.as_deref_mut();
            } else {
                return;
            }
        }

        if let Some(node) = current {
            let new_node = Box::new(Node {
                value,
                next: node.next.take(),
            });
            node.next = Some(new_node);
        }
        self.size += 1;
    }

    pub fn split_at(&mut self, index: usize) -> Option<LinkedList<T>> {
        if index == 0 || index > self.size {
            return None;
        }

        let mut current = self.head.as_deref_mut();
        let mut count = 1;

        while let Some(node) = current {
            if count == index {
                let next = node.next.take();
                let list = LinkedList {
                    head: next,
                    size: self.size - index,
                };
                self.size = index;
                return Some(list);
            }

            current = node.next.as_deref_mut();
            count += 1;
        }

        None
    }

    pub fn change_value_by_index(&mut self, index: usize, value: T) {
        let mut current = self.head.as_deref_mut();
        for _ in 0..=(index - 1) {
            if let Some(node) = current {
                current = node.next.as_deref_mut();
            } else {
                return;
            }
        }
        if let Some(ref mut node) = current {
            node.value = value;
        }
    }
}

pub struct ListIterator<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current {
            self.current = node.next.as_deref();
            Some(&node.value)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_push_back() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_push_front() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_insert_after() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(4);
        assert_eq!(list.iter().copied().collect::<Vec<i32>>(), vec![1, 2, 4]);
        list.insert_after(1, 3);

        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_split_at() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        let new_list = list.split_at(2).unwrap();

        let result2: Vec<i32> = new_list.iter().copied().collect();
        assert_eq!(result2, vec![3, 4]);

        let result1: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result1, vec![1, 2]);
    }

    #[test]
    fn test_push_back2() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_iter() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_push_front_and_back() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_back(2);
        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![1, 2]);
        list.push_front(3);
        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![3, 1, 2]);
        list.push_back(4);

        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![3, 1, 2, 4]);
    }

    #[test]
    fn test_insert_after_invalid_index() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.insert_after(5, 4);

        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_split_at_invalid_index() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let new_list = list.split_at(5);

        assert!(new_list.is_none());

        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_split_at_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        let new_list = list.split_at(0);
        assert!(new_list.is_none());
    }

    #[test]
    fn test_split() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        let new_list = list.split_at(2);
        assert!(new_list.is_some());
        let new_list = new_list.unwrap();
        let result: Vec<i32> = new_list.iter().copied().collect();
        assert_eq!(result, vec![3, 4]);
        assert_eq!(list.iter().copied().collect::<Vec<i32>>(), vec![1, 2]);
    }

    #[test]
    fn test_split_odd_elements() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        let new_list = list.split_at(2);
        assert!(new_list.is_some());
        let new_list = new_list.unwrap();
        let result: Vec<i32> = new_list.iter().copied().collect();
        assert_eq!(result, vec![3, 4, 5]);
        assert_eq!(list.iter().copied().collect::<Vec<i32>>(), vec![1, 2]);
    }

    #[test]
    fn test_len() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_change_value_by_index() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.change_value_by_index(1, 4);
        let result: Vec<i32> = list.iter().copied().collect();
        assert_eq!(result, vec![1, 4, 3]);
    }
}
