fn generate_digital(num: i32) -> String {
    String::from(
        match num {
            1 => "One",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eight",
            9 => "Nine",
            _ => { panic!("generate digital") }
        }
    )
}


fn generate_less_hundred(num: i32) -> String {
    if num < 10 {
        generate_digital(num)
    } else if num < 20 {
        String::from(
            match num {
                10 => "Ten",
                11 => "Eleven",
                12 => "Twelve",
                13 => "Thirteen",
                14 => "Fourteen",
                15 => "Fifteen",
                16 => "Sixteen",
                17 => "Seventeen",
                18 => "Eighteen",
                19 => "Nineteen",
                _ => { panic!("generate less than 20") }
            }
        )
    } else {
        let mut result = String::from(
            match num / 10 {
                2 => "Twenty",
                3 => "Thirty",
                4 => "Forty",
                5 => "Fifty",
                6 => "Sixty",
                7 => "Seventy",
                8 => "Eighty",
                9 => "Ninety",
                _ => { panic!("generate tens") }
            }
        );

        if num % 10 != 0 {
            result.push(' ');
            result.push_str(generate_digital(num % 10).as_str());
        }

        result
    }
}

fn generate_less_thousand(num: i32) -> String {
    let mut result = String::new();
    if num / 100 != 0 {
        result.push_str(generate_digital(num / 100).as_str());
        result.push_str(" Hundred");
    }
    if num % 100 != 0 {
        if !result.is_empty() { result.push(' '); }
        result.push_str(generate_less_hundred(num % 100).as_str());
    }
    result
}

impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return String::from("Zero");
        }

        let mut result = String::new();
        let mut i = 0;

        while num != 0 {
            i += 1;

            let hundred = num % 1000;
            num /= 1000;
            if hundred == 0 {
                continue;
            }

            let mut it = generate_less_thousand(hundred);

            if i == 1 {
                // do nothing
            } else if i == 2 {
                it.push_str(" Thousand");
            } else if i == 3 {
                it.push_str(" Million");
            } else {
                it.push_str(" Billion");
            }
            if !result.is_empty() {
                it.push(' ');
            }
            it.push_str(result.as_str());
            result = it;
        }

        result
    }
}

struct Solution {}

fn main() {}