pub fn sort<T: Ord>(list: &[T]) -> Vec<&T> {
    // Base case is when we get down to the last one.
    if list.len() == 1 {
        return list.iter().map(|v| v).collect();
    }
    if list.is_empty() {
        return Vec::new();
    }

    let mid = list.len() / 2;

    let left = sort(&list[..mid]);
    let right = sort(&list[mid..]);

    let mut results = Vec::with_capacity(list.len());

    let mut left_index = 0;
    let mut right_index = 0;

    loop {
        match (left.get(left_index), right.get(right_index)) {
            (Some(l), Some(r)) => {
                if l <= r {
                    results.push(*l);
                    left_index += 1;
                } else {
                    results.push(*r);
                    right_index += 1;
                }
            }
            (None, Some(r)) => {
                results.push(*r);
                right_index += 1;
            }
            (Some(l), None) => {
                results.push(*l);
                left_index += 1;
            }
            (None, None) => {
                break;
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sorts_the_list() {
        assert_eq!(sort(&[4, 3, 2, 6, 9]), [&2, &3, &4, &6, &9]);
    }

    #[test]
    fn it_sorts_the_list_with_duplicates() {
        assert_eq!(sort(&[4, 3, 2, 3, 6, 2, 9]), [&2, &2, &3, &3, &4, &6, &9]);
    }

    #[test]
    fn it_sorts_stably() {
        #[derive(Eq, PartialEq, Debug)]
        struct KnownOrder {
            position: usize,
        }

        use std::cmp::Ordering;
        impl Ord for KnownOrder {
            fn cmp(&self, _: &KnownOrder) -> Ordering {
                Ordering::Equal
            }
        }

        impl PartialOrd for KnownOrder {
            fn partial_cmp(&self, other: &KnownOrder) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl KnownOrder {
            fn new(position: usize) -> KnownOrder {
                KnownOrder { position }
            }
        }

        assert_eq!(
            sort(&[KnownOrder::new(1), KnownOrder::new(2), KnownOrder::new(3)]),
            &[&KnownOrder::new(1), &KnownOrder::new(2), &KnownOrder::new(3)]
            );
    }
}
