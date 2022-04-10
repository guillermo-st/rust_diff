mod lcs;

fn main() {
    let hello1 = vec![
        String::from("hello"),
        String::from("horrible"),
        String::from("world"),
        String::from("!"),
    ];
    let hello2 = vec![
        String::from("hello"),
        String::from("beautiful"),
        String::from("world"),
        String::from("!"),
    ];

    let a_grid = lcs::Lcs::new(&hello1, &hello2);
    println!("{}", a_grid);

    a_grid.diff();

}
