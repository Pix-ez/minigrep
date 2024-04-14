use std::fs;
use std::error::Error;


pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    let file_buffer = fs::read_to_string(config.filename)?;
    // println!("{}",file_buffer);
    if config.case_sensitive{
        for line in search_case_senstive(&config.query, &file_buffer){
            println!("{}", line)
        }

    }else {
        for line in search_case_insenstive(&config.query, &file_buffer){
            println!("{}", line)
        }
    }
    
    Ok(())
}


#[derive(Debug)]
pub struct Config{

    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}

//let file_buffer = fs::read_to_string(filename).expect("Something went wrong while reading file specified");

//println!("{}", file_buffer);



impl Config{

    pub fn new(args:&[String])->Result<Config, &str>{

       

        if args.len() < 3{
            if args.iter().any(|arg| arg == "-help") {
                return Err("\
                How to use minigrep- filename.txt search_phrase -c(option to enable case sensitive) ");

            }else if args.iter().any(|arg| arg == "-version" ) {
                return Err("\
                𝗠𝗶𝗻𝗶𝗴𝗿𝗲𝗽 𝘃𝟬.𝟬.𝟭 [𝗺𝗮𝗱𝗲 𝗯𝘆 𝗥𝗮𝗵𝘂𝗹]"); 
                
            }else{
                return Err("\
                How to use minigrep- filename.txt search_phrase -c(option to enable case sensitive) ");

            }
            
        }
        let mut case_sensitive =false;

        if args.len()>3{
            if args[3]=="-c"{
                case_sensitive=true;
            }
        }

       
         // Sample vector of &str
        // let my_args: Vec<&str> = vec!["-help", "-version", "-info"];

        // Check if any element of args matches "-help"
        
        
       
        Ok(Config{
            filename:args[1].clone(),
            query:args[2].clone(),
            case_sensitive

        })
    }

    
}

pub fn search_case_senstive<'a>(query:& str, file_buffer:&'a str)->Vec<&'a str>{
    let mut results= Vec::new();
    
    for line in file_buffer.lines(){
        if line.contains(query){
            results.push(line);
           
        }

    }
    results
    

}

pub fn search_case_insenstive<'a>(query:& str, file_buffer:&'a str)->Vec<&'a str>{
    let mut results= Vec::new();
    let query = query.to_lowercase();
    
    for line in file_buffer.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
           
        }

    }
    results
    

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "The";
        let file_buffer= "\
        The car is fast.
        this is so big";
        // let res =search(query,file_buffer);
        // println!("{:?}", res);
        assert_eq!(vec!["The car is fast."], search_case_senstive(query, file_buffer));
    }

    #[test]
    fn case_insensitive(){
        let query = "The";
        let file_buffer= "\
        The car is fast.
        this is so big";
        // let res =search(query,file_buffer);
        // println!("{:?}", res);
        assert_eq!(vec!["The car is fast."], search_case_insenstive(query, file_buffer));
    }

}