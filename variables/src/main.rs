fn main() {
    {
        let s1 = String::from("Hello, world!");
        let s2 = s1; // Ownership of `s1` is moved to `s2`
        // println!("s1: {}", s1); // This line would cause a compile error because `s1` is no longer valid
        println!("s2: {}", s2); // This is valid, `s2` owns the string now
    }

    {
        let s1 = String::from("Hello, world!");
        let s2 = s1.clone(); // `s2` is a clone of `s1`, both can be used independently
        println!("s1: {}", s1); // This is valid, `s1` is still valid
        println!("s2: {}", s2); // This is also valid, `s2` owns its own copy of the string
    }

    {
        let s = String::from("Hello, world!");
        string_length(s); // The ownership of `s` has been moved to the function `string_length`
        // println!("The length of the string is: {}", s.len()); // This line would cause a compile error
    }

    {
        let s = String::from("Hello, world!");
        let s = string_length_return(s); // The ownership of `s` is moved to the function and then returned
        println!("The length of the string is: {}", s.len()); // This is valid, `s` is now valid again
    }

    {
        let s = String::from("Hello, world!");
        let len = calc_length(&s); // `s` is still valid, we can use it to get the length
        println!("The length of the string is: {}", len); // This is valid, `s` is still valid
        println!("s: {}", s); // This is also valid, `s` is still valid
    }

    {
        let mut s = String::from("Hello,");
        append_word(&mut s); // We pass a mutable reference to `s`
        println!("s after appending: {}", s); // This is valid, `s` has been modified
    }

    {
        let mut s = String::from("Hello,");
        let r1 = &mut s; // Create a mutable reference to `s`
        println!("r1: {}", r1); // This is valid, we can use `r1` to access `s`
        let r2 = &mut s; // This line would cause a compile error because you cannot have two mutable references to the same data at the same time
        println!("r2: {}", r2); // Uncommenting this line would cause a compile error
    }

    {
        let s = String::from("Hello, world!");
        let word = first_word(&s);
        println!("The first word is: {}", word); // This is valid, `s` is still valid
    }
}

fn string_length(s: String) {
    println!("The length of the string is: {}", s.len());
}

fn string_length_return(s: String) -> String {
    println!("The length of the string is: {}", s.len());
    s // Returning the string back to the caller
}

fn calc_length(s: &String) -> usize {
    s.len() // This function borrows `s`, so `s` remains valid after the function call
}

fn append_word(s: &mut String) {
    s.push_str(" world!"); // This function modifies the string in place
    println!("Modified string: {}", s); // Print the modified string
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // Convert the string to bytes
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // Check for space character
            return &s[0..i]; // Return the slice up to the first space
        }
    }
    &s[..] // If no space is found, return the whole string
}