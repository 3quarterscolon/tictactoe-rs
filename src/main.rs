use std::io;

fn main() {
    let mut board = [' '; 9];
    let mut turn = true;
    for row in 1..4 {
        println!("\n   |   |");
        for col in 1..4 {
            print!(
                "{hori_line}{owner}{hori_line}{vert_line}", 
                owner = board[row * col - 1], 
                hori_line = if row != 3 { "_" } else { " " },
                vert_line = if col != 3 { "|" } else { "" }
            );
        }
    }

    println!(
        "\n{}'s turn.\nMove?",
        if turn {"X"} else {"O"}
    );

    let mut player_move = String::new();

    io::stdin()
        .read_line(&mut player_move)
        .expect("Input failed");
    
    // let player_move = [];
}
