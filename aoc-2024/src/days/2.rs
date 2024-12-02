struct Report { levels: Vec<i32>, }

pub fn solve(lines: impl Iterator<Item = String>) {
    let mut reports: Vec<Report> = Vec::new(); 
    for line in lines {
        let levels: Vec<i32> = line
            .split(" ")
            .map(|level_text| level_text.parse::<i32>().unwrap())
            .collect();

        reports.push(Report{ levels, });
    }

    let safe_reports_count = reports
        .iter()
        .filter(|report| are_levels_safe(&report.levels))
        .count();

    println!("The number of safe reports is {}", safe_reports_count);

    // 2nd star

    let safe_reports_with_dampener_count = reports
        .iter()
        .filter(|report| are_levels_safe_with_dampener(&report.levels))
        .count();

    println!("The number of safe reports with the Problem Dampener is {}", safe_reports_with_dampener_count);
}

fn are_levels_safe(levels: &[i32]) -> bool {
    let is_sorted = levels.is_sorted() || levels.is_sorted_by(|a, b| a >= b);
    let are_differences_valid = levels.is_sorted_by(is_difference_valid);
    return is_sorted && are_differences_valid;
}

fn is_difference_valid(a: &i32, b: &i32) -> bool {
    let difference = (a - b).abs();
    return difference >= 1 && difference <= 3;
}

fn are_levels_safe_with_dampener(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() {
        let dampened_levels = levels
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != i)
            .map(|(_, value)| *value)
            .collect::<Vec<_>>();

        if are_levels_safe(&dampened_levels) {
            return true;            
        }
    }

    return false;
}