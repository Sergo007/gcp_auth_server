use tracing::*;

pub fn token_transformation(token: &str) -> String {
    let t = token.to_string();
    let tl = token.replace("\r", "\n");
    let token: Vec<&str> = tl.split('\n').collect();
    let a = token.get(0).unwrap_or_else(|| {
        error!("Error: 'token_transformation'  {}", t);
        &""
    });
    a.to_string()
}

//region *** tests function here ***
#[cfg(test)]
mod token_transformation_tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_token_transformation1() {
        let template = read_to_string("print_access_token1.txt").expect("Unable to read file");
        assert_eq!(token_transformation(&template), "ya29.a0AfB_byDa6dKWgNYynhBsG32-BE1PhdvwJrOCufdsazwBH0W-8ip11SA9ak3L8eI3bmMFycOx2gpRNSJYfEWRUZl6-dBJRifph6Jdl7-bZRJQEeZHFOfdsaJr8gp2yE2CLvJqTqS3fdsabkvzFaCgYKAfdsadf8h5rouS11007H_g0179".to_string());
    }
    #[test]
    fn test_token_transformation2() {
        let template = "sss\n";
        assert_eq!(token_transformation(&template), "sss".to_string());
    }
    #[test]
    fn test_token_transformation3() {
        let template = "sss\n\n\r";
        assert_eq!(token_transformation(&template), "sss".to_string());
    }
    #[test]
    fn test_token_transformation4() {
        let template = "sss";
        assert_eq!(token_transformation(&template), "sss".to_string());
    }
    #[test]
    fn test_token_transformation5() {
        let template = "sss\r";
        assert_eq!(token_transformation(&template), "sss".to_string());
    }
}
