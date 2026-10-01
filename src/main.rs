use console::{
    Key::{self, ArrowDown, ArrowLeft, ArrowRight, ArrowUp},
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
    println!("_____________________");
    for row in game.iter() {
        print!("|");
        for cell in row.iter() {
            if *cell == 0 {
                print!("    |");
            } else {
                print!("{:^4}|", cell);
            }
        }
        println!();
        println!("---------------------");
    }
}

// TODO: Use filter to get rid of all zeros -> automatically moves everything to one side. Then
// resize with zeros
fn game_move(game: &mut [Vec<u32>], key: Key) {
    let before = game.to_vec();
    match key {
        ArrowDown => {
            for column in 0..game[0].len() {
                // store column in new vector
                let mut list = Vec::new();
                for row in game.iter() {
                    list.push(row[column]);
                }

                // move everything down
                for _ in 0..list.len() {
                    for n in (0..list.len() - 1).rev() {
                        if list[n] > 0 && list[n + 1] == 0 {
                            list[n + 1] = list[n];
                            list[n] = 0;
                        }
                    }
                }

                // merge
                for n in (1..list.len()).rev() {
                    if list[n] == list[n - 1] {
                        list[n] *= 2;
                        list[n - 1] = 0;
                    }
                }

                // move everything down
                for _ in 0..list.len() {
                    for n in (0..list.len() - 1).rev() {
                        if list[n] > 0 && list[n + 1] == 0 {
                            list[n + 1] = list[n];
                            list[n] = 0;
                        }
                    }
                }

                // update game
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

                // move everything up
                for _ in 0..list.len() {
                    for n in 1..list.len() {
                        if list[n] > 0 && list[n - 1] == 0 {
                            list[n - 1] = list[n];
                            list[n] = 0;
                        }
                    }
                }

                // merge
                for n in 0..list.len() - 1 {
                    if list[n] == list[n + 1] {
                        list[n] *= 2;
                        list[n + 1] = 0;
                    }
                }

                // move everything up
                for _ in 0..list.len() {
                    for n in 1..list.len() {
                        if list[n] > 0 && list[n - 1] == 0 {
                            list[n - 1] = list[n];
                            list[n] = 0;
                        }
                    }
                }

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

                // move everything left
                for _ in 0..list.len() {
                    for n in 1..list.len() {
                        if list[n] > 0 && list[n - 1] == 0 {
                            list[n - 1] = list[n];
                            list[n] = 0;
                        }
                    }
                }

                // merge
                for n in 0..list.len() - 1 {
                    if list[n] == list[n + 1] {
                        list[n] *= 2;
                        list[n + 1] = 0;
                    }
                }

                // update game
                row[..list.len()].copy_from_slice(&list[..]);
            }
        }
        ArrowRight => {
            for row in game.iter_mut() {
                // store row in new vector
                let mut list = Vec::new();
                for column in row.iter() {
                    list.push(*column);
                }

                // move everything right
                for _ in 0..list.len() {
                    for n in (0..list.len() - 1).rev() {
                        if list[n] > 0 && list[n + 1] == 0 {
                            list[n + 1] = list[n];
                            list[n] = 0;
                        }
                    }
                }

                // merge
                for n in (1..list.len()).rev() {
                    if list[n] == list[n - 1] {
                        list[n] *= 2;
                        list[n - 1] = 0;
                    }
                }

                // move everything right
                for _ in 0..list.len() {
                    for n in (0..list.len() - 1).rev() {
                        if list[n] > 0 && list[n + 1] == 0 {
                            list[n + 1] = list[n];
                            list[n] = 0;
                        }
                    }
                }

                // update game
                row[..list.len()].copy_from_slice(&list[..]);
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

// TODO: New lost logic
fn main() {
    let term = Term::stdout();

    let mut game = vec![vec![0; 4]; 4];
    game_spawn(2, &mut game);

    term.clear_screen().unwrap();
    game_print(&game);

    let lost = false;

    loop {
        match term.read_key() {
            Err(e) => println!("{}", e),
            Ok(key) => game_move(&mut game, key),
        };

        term.clear_screen().unwrap();
        game_print(&game);

        if game_check_won(&game) {
            break;
        }
    }
    if lost {
        println!("BWAHAHAHA nooooob");
    } else {
        println!("Du krasser oberficker hast gewonnen!");
    }
}
