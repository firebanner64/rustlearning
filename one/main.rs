
fn main() {
    // let mut text = String::new(); 

    // std::io::stdin().read_line(&mut text).unwrap();

    // let mut game = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];

    // initialize tic-tac-toe board
    let mut game: Vec<Vec<String>> = vec![vec![String::new(); 3]; 3];
    let len = game.len() * game[0].len();
    let mut istr: String;
    for i in 0..len {
        istr = (i + 1).to_string();
        game[i / 3][i % 3] = istr;
    }

    let mut inp: String;

    loop {
        render_board(&game);
        
        inp = String::new();
        read_line(&mut inp);
        if inp.len() != 2 {
            eprintln!("Error: 3");
            continue;
        }
        if !check_numeric(&inp) {
            eprintln!("Error: 2");
            continue;
        }

        let b2 = &inp[0..(inp.len() - 1)];

        // let loc = inp.parse::<usize>().unwrap();

        let loc = b2.parse::<usize>().unwrap() - 1;
        let row = loc / 3;
        let col = loc % 3;

        game[row][col] = " ".to_owned();

        
    }

}

fn render_board(board: &Vec<Vec<String>>) {
    for i in 0..board.len() {
        println!("   |   |   ");
        println!(" {} | {} | {} ", board[i][0], board[i][1], board[i][2]);
        println!("   |   |   ");

        if i != 2 {
            println!("-----------");
        }
    }
}

fn read_line(dest: &mut String) {
    std::io::stdin().read_line(dest).unwrap();
}

fn check_numeric(dest: &String) -> bool {
    let arr: Vec<char> = dest.chars().collect();
    let slice = &arr[0..(arr.len() - 2)];

    // let end = arr.len() - 2;

    for val in slice.iter() {
        if !val.is_numeric() {
            return false;
        }
    }

    return true;
}