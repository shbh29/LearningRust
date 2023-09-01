fn drink(bevarage: &str) {
    // don't drink sugary bevarage
    if bevarage == "Lemonade" {
        if cfg!(panic="abort") { println!("Not Your Party.. Run!!");}
        else { println!("Spit it out!!");}
    }
    println! ("Some refreshin {} is all I need", bevarage);
}
fn main() {
    drink("water");
    drink("Lemonade");
}
