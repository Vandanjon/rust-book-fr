// ANCHOR: here
trait Pilote {
    fn voler(&self);
}

trait Magicien {
    fn voler(&self);
}

struct Humain;

impl Pilote for Humain {
    fn voler(&self) {
        println!("Ici le capitaine qui vous parle.");
    }
}

impl Magicien for Humain {
    fn voler(&self) {
        println!("Décollage !");
    }
}

impl Humain {
    fn voler(&self) {
        println!("*agite frénétiquement ses bras*");
    }
}
// ANCHOR_END: here

fn main() {}
