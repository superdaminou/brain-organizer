use crate::application::error::ApplicationError;

#[derive(PartialEq, Eq,Debug)]
pub struct Attribut(pub String,pub String);

impl TryFrom<&str> for Attribut {
    type Error = ApplicationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let splitted = value.trim().split_once("=").ok_or(ApplicationError::DefaultError("Could not parse Attribute: ".to_string() + value))?;
        Ok(Self(splitted.0.to_string(), splitted.1.to_string()))
    }
}

pub fn extract_attributes(value: &str) -> Result<Vec<Attribut>,ApplicationError> {
    if !value.to_string().contains("[") {
        return Ok(vec![]);
    }
    println!("{}", value);
    value.chars()
        .skip_while(|v| v != &'[')
        .skip(1)
        .take_while(|v| v != &']')
        .collect::<String>()
        .split(",")
        .filter(|l|!l.is_empty())
        .map(Attribut::try_from)
        .collect::<Result<Vec<Attribut>,ApplicationError>>()
}



#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_extract_attributes() {
        let combinations :Vec<(&str, Vec<Attribut>)> = vec![
            (" ata", vec![]),
            ("[test=a]", vec![Attribut("test".to_string(), "a".to_string())]),
            ("[test=a, another=toto]", vec![Attribut("test".to_string(), "a".to_string()), Attribut("another".to_string(), "toto".to_string())]),
            ("[test=a, another=toto] // with comment", vec![Attribut("test".to_string(), "a".to_string()), Attribut("another".to_string(), "toto".to_string())]),
            ("avec des choses avant [test=a, another=\"un ptit champs chelou\"] // with comment", vec![Attribut("test".to_string(), "a".to_string()), Attribut("another".to_string(), "\"un ptit champs chelou\"".to_string())]),
            ];
            
        combinations.iter().for_each(|combinaisons|{
             assert_eq!(extract_attributes(combinaisons.0).unwrap(), combinaisons.1)
            });
    }
}

    

