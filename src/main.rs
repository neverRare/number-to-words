use std::io;

fn main() {
    println!("Enter a number to spell out (enter \"exit\" to exit)");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let input: u64 = match input.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let output = match wordify(input, 0) {
            Ok(words) => words,
            Err(_) => {
                continue;
            }
        };
        println!("= {}", output);
    }
}
fn wordify_ones(num: u64) -> Result<String, String> {
    Ok(String::from(match num {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => {
            return Err(format!("{} is not a 1 digit number", num));
        }
    }))
}
fn wordify_tens(num: u64) -> Result<String, String> {
    if num == 0 {
        return Ok(String::from("zero"));
    }
    let ones = num % 10;
    let tens = (num - ones) / 10;
    if tens == 0 {
        return wordify_ones(ones);
    }
    if tens == 1 {
        return Ok(String::from(match ones {
            0 => "ten",
            1 => "eleven",
            2 => "twelve",
            3 => "thirteen",
            4 => "fourteen",
            5 => "fifteen",
            6 => "sixteen",
            7 => "seventeen",
            8 => "eighteen",
            9 => "nineteen",
            _ => {
                return Err(String::from("invalid number"));
            }
        }));
    }
    let tens_str = String::from(match tens {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => {
            return Err(String::from("invalid number"));
        }
    });
    Ok(if ones == 0 {
        format!("{}", tens_str)
    } else {
        format!("{}-{}", tens_str, wordify_ones(ones)?)
    })
}
fn wordify_hundreds(num: u64) -> Result<String, String> {
    if num == 0 {
        return Ok(String::from("zero"));
    }
    let all_tens = num % 100;
    let hundreds = (num - all_tens) / 100;
    if hundreds == 0 {
        return wordify_tens(all_tens);
    }
    let hundreds_str = wordify_ones(hundreds)?;
    Ok(if all_tens == 0 {
        format!("{} hundred", hundreds_str)
    } else {
        format!("{} hundred {}", hundreds_str, wordify_tens(all_tens)?)
    })
}
fn get_thousand_word(place: u32) -> Result<String, String> {
    Ok(String::from(match place {
        0 => "",
        1 => "thousand",
        2 => "million",
        3 => "billion",
        4 => "trillion",
        5 => "quadrillion",
        6 => "quintillion",
        7 => "sextillion",
        8 => "septillion",
        9 => "octillion",
        10 => "nonillion",
        11 => "decillion",
        12 => "undecillion",
        13 => "duodecillion",
        _ => {
            return Err(String::from("overflow"));
        }
    }))
}
fn wordify(num: u64, thousandth_place: u32) -> Result<String, String> {
    if num == 0 {
        return Ok(String::from("zero"));
    }
    let all_hundreds = num % 1000;
    let rest = (num - all_hundreds) / 1000;
    let hundreds_str = wordify_hundreds(all_hundreds)?;
    let rest_str = wordify(rest, thousandth_place + 1)?;
    let place = get_thousand_word(thousandth_place)?;
    Ok(if all_hundreds == 0 {
        rest_str
    } else if rest == 0 {
        if thousandth_place == 0 {
            format!("{}", hundreds_str)
        } else {
            format!("{} {}", hundreds_str, place)
        }
    } else if thousandth_place == 0 {
        format!("{} {}", rest_str, hundreds_str)
    } else {
        format!("{} {} {}", rest_str, hundreds_str, place)
    })
}
