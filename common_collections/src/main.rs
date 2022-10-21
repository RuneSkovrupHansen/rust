use std::{collections::HashMap, fmt, vec};

fn main() {
    // vectors();
    // strings();
    // hash_maps();
    // vector_stats();
    // string_conversion();
    employee_chart();
}

fn vectors() {
    let v1: Vec<i32> = Vec::new();

    // Use of vec![] macro
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);
    v3.push(7);

    let index: usize = 2;

    let e1: &i32 = &v2[index];
    println!("The element of e1 is: {}", e1);

    let e2: Option<&i32> = v2.get(index);
    match e2 {
        Some(e2) => println!("The value of e2 is: {}", e2),
        None => println!("e2 does not have a value"),
    }

    // Using [] to get a value outside of a vector range causes the program to panic. Useful for when you want to program to actually crash.
    // Using get() will return None, forcing you to handle the outcome.

    // Borrowing from a vector, i.e. holding an immutable reference to an item, will prevent modifications of the vector to ensure that the reference is valid. It's not possible to have a mutable borrow while an immutable borrow is still active.

    // Note that event appending is prevented because of how the heap memory works - if the object needs to be moved in memory to a new location when an element is added, the existing reference would point to deallocated memory. This is prevented.

    print_vector(&v2);
    print_vector(&v3);

    for i in &mut v3 {
        // Use of dereferencing operator '*' to get value of reference
        *i += 10;
    }

    print_vector(&v3);

    // Though different types cannot be stored in a vector, different variants of an Enum can.
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let v4: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(15.12),
        SpreadsheetCell::Text(String::from("Blue")),
    ];
}

// Vectors are freed here since they go out of scope.

fn print_vector(v: &Vec<i32>) {
    for i in v {
        println!("{}", i);
    }
}

fn strings() {
    let s1 = String::from("initial contents");
    println!("{}", s1);

    let mut s2: String = String::from("foo");
    let s3: String = String::from("bar");
    s2.push_str(&s3);
    println!("{}", s2);

    // The '+' operation requires a non-borrowed value on the left, it also moves ownership, so that s2 is no longer usable. It's because of the methods which + references.
    let s4 = s2 + &s3;
    println!("{}", s4);

    let s5: String = String::from("tic");
    let s6: String = String::from("tac");
    let s7: String = String::from("toe");

    let s8: String = format!("{}-{}-{}", s5, s6, s7);

    // s8[10]; Compiler error.

    // Slices are valid
    let s9: &str = &s8[0..=2];
    println!("{}", s9); // Tic

    for c in s8.chars() {
        println!("{}", c)
    }
}

fn hash_maps() {
    // Not automatically brought into scope in the prelude
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name: String = String::from("team_name");

    // Use copied() to get copy instead of reference, use unwrap_or() to handle Option enum to provide default value.
    let team_score: i32 = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Types which implement the Copy trait will have values copied into the hashmap, other types will be moved into the hashmap, meaning that ownership is moved.

    let mut colors = HashMap::new();

    let key: String = String::from("favorite_color");
    let value: String = String::from("blue");

    colors.insert(key, value);

    // Invalid since value has been moved.
    // println!("{}", value);

    // Overwrite a value
    colors.insert(String::from("favorite_color"), String::from("red"));

    // Adding key and value if key is not present.
    colors
        .entry(String::from("favorite_color"))
        .or_insert(String::from("green"));

    // entry() returns an enum called Entry. API for checking if a key exists, and performing actions if it does not. or_insert returns mutable reference to value, if it does not exist, the value is inserted and a reference is returned.

    let mut word_count: HashMap<&str, i32> = HashMap::new();

    let sentence: &str = "hello world wonderful world";

    for word in sentence.split_whitespace() {
        // Get mutable reference to value and insert 0 if key does not exist.
        let count = word_count.entry(word).or_insert(0);
        // Use dereference operator to increment the value.
        *count += 1;
    }

    println!("{:?}", word_count);
}

fn vector_stats() {
    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

    let values: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 2, 6, 1, 3, 7, 3, 7, 2, 10, 20, 1];

    let average = calculate_average(&values);
    let median = calculate_median(&values);
    let mode = calculate_mode(&values);

    println!("average: {}, median: {}, mode: {}", average, median, mode);
}

fn calculate_average(vector: &Vec<i32>) -> f32 {
    let sum: i32 = vector.iter().sum();
    let count: i32 = vector.len() as i32;
    return sum as f32 / count as f32;
}

fn calculate_median(vector: &Vec<i32>) -> f32 {
    let mut vector = vector.clone();
    vector.sort();
    if vector.len() % 2 == 0 {
        let index = vector.len() as f32 / 2 as f32;
        let lower_index = (index.floor() - 1.0) as usize;
        let upper_index = index.ceil() as usize;
        (vector[lower_index] + vector[upper_index]) as f32 / 2 as f32
    } else {
        let index = vector.len() as f32 / 2 as f32;
        let index = index.floor() as usize;
        vector[index] as f32
    }
}

fn calculate_mode(vector: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    for value in vector {
        let count = count_map.entry(*value).or_insert(0);
        *count += 1;
    }

    // iter() to iterate through key and values.
    // max_by() get the max value by continuously comparing a and b, based on the 1. index, i.e. the value.
    // map() to return only the key of the pair.
    // unwrap() to handle the Option enum to return only the value or panic.

    let max = count_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();
    return *max;
}

fn string_conversion() {
    let string_normal = String::from("This is a normal string");
    let string_pig_latin = convert_to_pig_latin(&string_normal);

    println!("normal: {}", string_normal);
    println!("pig-latin: {}", string_pig_latin);
}

fn convert_to_pig_latin(string: &String) -> String {
    const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
    let mut string_pig_latin = String::from("");
    for word in string.split_whitespace() {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        if VOWELS.contains(&first_char) {
            string_pig_latin.push_str(&format!("{}-hay ", word))
        } else {
            for char in chars {
                string_pig_latin.push(char);
            }
            string_pig_latin.push_str(&format!("-{}ay ", first_char))
        }
    }

    return string_pig_latin.trim_end().to_string();
}

enum Department {
    Sales,
    Engineering,
    Marketing,
}

impl fmt::Display for Department {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Sales => "Persuasive purveyors",
                Self::Engineering => "Gadgeteers",
                Self::Marketing => "Displayers",
            }
        )
    }
}

fn employee_chart() {
    let mut employee_chart: HashMap<String, Department> = HashMap::new();

    employee_chart.insert(String::from("Sally"), Department::Engineering);
    employee_chart.insert(String::from("Azir"), Department::Sales);
    employee_chart.insert(String::from("Kefir"), Department::Marketing);
    employee_chart.insert(String::from("Nord"), Department::Engineering);

    // print_chart(&employee_chart);
    print_sorted_chart(&employee_chart);
}

fn print_chart(chart: &HashMap<String, Department>) {
    for (name, department) in chart {
        println!("name: {}, department: {}", name, department);
    }
}

fn print_sorted_chart(chart: &HashMap<String, Department>) {
    let mut name_sorted: Vec<(&String, &Department)> = chart.iter().collect();
    name_sorted.sort_by(|a, b| a.0.cmp(b.0));
    for (name, department) in &name_sorted {
        println!("name: {}, department: {}", name, department);
    }
}
