use std::io;
use std::rand::sample;
use std::rand::task_rng;

fn main() {

    //Input
    let mut iInput: (int,int);

    loop {
        match get_initial_input() {
            Some(t) => {
                iInput = t;
                break;
            },
            None => {
                println!("Invalid input");
                continue;
            },
        }
    }

    let size: int = iInput.val0();
    let sizeu: uint = size.to_uint().expect("Failed to parse integer.");

    let mut state: Vec<int> = Vec::from_elem(sizeu*sizeu, 1);
    let count: int = iInput.val1();
    let countu: uint = count.to_uint().expect("Failed to parse integer.");

    //Place mines.
    let mut rng = task_rng();
    let minePositions = sample(&mut rng, range(0,state.len()), countu);

    for c in minePositions.iter() {
        *state.get_mut(*c) = 2;
    }

    loop {

        //Draw
        print_state(state.slice(0,state.len()), sizeu);
        println!("Enter the cell to clear in a x,y format: ")
        
        //Input
        let input = get_input();
        let xy: (int, int) = match input {
            Some((x,y)) => (x-1,y-1),
            None => {
                println!("Make sure your input is correct.");
                continue;
            }
        };

        let pos: int = size * xy.val1() + xy.val0();
        let uPos = pos.to_uint().expect("Failed to parse pos to uint.");

        //Check rules.
        if !in_bounds(xy,size) {
            println!("That is not a valid cell.");
            continue;
        }

        match *state.get(uPos) {
            0 => println!("Already cleared!"),
            1 => { 
                *state.get_mut(uPos) = 0;
                println!("Cleared {0},{1}!", xy.val0()+1, xy.val1()+1);
            },
            2 => {
                println!("You hit a mine! GAMEOVER");
                break;
            }
            _ => println!("Error clearing mine.")
        }

        if check_win(&state) {
            println!("You have won!");
            break;
        }
    }
}

fn print_state(slice: &[int], width: uint) {
    for x in range(1, width*width + 1)
    {
        let state = slice.get(x-1).unwrap();
        match *state {
            0 => print!("  "),
            1 => print!(". "),
            2 => print!(". "),
            _ => print!("? "),
        }

        if x % width == 0 {
            print!("\n");
        }
    }
    print!("\n");
}

fn get_input() -> Option<(int,int)> {

    let result = io::stdin().read_line();
    let string = match result {
        Ok(s) => s,
        Err(_) => return None,
    };

    let mut splitResult = string.as_slice().split(',');
    let mut strings: Vec<&str> = Vec::new();

    for _ in range(0,2) {
        let st = match splitResult.next() {
            Some(s) => s,
            None => { 
                return None;
            }
        };
        strings.push(st);
    }

    let xs = *strings.get(0);
    let ys = *strings.get(1);

    let x = from_str::<int>(xs.trim()).unwrap();
    let y = from_str::<int>(ys.trim()).unwrap();

    let xy: (int,int)  = (x, y);

    Some(xy)
}

fn get_initial_input() -> Option<(int,int)>{

    println!("Input the size of the board (5 -= 5x5): ");
    let size: int =  match get_int() {
        Some(i) => i,
        None => return None,
    };

    println!("Input the number of mines: ");
    let count: int = match get_int() {
        Some(i) => i,
        None => return None,
    };

    if count > size*size {
        println!("Too many mines!");
        return None;
    }

    Some( (size, count) )
}

fn get_int() -> Option<int> {

    let result = io::stdin().read_line();
    let string = match result {
        Ok(s) => s,
        Err(_) => return None,
    };
    let final: int = match from_str::<int>(string.as_slice().trim()) {
        Some(s) => s,
        None => return None,
    };
    Some(final)
}

fn in_bounds(b: (int, int), width: int) -> bool {
    match b {
        (x,y) if y<0 || x<0 => return false,
        (x,y) if y>=width || x>=width => return false,
        (_,_) => return true
    }
}

fn check_win(state: &Vec<int>) -> bool {
    let mut win = true;

    for x in state.iter() {
        if  *x == 1 {
            win = false;
        }
    }
    win
}