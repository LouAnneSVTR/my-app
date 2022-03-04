use fst_wasm::{FiniteStateMachine, Node, Transition};

use std::fmt;
use std::fmt::Formatter;

fn main() {

    let mut test = FiniteStateMachine::new();
    let name = 0;
    let name2 = 1;
    let name3 = 2;

    test.addNode(name);
    test.addNode(name2);
    test.addNode(name3);

    test.addTransition('a',0,1);
    test.addTransition('b',0,2);
    test.addTransition('c',1,2);
    test.addTransition('d',2,2);

    test.displayFSM();
    test.displayTransition()

}

/*
fn main() {
    let mut vec = init_vector2d(4);
    println!("Initialisation : Vector");
    print_vec(vec.clone());

    //vec = rng_automate_pattern(vec);
    vec = rng_given_pattern(vec);
    println!("Initialisation : Transistions");
    print_vec(vec.clone());


    print!("Entrez le mot à tester :  ");

    let mut stdin = stdin();
    let mut input = String::new();

    stdin.read_line(&mut input);

    let mut word_vector = Vec::new();
    for char in input.chars() {
        word_vector.push(char);
    }

    let mut result = process(word_vector, vec);
    let lenght = result.len();
    print!("Taille du chemin pour ce mot.");
    println!("{}",lenght);
    dbg!(result);

    //Test : word aabc
    println!("Test 1 : abcd");
    let word1 = vec!["a","b","c","d"];
    let mut result = process(word1,vec);
    let lenght = result.len();
    print!("Taille du chemin pour ce mot.");
    println!("{}",lenght);
    dbg!(result);

    let mut vec2 = init_vector2d(4);
    println!("Initialisation : Vector");
    print_vec(vec2.clone());

    //vec = rng_automate_pattern(vec);
    vec2 = rng_given_pattern(vec2);
    println!("Initialisation : Transistions");
    print_vec(vec2.clone());

    //Test : word aabc
    println!("Test 2 : bbcd");
    let word2 = vec!["b","b","c","d"];
    let mut result2 = process(word2,vec2);
    let lenght2 = result2.len();
    print!("Taille du chemin pour ce mot.");
    print!("{}",lenght2);
    dbg!(result2);

}*/
