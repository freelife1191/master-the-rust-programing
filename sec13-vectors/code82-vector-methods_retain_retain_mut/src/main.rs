fn retain_positive(x: &i32) -> bool {
    *x > 0
}

fn retain_and_modify_positives(x: &mut i32) -> bool {
    if *x > 0 {
        *x += 10;
        true
    } else {
        false
    }
}

fn main() {
    /*
    retain() and retain_mut()
    */
    let mut numbers = vec![-3, -2, -1, 0, 1, 2, 3];

    // This will retain only the positive numbers in the Vec
    // numbers.retain(|x| *x > 0);
    // numbers.retain(retain_positive);
    // numbers.retain_mut(retain_and_modify_positives); // Output: [11, 12, 13]
    numbers.retain_mut(|x| if *x > 0 {
        *x += 10;
        true
    } else {
        false
    });
    println!("{:?}", numbers); // prints: [1, 2, 3]

    // retain those elements whose length is less than 6
    let mut planets = vec!["Mercury", "Jupiter", "Earth", "Saturn", "Mars"];
    planets.retain(|planet| planet.len() < 6);
    println!("{:?}", planets); // prints: ["Earth", "Mars"]

    // retain those elements whose length is greater than 6
    // and modify the retained elements by appending length information
    // e.g: Hoth: 4
    let mut star_wars_planets = vec![
        String::from("Tatooine"),
        String::from("Coruscant"),
        String::from("Hoth"),
        String::from("Dagobah"),
        String::from("Mustafar"),
    ];

    star_wars_planets.retain_mut(|planet| {
        if planet.len() > 6 {
            *planet += &format!(": {}", planet.len());
            true
        } else {
            false
        }
    });
    println!("{:?}", star_wars_planets); // prints: ["Tatooine: 8", "Coruscant: 9", "Dagobah: 7", "Mustafar: 8"]
}
