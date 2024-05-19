use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // https://stackoverflow.com/questions/27996430/reversing-a-string-in-rust
    let drow: String = input.graphemes(true).rev().collect();
    return drow;
}

pub fn main() {
    reverse("sinep")
}
