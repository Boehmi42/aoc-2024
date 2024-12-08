use regex::{CaptureMatches, Regex};

pub fn puzzle_solution()
{
    //Use regex mul\([0-9]+,[0-9]\) for matching correct mul stuff? :)
    let input_file = "data/day3/input_1.txt";

    let file_content = std::fs::read_to_string(input_file).unwrap().replace("\n", "");

    //let re = Regex::new(r"/mul\([0-9]+,[0-9]+\)/g").unwrap();
    let re = Regex::new(r"mul\(([0-9]+,[0-9]+)\)").unwrap();

    let mut sum: u64 = 0;

    for line in file_content.split_whitespace()
    {
        //let mul_strs: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let mul_strs: Vec<&str> = re.captures_iter(line).map(|m| m.get(1).unwrap()).map(|x| x.as_str()).collect();
        //let mul_strs: Vec<&str> = re.captures_iter(line).map(|m| m.get(0).unwrap().as_str()).collect();

        for (a,b) in mul_strs.iter().map(|x| x.split_once(",").unwrap())
        {
            sum += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
        }
    }

    println!("Sum: {sum}");

    let mut sum_2: u64 = 0;

    //let re_begin = Regex::new(r".*(mul\(([0-9]+,[0-9]+)\)).*don't\(\)").unwrap();
    let re_begin = Regex::new(r".*?don't\(\)").unwrap();
    let re_midpart = Regex::new(r"do\(\).*?don't\(\)").unwrap();
    let re_end = Regex::new(r"do\(\).*").unwrap();


    let start_block_match = re_begin.find(&file_content).unwrap();
    let start_block_str = start_block_match.as_str();
    let start_block_end = start_block_match.end();
    let mut muls = re.captures_iter(start_block_str).map(|x| x.get(1).unwrap().as_str()).collect::<Vec<&str>>();

    let mut sum_of_muls: Vec<u64> = muls
        .into_iter()
        .map(|x| x.split_once(",").unwrap())
        .map(|(a,b)| a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap())
        .collect();

    sum_of_muls.iter().for_each(|num|
    {
       sum_2 += num;
    });

    let mid_blocks_match = re_midpart.find_iter(file_content.get(start_block_end..).unwrap()).collect::<Vec<regex::Match>>();
    let mid_block_end = start_block_end + mid_blocks_match.iter().max_by_key(|x| x.end()).unwrap().end();

    for mid_block in mid_blocks_match
    {
        let mid_block_str = mid_block.as_str();

        let muls = re.captures_iter(mid_block_str).map(|x| x.get(1).unwrap().as_str()).collect::<Vec<&str>>();

        let sum_of_muls: Vec<u64> = muls
            .into_iter()
            .map(|x| x.split_once(",").unwrap())
            .map(|(a,b)| a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap())
            .collect();

        sum_of_muls.iter().for_each(|num|
        {
           sum_2 += num;
        });

    }

    if let Some(end_block_match) = re_end.find(file_content.get(mid_block_end..).unwrap())
    {
        let end_block_str = end_block_match.as_str();

        muls = re.captures_iter(end_block_str).map(|x| x.get(1).unwrap().as_str()).collect::<Vec<&str>>();

        let sum_of_muls: Vec<u64> = muls
            .into_iter()
            .map(|x| x.split_once(",").unwrap())
            .map(|(a,b)| a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap())
            .collect();

        sum_of_muls.iter().for_each(|num|
        {
           sum_2 += num;
        });
    }

    println!("Sum 2: {sum_2}");
}
