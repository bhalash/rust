use std::collections::HashMap;

#[derive(Debug)]
struct Puppy {
    name: String,
}

fn main() {
    let jonsey = Puppy { name: "Jonsey".to_string() };
    let buddy = Puppy { name: "Buddy".to_string() };
    let pearl = Puppy { name: "Pearl".to_string() };

    let mut puppies = HashMap::new();

    puppies.insert(&jonsey.name, &jonsey);
    puppies.insert(&buddy.name, &buddy);
    puppies.insert(&pearl.name, &pearl);

    let kiki = Puppy { name: "Kiki".to_string() };
    puppies.entry(&kiki.name).or_insert(&kiki);

    for (_, puppy) in puppies.into_iter() {
        println!("{:?}", puppy);
    }
}
