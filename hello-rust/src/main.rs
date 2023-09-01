
#[cfg(not(panic = "abort"))]
fn handlePanic() {
    println!("We will handle this Error");
}

#[cfg(panic = "abort")]
fn handlePanic() {
    panic!("Error occurred!");
}

fn person(name: Option<&str>) {
    // unwrap
    // let unwrapped_name = name.unwrap(); // 

    // if unwrapped_name =="Shubham" { 
    //     handlePanic();
    // }
    
    // println! (" Hello {} ", unwrapped_name);

    // unwrap
    match name {
        Some("Shubham") => handlePanic(),
        Some(inner) => println! (" Hello {} ", inner),
        None => panic!("Value cannot be null"),
    }
}

fn main() {
    person(Some("abc"));
    person(Some("Shubham")); 
    person(None); 
    
}