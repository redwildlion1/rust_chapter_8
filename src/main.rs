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

    //Convert strings to pig latin. The first consonant of each word is moved
    // to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead
    // (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
    // encoding!
    let string = "first apple";
    let pig_latin = pig_latin(string);
    println!("Pig Latin: {}", pig_latin);

    fn pig_latin(string: &str) -> String{
        let mut pig_latin = String::new();
        for word in string.split_whitespace(){
            let mut chars = word.chars();
            let first_char = chars.next().unwrap();
            if first_char == 'a' || first_char == 'e' || first_char == 'i' || first_char == 'o' || first_char == 'u'{
                pig_latin.push_str(word);
                pig_latin.push_str("-hay ");
            }else{
                let mut new_word = String::new();
                for c in chars{
                    new_word.push(c);
                }
                new_word.push('-');
                new_word.push(first_char);
                new_word.push_str("ay ");
                pig_latin.push_str(&new_word);
            }
        }
        pig_latin
    }
}
