use std::marker::Sized;
use num::integer::lcm;

pub struct Batches<T> {
    loop_back: bool,
    elements: Vec<T>,

    offset: usize,
    total: usize,
    size: usize
}

impl<T: Sized + Clone> Iterator for Batches<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.offset < self.total {
            let from = self.offset;
            let to = self.offset + self.size;

            let batch: Vec<T> = self.elements[from..to].to_vec();
            self.offset += self.size;
            Some(batch)
        } else {
            if self.loop_back {
                self.offset = 0;
                self.next()
            } else {
                None
            }
        }
    }
}

pub fn prepare<T: Clone>(samples: Vec<T>, batch_size: usize, loop_back: bool) -> Batches<T> {
    if batch_size == 0 || samples.len() == 0 {
        Batches {
            loop_back: false,
            elements: vec![],
            offset: 0,
            total: 0,
            size: 0
        }
    } else {
        let samples_amount = samples.len();

        let total_elements = lcm(samples_amount, batch_size);
        let repeat_times =  total_elements / samples_amount;

        let several_cycles: Vec<T> = vec![samples; repeat_times]
            .into_iter().flatten().collect();

        Batches {
            loop_back: loop_back,
            elements: several_cycles,
            offset: 0,
            total: total_elements,
            size: batch_size
        }
    }
}

#[test]
fn test_3_over_5() {
    let mut batches = prepare(vec![1,2,3,4,5], 3, false);
    assert_eq!(batches.next(), Some(vec![1,2,3]));
    assert_eq!(batches.next(), Some(vec![4,5,1]));
    assert_eq!(batches.next(), Some(vec![2,3,4]));
    assert_eq!(batches.next(), Some(vec![5,1,2]));
    assert_eq!(batches.next(), Some(vec![3,4,5]));
    assert_eq!(batches.next(), None);
}

#[test]
fn test_loop_back() {
    let mut batches = prepare(vec![1,2,3,4,5], 3, true);
    assert!(batches.take(1000).next().is_some())
}