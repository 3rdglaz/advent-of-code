use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .map(|line| {
            let mut iterator = line.chars().filter_map(|character| character.to_digit(10));
            let first: u32 = iterator.next().expect("tem que ser num");
            match iterator.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("numero carai")
        })
        .sum::<u32>();

    Ok(output.to_string())
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
