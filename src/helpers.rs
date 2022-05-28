use serde::Serializer;
use validator::ValidationError;

pub trait Stringify {
    fn stringify(&self) -> Option<String>;
}

impl Stringify for Option<i64> {
    fn stringify(&self) -> Option<String> {
        self.map(|c| c.to_string())
    }
}

pub fn option_stringify<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Stringify,
    S: Serializer,
{
    serializer.serialize_str(&*value.stringify().unwrap())
}

pub fn format_available_payment_method<T>(methods_slice: &[T]) -> String
where
    T: std::fmt::Display,
{
    let mut start = format!("{}", methods_slice.first().unwrap());
    for tipo in methods_slice.iter().skip(1) {
        start.push_str(&*format!(",{}", tipo));
    }
    start
}

pub fn validate_cpf(cpf: &str) -> Result<(), ValidationError> {
    let all_digits_repeated = [cpf.chars().next().unwrap()]
        .repeat(11)
        .into_iter()
        .collect::<String>();

    if cpf.len() != 11 || cpf == &*all_digits_repeated {
        let error = ValidationError {
            code: Default::default(),
            message: None,
            params: Default::default(),
        };
        return Err(error);
    };

    let mut first_sum = 0;
    let mut second_sum = 0;

    for (idx, number) in cpf.chars().map(|d| d.to_digit(10).unwrap()).enumerate() {
        second_sum += number * (11 - idx as u32);
        if idx == 9 {
            break;
        };
        first_sum += number * (10 - idx as u32);
    }

    let first_validator = cpf.chars().nth(9).and_then(|d| d.to_digit(10)).unwrap();
    let second_validator = cpf.chars().nth(10).and_then(|d| d.to_digit(10)).unwrap();

    if first_sum * 10 % 11 != first_validator || second_sum * 10 % 11 != second_validator {
        let error = ValidationError {
            code: Default::default(),
            message: None,
            params: Default::default(),
        };

        return Err(error);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_cpf_valid() {
        let res = validate_cpf("41810420814");
        assert!(res.is_ok());
    }
    #[test]
    fn t_cpf_invalid() {
        let res = validate_cpf("11111111111");
        assert!(res.is_err());
    }
}
