// Yes, yes, we know. It's an exercise, compiler, we want it that way!
#[allow(unused_mut)]

fn main() {
    // 1. Uncomment the code below. Create a closure that returns the square of an integer (the
    // number multiplied by itself), and assign the closure to the "square" variable. Then run the
    // code and make sure it works.

    let square = |x: i32| x * x;
    println!("5 squared is {}\n", square(5));

    // 2. Uncomment the code below.  Finish the .map() iterator adaptor call by passing it a closure
    // which takes a tuple of two integers as a parameter, and returns a tuple with the first
    // integer incremented by 1, and the second integer left alone.  For example, if given the input
    // (0, 1), it should return (1, 1). Run the code and make sure it works.

    // println!("Part 2 Result");
    let pairs = vec![(0, 1), (2, 3), (4, 5)];
    // pairs
    //     .into_iter()
    //     .map(|(x, y)| (x + 1, y))
    //     .for_each(|t| println!("{:?}", t));
    // println!();

    // 3. Uncomment the code below. There is a mutable vector named `numbers`. Use an iterator over
    // mutable references to multiply each of the values in `numbers` by 3.
    // Hint 1: You'll need .iter_mut() -- bonus points if you use the shorter, syntactic sugar form!
    // Hint 2: `x` will be a mutable reference, so remember to dereference it to use it

    println!("Part 3 Result");
    let mut numbers = vec![1, 2, 3, 4];
    for x in numbers.iter_mut() {
        *x *= 3; // multiply the value by 3 via the mutable reference x
    }
    println!("{:?}", numbers); // should print [3, 6, 9, 12]
    println!();

    // 4. Uncomment the code below.  Take the vector of words and
    // - Convert the vector into an iterator with .into_iter()
    // - Use .filter() to remove any word that contains the letter "h" -- use .contains()
    // - Use .map() to convert all the words to uppercase -- use .to_uppercase()
    // - Use .collect() to put the transformed words back into a vector
    //
    // Hint: .to_uppercase() is a method on `str` which returns a String

    let words = vec!["autobot", "beach", "car", "decepticon", "energon", "frothy"];
    let transformed: Vec<_> = words
        .into_iter()
        .filter(|x| !x.contains("h"))
        .map(|x| x.to_uppercase())
        .collect();
    println!("Transformed: {:?}", transformed);

    // Challenge:
    //
    // - Rewrite the code in #2 as a for loop
    // Expected: (1, 1), (3, 3), (5, 5)
    println!("Part 2 Result");
    let mut mapped_pairs = vec![];
    for (x, y) in pairs.into_iter() {
        mapped_pairs.push((x + 1, y));
    }

    for t in mapped_pairs.iter() {
        println!("{:?}", t);
    }
    println!();

    // - Rewrite the code in #3 in functional style (without a for loop).  Hint: There are multiple
    // ways to accomplish this, but they all end with an iterator consumer.
    //  Expected: [3, 6, 9, 12]
}
