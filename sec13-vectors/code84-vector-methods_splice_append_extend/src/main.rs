fn main() {
    /*
    splice()
    */
    // Primary temperature readings (in Celsius) over the last 10 hours
    let mut primary_readings = vec![22.5, 23.0, 22.8, 0.0, 0.0, 0.0, 23.2, 22.9, 22.4, 22.0];

    // Backup readings for hours 4 to 6
    let backup_readings = vec![22.7, 22.6, 23.0];

    // This will not compile because `splice` expects an iterator
    let faulty_readings: Vec<_> = primary_readings.splice(3..6, backup_readings).collect();

    println!("Corrected primary readings: {:?}", primary_readings);
    println!("Faulty readings: {:?}", faulty_readings);
}
