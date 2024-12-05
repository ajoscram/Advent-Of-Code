pub const DAY: &str = "5";

struct Rule {
    before: i32,
    after: i32,
}

struct Update {
    pages: Vec<i32>,
}

pub fn solve(lines: impl Iterator<Item = String>) {
    let mut rules: Vec<Rule> = vec![];
    let mut updates: Vec<Update> = vec![];

    for line in lines {
        if let Some(rule) = Rule::new(&line) {
            rules.push(rule);
        }
        else if let Some(update) = Update::new(&line) {
            updates.push(update);
        }
    }

    let middle_page_sum: i32 = updates
        .iter()
        .filter(|update| update.respects(&rules))
        .map(|update| update.middle_page())
        .sum();

    println!("The sum of all middle pages in correctly-ordered updates is {}", middle_page_sum);
    
    // 2nd star

    let middle_page_ordered_sum: i32 = updates
        .iter()
        .filter(|update| !update.respects(&rules))
        .map(|update| update.order(&rules))
        .map(|update| update.middle_page())
        .sum();

    println!("The sum of all middle pages in updates that needed ordering is {}", middle_page_ordered_sum);
}

impl Rule {
    fn new(line: &String) -> Option<Rule> {
        const PIPE: char = '|';

        if !line.contains(PIPE) {
            return None;
        }

        let split_numbers: Vec<i32> = line
            .split(PIPE)
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        return Some(Rule { before: split_numbers[0], after: split_numbers[1] });
    }

    fn is_respected(&self, update: &Update) -> bool {
        return update.pages
            .iter()
            .skip_while(|page| **page != self.after)
            .all(|page| *page != self.before);
    }

    fn order(&self, update: &mut Update) {
        let before_index = update.pages
            .iter()
            .position(|page| *page == self.before);
        let after_index = update.pages
            .iter()
            .position(|page| *page == self.after);

        if let (Some(before_index), Some(after_index)) = (before_index, after_index) {
            
            if before_index < after_index {
                return;
            }

            update.pages.swap(before_index, after_index);
        }
    }
}

impl Update {
    fn new(line: &String) -> Option<Update> {
        const COMMA: char = ',';

        if !line.contains(COMMA) {
            return None;
        }

        let pages: Vec<i32> = line
            .split(COMMA)
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        return Some(Update { pages });
    }

    fn middle_page(&self) -> i32 {
        return self.pages[self.pages.len() / 2];
    }

    fn respects(&self, rules: &Vec<Rule>) -> bool {
        return rules
            .iter()
            .all(|rule| rule.is_respected(self));
    }

    fn order(&self, rules: &Vec<Rule>) -> Update {
        let mut ordered = Update { pages: self.pages.clone() };

        while !ordered.respects(rules) {
            rules
                .iter()
                .for_each(|rule| rule.order(&mut ordered));
        }

        return ordered;
    }
}