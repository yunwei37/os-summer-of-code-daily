// AsRef and AsMut allow for cheap reference-to-reference conversions.
// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
// and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

// I AM NOT DONE
// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound
/*
trait cn {
    fn byte_counter(self) -> usize;
}

trait cc{
    fn char_counter(self) -> usize;
}


impl cn for &str{
    fn byte_counter(self:&str) -> usize {
        self.as_ref().as_bytes().len()
    }
}


impl cn for String{
    fn byte_counter(self:String) -> usize {
        self.as_ref().as_bytes().len()
    }
}
// Obtain the number of characters (not bytes) in the given argument
// Add the AsRef trait appropriately as a trait bound
impl cc for &str{
    fn char_counter(self:&str) -> usize {
        self.as_ref().chars().count()
    }    
}

impl cc for String{
    fn char_counter(self:String) -> usize {
        self.as_ref().chars().count()
    }    
}
*/

fn byte_counter<T: std::convert::AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

fn char_counter<T: std::convert::AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

fn main() {
    let s = "Café au lait";
    println!("{}", char_counter(s));
    println!("{}", byte_counter(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }
}
