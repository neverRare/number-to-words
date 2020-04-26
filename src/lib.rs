fn wordify_ones(num: u64) -> Result<&'static str, &'static str> {
    Ok(match num {
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
        _ => return Err("invalid 1 digit number"),
    })
}
fn wordify_tens(num: u64) -> Result<String, &'static str> {
    if num == 0 {
        return Ok("zero".to_string());
    }
    let ones = num % 10;
    let tens = (num - ones) / 10;
    if tens == 0 {
        return Ok(wordify_ones(ones)?.to_string());
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
            _ => unreachable!(),
        }));
    }
    let tens_str = String::from(match tens {
        0 => unreachable!(),
        1 => unreachable!(),
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => return Err("invalid 2 digit number"),
    });
    Ok(if ones == 0 {
        tens_str
    } else {
        format!("{}-{}", tens_str, wordify_ones(ones)?)
    })
}
fn wordify_hundreds(num: u64) -> Result<String, &'static str> {
    if num == 0 {
        return Ok("zero".to_string());
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
fn get_thousand_word(place: u32) -> Result<String, &'static str> {
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
        _ => return Err("overflow"),
    }))
}
pub fn wordify(num: u64, thousandth_place: u32) -> Result<String, &'static str> {
    if num == 0 {
        return Ok("zero".to_string());
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
            hundreds_str
        } else {
            format!("{} {}", hundreds_str, place)
        }
    } else if thousandth_place == 0 {
        format!("{} {}", rest_str, hundreds_str)
    } else {
        format!("{} {} {}", rest_str, hundreds_str, place)
    })
}
