

fn main() {

    let names = vec!["Bob", "Frank", "Ferris"];
    
    // iter - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);


    // into_iter - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    // ERROR: names is moved
    // println!("names: {:?}", names);


    // iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.
    let mut firstname = vec!["Bob", "Frank", "Ferris"];

    for name in firstname.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("firstname: {:?}", firstname);
}
