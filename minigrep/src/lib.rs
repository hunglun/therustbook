use std::fs;

pub struct Config {
    query : String,
    filename : String,
}

pub fn run(config : Config) {

    let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong reading the file");
    let query =  config.query;
    search_query_case_sensitive(&query, &contents);
    
}

pub fn parse_config(args : &Vec<String>) -> Config {
    Config {
        query:args[1].clone(),
        filename:args[2].clone(),
    }
}

fn search_query_case_sensitive<'a>(query : &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            println!("{}", line);
            result.push(line);
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hello() {
        assert_eq!(true,true);
    }
    #[test]
    fn test_search_query_case_sensitive(){
        assert_eq!(vec!["the moon","        the sun"],
        search_query_case_sensitive("the", r"the moon
        The earth
        the sun
        "))
    }
}