use console::{
    Key::{self, ArrowDown, ArrowLeft, ArrowRight, ArrowUp},
    Term,
};
use rand::prelude::*;

fn game_spawn(count: usize, game: &mut [Vec<u32>]) -> bool {
    let mut rnd = rand::rng();
    let mut counter = 0;

    let mut is_full = true;
    for row in game.iter() {
        for cell in row.iter() {
            if *cell == 0 {
                is_full = false;
            }
        }
    }

    if is_full {
        return false;
    }

    while counter < count {
        for row in game.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == 0 && counter < count && rnd.random::<f32>() > 0.9 {
                    *cell = 2;
                    counter += 1;
                }
            }
        }
    }
    true
}

fn game_print(game: &[Vec<u32>]) {
    for row in game.iter() {
        for cell in row.iter() {
            if *cell == 0 {
                print!("____|");
            } else {
                print!("{:4}|", cell);
            }
        }
        println!();
    }
}

// FIXME: change to two step togic: first move all, then merge, then move again
// FIXME: Only spawn a number if anything moved within a move
fn game_move(game: &mut [Vec<u32>], key: Key) -> bool {
    match key {
        ArrowDown => {
            // go through every column
            for column in 0..game[0].len() {
                // repeat 4 times
                for _ in 0..game.len() {
                    // go up the rows, starting at the second last one
                    for n in (0..game.len() - 1).rev() {
                        // check if a cell is bigger than 0 and if the cell below is empty
                        if game[n][column] > 0 && game[n + 1][column] == 0 {
                            game[n + 1][column] = game[n][column];
                            game[n][column] = 0;
                        // check if the cell below has the same value as the current cell and merge
                        } else if game[n][column] > 0 && game[n + 1][column] == game[n][column] {
                            game[n + 1][column] *= 2;
                            game[n][column] = 0;
                        }
                    }
                }
            }
            game_spawn(1, game)
        }
        ArrowUp => {
            // go through every column
            for column in 0..game[0].len() {
                // repeat 4 times
                for _ in 0..game.len() {
                    // go down the rows, starting at the second row
                    for row in 1..game.len() {
                        // check if the cell is bigger than 0 and if the cell above is empty
                        if game[row][column] > 0 && game[row - 1][column] == 0 {
                            game[row - 1][column] = game[row][column];
                            game[row][column] = 0;
                        // check if the cell above has the same calue as the current cell and merge
                        } else if game[row][column] > 0
                            && game[row - 1][column] == game[row][column]
                        {
                            game[row - 1][column] *= 2;
                            game[row][column] = 0;
                        }
                    }
                }
            }
            game_spawn(1, game)
        }
        ArrowLeft => {
            for row in 0..game.len() {
                for _ in 0..game.len() {
                    for column in 1..game[0].len() {
                        if game[row][column] > 0 && game[row][column - 1] == 0 {
                            game[row][column - 1] = game[row][column];
                            game[row][column] = 0;
                        } else if game[row][column] > 0
                            && game[row][column - 1] == game[row][column]
                        {
                            game[row][column - 1] *= 2;
                            game[row][column] = 0;
                        }
                    }
                }
            }
            game_spawn(1, game)
        }
        ArrowRight => {
            for row in 0..game.len() {
                for _ in 0..game.len() {
                    for column in (0..game[0].len() - 1).rev() {
                        if game[row][column] > 0 && game[row][column + 1] == 0 {
                            game[row][column + 1] = game[row][column];
                            game[row][column] = 0;
                        } else if game[row][column] > 0
                            && game[row][column + 1] == game[row][column]
                        {
                            game[row][column + 1] *= 2;
                            game[row][column] = 0;
                        }
                    }
                }
            }
            game_spawn(1, game)
        }
        _ => false,
    }
}

fn game_check_won(game: &[Vec<u32>]) -> bool {
    for row in game.iter() {
        for cell in row.iter() {
            if *cell == 2048 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let term = Term::stdout();

    let mut game = vec![vec![0; 4]; 4];
    game_spawn(2, &mut game);

    term.clear_screen().unwrap();
    game_print(&game);

    let lost;

    loop {
        match term.read_key() {
            Err(e) => println!("{}", e),
            Ok(key) => {
                if !game_move(&mut game, key) {
                    lost = true;
                    break;
                }
            }
        };
        term.clear_screen().unwrap();
        game_print(&game);
        if game_check_won(&game) {
            lost = false;
            break;
        }
    }
    if lost {
        println!("BWAHAHAHA nooooob");
    } else {
        println!("Du krasser oberficker hast gewonnen!");
    }
}
