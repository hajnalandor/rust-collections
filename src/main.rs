fn main() {
    println!("Vectors:");
    vectors();
    hashmaps();
    strings();
}

// Vectors
fn vectors() {
    // creating a new vector Vec<T>
    let _v: Vec<i32> = Vec::new();    // we aren’t inserting any values into this vector, 
                                    // Rust doesn’t know what kind of elements we intend to store, we added a type notation
    // v.push(8);                      // cannot borrow `v` as mutable, as it is not declared as mutable
    let mut v: Vec<i32> = Vec::new();
    // updating a vector
    v.push(7);
    v.push(3);
    v.push(12);
    println!("{:?}",v);

    reading_elements_from_vec();
    iterating_over_vec();
    vector_storing_multiple_types();
} // v goes out of scope and is freed here, all of it's contents are also dropped

fn reading_elements_from_vec() {
    // another way to create vector
    let v = vec![1, 2, 3, 4, 5];

    // 1.) note. panic if it references a nonexistent element 
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // 2.) v.get(index) returns Option<Some(T),None>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    /*
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];  immutable borrow

        v.push(6);  mutable borrow

        println!("The first element is: {}", first);  immutable borrow used here
    */
}

fn iterating_over_vec() {
    let v = vec![100, 32, 57]; // immutable vector 
    
    for i in &v { // loop over immutable references
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57]; // mutable vector
    for i in &mut v { // loop over mutable reference
        *i += 50;  // dereferece iterator i
    }
}

fn vector_storing_multiple_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let _row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn hashmaps() {
    // Hashmap<K,V>  
    // it’s not included in the features brought into scope automatically, we need to import it
    
    use std::collections::HashMap;

    let mut scores = HashMap::new(); // create empty hashmap

    scores.insert(String::from("Blue"), 10);  // insert element
    scores.insert(String::from("Yellow"), 50); 


    // another way to construct hashmap
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let _scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // zip() returns a new iterator that will iterate over two other
    // iterators, returning a tuple where the first element comes from the
    // first iterator, and the second element comes from the second iterator.
    
    // collect()
    // Transforms an iterator into a collection.
    //
    // `collect()` can take anything iterable, and turn it into a relevant
    // collection.

    
    // ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}",field_name);   // error, hash map is the owner of field_name
    // for type that implement COpy trait(e.g. i32), the values are copied into the hash map


    // accessing values in hash map
    let field_name = String::from("Favorite color");
    let field_value = map.get(&field_name);  // get returns Option(&V)
    println!("{:?}",field_value);  // prints Some("Blue")
    println!("{:?}",field_value.unwrap());  // prints "Blue"

    // iterating over hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50); // or insert method defined on Entry returns a mutable reference to the value for the corresponding entry
    scores.entry(String::from("Blue")).or_insert(50);   
    println!("{:?}",scores.entry(String::from("Yellow")).or_default());
    println!("{:?}",scores.entry(String::from("Not exists")).or_default());

    // updating a value based on the old value
    println!("Updating value:");
    println!("{:?}",scores.entry(String::from("Not exists")).or_default());
    let count = scores.entry(String::from("Not exists")).or_insert(0);
    *count +=1 ;
    println!("{:?}",scores.entry(String::from("Not exists")).or_default());

    // Hashing function:
    /*
        By default, HashMap uses a “cryptographically strong”1 hashing function that can provide resistance to Denial of Service (DoS) attacks.
        This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.
        If you profile your code and find that the default hash function is too slow for your purposes,
        you can switch to another function by specifying a different hasher. 
        A hasher is a type that implements the BuildHasher trait.
    */
}

fn strings() {
    // strings are implemented as a collection of bytes
    // Rust has only one string type in the core language, which is the string slice str, UTF-8 encoded
    // The String type is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type

    // creating a new string
    let mut _s = String::new();
    let data = "initial contents"; // &str reference type

    let _s = data.to_string();
    
    // the method also works on a literal directly:
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // appending to a string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    println!("s1 is {}",s1);

    // concatenation with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // fn add(self, s: &str) -> String { ... }  self type is &str

    //format macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}",s);

    //indexing into strings
    // error
    // let s1 = String::from("hello");
    // let h = s1[0];
    
    println!("{}",String::from("Hola").len());
    println!("{}",String::from("Здравствуйте").len());
    
    let hello = "Здравствуйте";
    let answer = &hello[0..4]; //&hello[0..1] panic at runtime!!!
    println!("");
    println!("{}",answer);
    
    // methods for iterating over strings
    println!("iterating over chars");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    println!("iterating over bytes");

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }


}