fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for (i, x) in matrix.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            transposed[j][i] = *y;
        }
    }
    transposed
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}
