use std::collections::HashMap;

pub fn puzzle_solution()
{
    let input_file = "data/day1/input_1.txt";

    let file_content = std::fs::read_to_string(input_file).unwrap();
    let file_dist_pairs: Vec<(i32, i32)> = file_content
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| 
        {
            //println!("Line: {line}");
            let mut split_line = line.split_whitespace();
            let first_num = split_line.next().unwrap().parse::<i32>().unwrap();
            let second_num = split_line.next().unwrap().parse::<i32>().unwrap();

            (first_num, second_num)
        })
        .collect::<Vec<(i32, i32)>>();

    let mut left_numbers = file_dist_pairs
        .iter()
        .map(|x| x.0)
        .collect::<Vec<i32>>();
    let mut right_numbers = file_dist_pairs
        .iter()
        .map(|x| x.1)
        .collect::<Vec<i32>>();

    left_numbers.sort();
    right_numbers.sort();


    let total_dist = left_numbers.iter().zip(right_numbers.iter())
        .fold(0, |acc, x| acc + (x.0 - x.1).abs());

    println!("{total_dist}");
        
    let mut left_number_dict: HashMap<i32, (i32,i32)> = HashMap::new();

    for left_num in left_numbers.iter_mut()
    {
        //value.0 is the count of this num encountered in original list
        //value.1 is the count of this num encountered in other list
        let left_entry = left_number_dict.entry(*left_num).or_insert((0,0));
        left_entry.0 += 1;
        left_entry.1 = 0;
    }


    for right_num in right_numbers
    {
        if let Some(entry) = left_number_dict.get(&right_num)
        {
           left_number_dict.insert(right_num, (entry.0, entry.1 + 1)); 
        }
        
    }
    
    let mut similarity_score = 0;
    for (key, val) in left_number_dict.iter()
    {
        similarity_score += key * val.0 * val.1;
    }

    println!("Similarity score: {similarity_score}");
}
