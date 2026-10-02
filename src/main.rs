use console::{
    Key::{self, ArrowDown, ArrowLeft, ArrowRight, ArrowUp, Escape},
    Term,
};
use rand::prelude::*;

fn game_spawn(count: usize, game: &mut [Vec<u32>]) {
    let mut rnd = rand::rng();
    let mut counter = 0;

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
}

fn game_print(game: &[Vec<u32>]) {
    println!("┌────┬────┬────┬────┐");
    for (n, row) in game.iter().enumerate() {
        print!("│");
        for cell in row.iter() {
            if *cell == 0 {
                print!("    │");
            } else {
                print!("{:^4}│", cell);
            }
        }
        println!();
        if n != game.len() - 1 {
            println!("├────┼────┼────┼────┤");
        }
    }
    println!("└────┴────┴────┴────┘");
}

fn process_list(list: &mut Vec<u32>) {
    let list_len = list.len();
    // clear zeros
    list.retain(|&x| x != 0);

    // merge
    let mut result = Vec::new();
    let mut i = 0;
    while i < list.len() {
        if i + 1 < list.len() && list[i] == list[i + 1] {
            result.push(list[i] * 2);
            i += 2; // skip the merged entry
        } else {
            result.push(list[i]);
            i += 1;
        }
    }

    // fill with zeros
    result.resize(list_len, 0);

    *list = result;
}

fn game_move(game: &mut [Vec<u32>], key: Key) {
    let before = game.to_vec();
    match key {
        ArrowDown => {
            for column in 0..game[0].len() {
                // store column in new vector
                let mut list = Vec::new();
                for row in game.iter().rev() {
                    list.push(row[column]);
                }

                process_list(&mut list);
                list.reverse();

                for n in 0..list.len() {
                    game[n][column] = list[n];
                }
            }
        }
        ArrowUp => {
            for column in 0..game[0].len() {
                // store column in new vector
                let mut list = Vec::new();
                for row in game.iter() {
                    list.push(row[column]);
                }

                process_list(&mut list);

                // update game
                for n in 0..list.len() {
                    game[n][column] = list[n];
                }
            }
        }
        ArrowLeft => {
            for row in game.iter_mut() {
                // store row in new vector
                let mut list = Vec::new();
                for column in row.iter() {
                    list.push(*column);
                }

                process_list(&mut list);

                // update game
                *row = list;
            }
        }
        ArrowRight => {
            for row in game.iter_mut() {
                // store row in new vector
                let mut list = row.to_vec();
                list.reverse();

                process_list(&mut list);
                list.reverse();

                // update game
                *row = list;
            }
        }
        _ => {}
    }

    if before != game {
        game_spawn(1, game);
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

fn game_check_lost(game: &[Vec<u32>]) -> bool {
    let mut is_lost = true;
    for row in game.iter() {
        for cell in row.iter() {
            if *cell == 0 {
                is_lost = false;
            }
        }
    }

    for row in game.iter() {
        for n in 0..row.len() - 1 {
            if row[n] == row[n + 1] {
                is_lost = false;
            }
        }
    }

    for column in 0..game[0].len() {
        let mut list = Vec::new();
        for row in game.iter() {
            list.push(row[column]);
        }

        for n in 0..list.len() - 1 {
            if list[n] == list[n + 1] {
                is_lost = false;
            }
        }
    }
    is_lost
}

fn main() {
    let term = Term::stdout();

    let mut game = vec![vec![0; 4]; 4];
    game_spawn(2, &mut game);

    term.clear_screen().unwrap();
    game_print(&game);

    loop {
        match term.read_key() {
            Err(e) => println!("{}", e),
            Ok(key) => {
                if key == Escape {
                    break;
                } else {
                    game_move(&mut game, key)
                }
            }
        };

        term.clear_screen().unwrap();
        game_print(&game);

        if game_check_won(&game) {
            println!("Du krasser oberficker hast gewonnen!");
            break;
        } else if game_check_lost(&game) {
            println!("BWAHAHAHA nooooob");
            break;
        }
    }
}
