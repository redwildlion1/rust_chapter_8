use std::collections::HashMap;

fn main() {
    // Given a list of integers, use a vector and return the mean (the average
    // value), median (when sorted, the value in the middle position), and
    // mode (the value that occurs most often; a hash map will be helpful
    // here) of the list.
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mean = mean(&list);
    let median = median(&list);
    let mode = mode(&list);
    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);

    fn mean(list: &Vec<i32>) -> f64 {
        let sum: i32 = list.iter().sum();
        sum as f64 / list.len() as f64
    }

    fn median(list: &Vec<i32>) -> i32{
        let mut sorted_list = list.clone();
        sorted_list.sort();
        let mid = sorted_list.len() / 2;
        sorted_list[mid]
    }

    fn mode(list: &Vec<i32>) -> i32{
        let mut hash_map:HashMap<i32,i32> = HashMap::new();
        for &i in list.iter(){
            let count = hash_map.entry(i).or_insert(0);
            *count += 1;
        }
        let mut max = 0;
        let mut mode = 0;
        for (key, value) in hash_map.iter(){
            if *value > max{
                max = *value;
                mode = *key;
            }
        }
        mode
    }
}
