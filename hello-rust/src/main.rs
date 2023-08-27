fn drink(bevarage: &str) {
    // don't drink sugary bevarage
    if bevarage == "Lemonade" {
        panic! ("Aaa");
    }
    println! ("Some refreshin {} is all I need", bevarage);
}
fn main() {
    drink("water");
    drink("Lemonade");
}
