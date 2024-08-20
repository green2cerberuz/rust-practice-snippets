

fn main() {
    let (mut fahr, mut celsius): (i32, i32);
    let (lower, upper, step) = (0, 300, 20);

    fahr = lower;
    
    while fahr <= upper {
        celsius = 5 * (fahr-32) /9;
        println!("{}\t{}\n", fahr, celsius);
        fahr +=step;
    }
}