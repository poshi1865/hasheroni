use hasheroni::Hasheroni;

fn main() {
    let vector: Vec<(u32, String)> = vec![
        (0, String::from("Nice")),
        (1, String::from("Plants")),
        (2, String::from("Guitar"))
    ];
    let mut hm: Hasheroni<u32, String> = Hasheroni::from(vector);

    for i in 0..3 {
        println!("{}", hm.get(i).unwrap());
    }
}
