const dist_min: i32 = 1;
const dist_max: i32 = 3;

struct ReportEntry
{
    data: Vec<i32>,
    is_safe: bool
}

impl ReportEntry
{
    pub fn new(data: Vec<i32>) -> ReportEntry
    {
        ReportEntry{data, is_safe: false}
    }
    pub fn is_safe_report(data: &[i32]) -> bool
    {
        let mut inc_dec: i8 = -1;
        let mut unsafe_inc_dec: bool = false;
        let mut unsafe_distance = false;
        for (prev, current) in data.iter().zip(data.iter().skip(1))
        {
            let diff = prev - current;
            let inc_dec_current = match diff 
            {
                x if x > 0 => 2,
                x if x < 0 => 1,
                _ => 0 
            };

            if inc_dec == -1
            {inc_dec = inc_dec_current;}

            unsafe_inc_dec = inc_dec != inc_dec_current;
            unsafe_distance = diff.abs() > dist_max || diff.abs() < dist_min;

            if unsafe_distance || unsafe_inc_dec
            {return false;}
        }

        true 
    }
    pub fn check_safe_report(&mut self)
    {
        self.is_safe = ReportEntry::is_safe_report(&self.data);
    }

    pub fn is_safe_report_with_one_joker(&self) -> bool
    {
        for index in 0..self.data.len()
        {
            println!("Retrying report without index {index} num {}", self.data.get(index).unwrap()); 
            let mut cloned_data = self.data.clone();
            let _ = cloned_data.remove(index);
            let is_safe_report = ReportEntry::is_safe_report(&cloned_data);
            
            if is_safe_report
            {return true;}
        
           
        }

        println!("Failed fixing the row");

        return false;
    }
}


pub fn puzzle_solution()
{
    let input_file = "data/day2/input_1.txt";
    let file_content = std::fs::read_to_string(input_file).unwrap();

    let number_vecs: Vec<Vec<i32>> = file_content
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| 
        {
            //println!("Line: {line}");
            let mut split_line = line.split_whitespace();
            let mut numbers: Vec<i32> = Vec::new();

            while let Some(num) = split_line.next()
            {numbers.push(num.parse::<i32>().unwrap());}

            numbers
        })
        .collect::<Vec<Vec<i32>>>();

    let mut safe_count: u32 = 0;
    let mut reports: Vec<ReportEntry> = number_vecs.into_iter().map(|x| ReportEntry::new(x)).collect();

    for report in reports.iter_mut()
    {
       report.check_safe_report();
       if report.is_safe
       {safe_count += 1;}
    }

    println!("Day2: Safe count: {safe_count}");

    for report in reports.iter().filter(|x| !x.is_safe)
    {
        if report.is_safe_report_with_one_joker()        
        {safe_count += 1;}
    }

    println!("Day2: Safe count with Problem Dampener active: {}", safe_count);

}

fn part_2(number_vecs: &[Vec<i32>]) -> String
{
    let mut safe_count: u32 = 0;

    for number_vec in number_vecs.iter()
    {
        //1=incrementing, 2=decrementing, 0=same, -1=init
        let mut inc_dec: i8 = -1;
        let mut unsafe_inc_dec: bool = false;
        let mut unsafe_distance = false;
        let mut joker_used: bool = false;
        for (index, (prev, current)) in number_vec.iter().zip(number_vec.iter().skip(1)).enumerate()
        {
            let diff = prev - current;
            let inc_dec_current = match diff 
            {
                x if x > 0 => 2,
                x if x < 0 => 1,
                _ => 0 
            };

            if inc_dec == -1
            {inc_dec = inc_dec_current;}

            unsafe_inc_dec = inc_dec != inc_dec_current;
            unsafe_distance = diff.abs() > dist_max || diff.abs() < dist_min;


            if unsafe_distance || unsafe_inc_dec
            {
                match joker_used
                {
                    true => 
                    {
                        println!("Unsafe number for {prev} - {current}: {}", number_vec
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" "));
                            break;
                    }
                    false => 
                    {
                        if unsafe_inc_dec
                        {
                            println!("==Resetting inc dec==");
                            inc_dec = -1;
                        }

                        unsafe_inc_dec = false;
                        unsafe_distance = false;
                        joker_used = true;
                        
                        /*println!("Used joker for {prev} - {current}: {}", number_vec
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" "));
                        */
                    }
                }
            }
        }

        //Decide whether safe or unsafe
        if !unsafe_inc_dec && !unsafe_distance
        {safe_count += 1;}
    }

    format!("{safe_count}")
}
