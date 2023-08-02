use std::collections::HashSet;

fn remove_duplicates<T>(list: Vec<T>) -> Vec<T> where T: Clone + std::cmp::Eq + std::hash::Hash + std::fmt::Debug {
    let mut seen = HashSet::new();
    let mut uniques = list.clone();
    uniques.retain(|c| seen.insert(c.clone()));
    uniques
}

#[test]
fn rm_dupl() {
    let list = vec![0,1,2,3,4,10,2,4,5,3,2,2,43,2,21,1,2,3,2,2,1,1,1];
    let uniques = remove_duplicates(list);
    assert_eq!(uniques, vec![0,1,2,3,4,10,5,43,21]);
}

pub fn distribute<T>(list: &mut Vec<T>) where T: Clone + std::cmp::Eq + std::hash::Hash + std::fmt::Debug {
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

    let mut prioheap = Vec::<PrioItem<T>>::new();
    for ival in remove_duplicates(list.to_vec()).iter() {
        let value = ival;
        let count = list.iter().filter(|x| *x == value).count();
        let freq = list.len() as f64 / count as f64;
        let prio = freq/2.0;
        put_on_heap(&mut prioheap,
            PrioItem {
                value: value.clone(), count, freq, prio
            });
    }

    list.clear();
    loop {
        match prioheap.pop() {
            Some(mut prioitem) => {
                list.push(prioitem.value.clone());
                prioitem.count -= 1;
                prioitem.prio += prioitem.freq;
                if prioitem.count > 0 {
                    put_on_heap(&mut prioheap, prioitem);
                }
            },
            None => break
        }
    }
}

#[test]
fn distribution() {
    let mut list = vec![0,0,1,1,2,2];
    distribute(&mut list);
    assert_eq!(list, vec![0,1,2,0,1,2]);

    let mut list = vec![0,0,0,0,1,1,1,2,2,3];
    distribute(&mut list);
    assert_eq!(list, vec![0,1,2,0,3,1,0,2,1,0]);

    let mut list = vec![0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,2,2,2,2,2,2,3,3,3,3,3,4,4,4,4,4,5,5,5,5,6,6,6,7,7,7,8,8,9,9,10,10,11,12,13,14];
    distribute(&mut list);
    assert_eq!(list, vec![0,1,2,3,4,5,6,7,0,1,10,8,9,2,3,4,0,1,5,2,0,13,14,11,12,6,7,4,3,1,0,2,5,1,0,4,3,10,8,9,2,1,0,7,6,5,4,3,2,1,0]);
}
