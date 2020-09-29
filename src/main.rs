use std::env;
use std::fs;
use std::io;
use std::time::SystemTime;


fn process_directory(dir:String, diff:u64) -> io::Result<()>{
    for entry in fs::read_dir(dir)?{
        let metadata = entry?;
        let path = metadata.path();
        let metadata = metadata.metadata()?;
        match metadata.accessed(){
            Ok(a) => {
                let is_dir = metadata.is_dir();
                let difference = a.elapsed().expect("Clock Error");
                if difference.as_secs() > diff{
                    if is_dir {
                        fs::remove_dir_all(path)?;
                    }
                    else{
                        fs::remove_file(path)?;
                    }                   
                }
            },
            Err(a) => {
                println!("Unable to read last access time on file. Windows needs to be explicitly configured to save this information.");
                println!("{}",a);
            }
        }
    }
    Ok(())
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Err: No directory provided.");
    }
    if args.len() < 3 {
        println!("Err: No interval quantity provided.");
    }
    if args.len() < 4 {
        println!("Err: No interval provided.");
        println!("The command should be formatted as: \"compactor directory quantity interval");
        println!("Example: compactor ./downloads 2 weeks");
        return;
    }
    


    let target_dir = &args[1];
    let quantifier = &args[2];
    let interval = &args[3];
    let diff:u64;
    let quantifier:u64 = match quantifier.parse::<u64>(){
        Ok(a) => a,
        Err(a) => {
            println!("Non-numeric quantifier \"{}.\" Must be an integer.",a);
            return;
        }
    };
    diff = match &interval[..]{
        "minute" | "minutes" => 60,
        "hour" | "hours" => 60 * 60,
        "day" | "days" => 60 * 60 * 24,
        "week" | "weeks" => 60 * 60 * 24 * 7,
        a => {
            println!("Unrecognized interval \"{}\". Must be one of \"minutes, hours, days or weeks\"",a);
            return;
        }
    };
    let diff = diff * quantifier;
    println!("{}", diff);
    match process_directory(target_dir.to_string(), diff){
        Ok(_) => {},
        Err(a) => println!("{}",a)
    }    
}
