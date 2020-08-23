#![allow(clippy::needless_return)]
#![feature(is_sorted)]
#[path = "./errors_and_warns.rs"] pub mod errors_and_warns;
#[path = "./tokenizer.rs"] pub mod tokenizer;
#[path = "./key_loader.rs"] pub mod key_loader;
#[path = "./tree_creator.rs"] pub mod tree_creator; //#[path] allows to load a module from an arbitrary part
/**
 * Compiles ARS into bytecode defined in docs/
 * # Arguments
 * * `ars_string` - string containing ARS code
 * # Returns
 * `Vec<u8>` containing compiled code
 */
pub fn compile_ars(ars_string: String) /*-> Vec<String>*/
{
    //Commented out to disable compiler errors
    let mut compiled = Vec::<u8>::new();
    //let mut tree_from_ars = tree_creator::create_ars_tree(ars_string);
    //println!("{:?}", tree_from_ars);
}
