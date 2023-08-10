#[derive(Debug, Clone)]
struct PrioItem<K> {
    value: K,
    count: usize,
    freq: f64,
    prio: f64
}

fn put_on_heap<K>(heap: &mut Vec::<PrioItem<K>>, item: PrioItem<K>) {
    let mut idx = heap.len();
    heap.push(item);
    while idx > 0 {
        if heap[idx].prio > heap[idx-1].prio || (heap[idx].prio == heap[idx-1].prio && heap[idx].freq <= heap[idx-1].freq) {
            heap.swap(idx, idx-1);
           }
        idx -= 1;
    }
}

pub fn distribute<T>(counts: &Vec<(usize, T)>) -> Vec<T> 
    where T: Clone + std::cmp::Eq + std::hash::Hash + std::fmt::Debug {
    let total_count = counts.iter().fold(0, |acc, x| acc + x.0);
    let mut prioheap = Vec::<PrioItem<T>>::with_capacity(counts.len());
    for count in counts {
        if count.0 > 0 {
           let freq = total_count as f64 / count.0 as f64;
           let prio = freq/2.0;
           put_on_heap(&mut prioheap,
               PrioItem {
                   value: count.1.clone(),
                   count: count.0, freq, prio
               });
        }
    }

    let mut list = Vec::<T>::with_capacity(total_count);
    while let Some(mut prioitem) = prioheap.pop() {
        list.push(prioitem.value.clone());
        prioitem.count -= 1;
        prioitem.prio += prioitem.freq;
        if prioitem.count > 0 {
            put_on_heap(&mut prioheap, prioitem);
        }
    }
    list
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn distribution() {
        let element_counts = vec![(0, 0), (0, 1), (2, 2)];
        let list = distribute(&element_counts);
        assert_eq!(list, vec![2,2]);

        let element_counts = vec![(2, 0), (2, 1), (2, 2)];
        let list = distribute(&element_counts);
        assert_eq!(list, vec![0,1,2,0,1,2]);

        let element_counts = vec![(4, 0), (3, 1), (2, 2), (1, 3)];
        let list = distribute(&element_counts);
        assert_eq!(list, vec![0,1,2,0,3,1,0,2,1,0]);

        let element_counts = vec![(8, 0), (7, 1), (6, 2), (5, 3), (5, 4), (4, 5), (3, 6),
                                  (3, 7), (2, 8), (2, 9), (2,10), (1, 11), (1, 12), (1, 13), (1,14)];
        let list = distribute(&element_counts);
        assert_eq!(list, vec![0,1,2,3,4,5,6,7,0,1,10,8,9,2,3,4,0,1,5,2,0,13,14,11,12,
                              6,7,4,3,1,0,2,5,1,0,4,3,10,8,9,2,1,0,7,6,5,4,3,2,1,0]);
    }
}
