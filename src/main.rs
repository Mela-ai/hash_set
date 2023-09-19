/***********************************************************************************
 * The HashSet implements the set data structure in Rust. Just like a set, it allows
 * for the storage of values without duplicates.
 * HashSet is part of the Rust standard collections library, so it must be included
 * in the source file. 
 ***********************************************************************************/

use std::collections::HashSet;

// create a HashSet
fn define_hashset() {
    let color: HashSet<String> = HashSet::new();
    println!("HashSet = {:?}", color);
    println!();
}

// HashSet Operation: insert values `insert()` method
fn add_values() {
    // HashSet variable must be declared mutable to insert values
    let mut colors: HashSet<&str> = HashSet::new();

    // insert values
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");
    colors.insert("Purple");

    // values are printed in different order
    println!("colors = {:?}", colors);
}

// HashSet Operation: check if value is Present in the HashSet
fn check_values() {
    let mut colors: HashSet<&str> = HashSet::new();

    // insert values
    colors.insert("Red");
    colors.insert("Yellow");

    println!("colors = {:?}", colors);

    // check for a value in a HashSet
    if colors.contains("Red") {
        println!("We have the color \"Red\" in the HashSet,"); 
    }
    println!();
}

// HashSet operator: Remove values 
fn remove_values() {
    let mut colors: HashSet<&str> = HashSet::new();

    // insert elements
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");
    println!("colors before remove operation = {:?}", colors);

    // remove elements
    colors.remove("Red");
    println!("colors after remove opeeation = {:?}", colors);
    println!();
}

// HashSet Operator: Iterate over values 
fn iterate_values() {
    let mut colors: HashSet<&str> = HashSet::new();

    // insert elements
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    // iterate over the HashSet
    for color in colors {
        // print each value
        println!("{}", color);
    }
    println!();
}

// HashSet Operation: Hashset with default values
fn default_values() {
    let numbers = HashSet::from([2, 7, 8, 10]);
    println!("numbers = {:?}", numbers);
    println!();
}

// Set Operations: union of two sets
fn set_union() {
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2: HashSet<i32> = HashSet::from([1, 2, 7]);

    // union of hashsets, make reference to hashset2
    let result: HashSet<_> = hashset1.union(&hashset2).collect();

    // duplicate values are omitted
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("union = {:?}", result);
    println!();
}

// Set Operations: intersection of two sets
fn set_intersection() {
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7]);

    // set intersection
    let result: HashSet<_> = hashset1.intersection(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("intersection = {:?}", result);
    println!();
}

// Set Operations: difference between two sets
fn set_difference() {
    let hashset1 = HashSet::from([1, 2, 3, 4]);
    let hashset2 = HashSet::from([4, 3, 2]);

    // difference between HashSet
    let result: HashSet<_> = hashset1.difference(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("difference = {:?}", result);
    println!();
}

// Set Operations: symmetric difference between two sets
fn symmetric_difference() {
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7, 9]);

    // symmetric difference
    let result: HashSet<_> = hashset1.symmetric_difference(&hashset2).collect();
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("symmetric difference = {:?}", result);
    println!();
}
fn main() {
    define_hashset();       // define a new HashSet
    add_values();           // insert values into HashSet
    check_values();         // check values in the HashSet
    
    remove_values();        // remove elements from a HashSet
    iterate_values();       // iterate values in the HashSet
    default_values();       // Hashset with default values
    
    set_union();            // union of two sets 
    set_intersection();     // intersection of two sets
    set_difference();       // difference between two sets
    symmetric_difference(); // symmetric difference betweeen two sets 
}
