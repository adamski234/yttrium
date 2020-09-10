#![allow(clippy::needless_return)]
#![feature(is_sorted)]
#[path = "./errors_and_warns.rs"] pub mod errors_and_warns;
#[path = "./tokenizer.rs"] mod tokenizer;
#[path = "./key_loader.rs"] pub mod key_loader;
#[path = "./tree_creator.rs"] pub mod tree_creator; //#[path] allows to load a module from an arbitrary part
#[path ="./interpreter.rs"] pub mod interpreter;
#[path ="./environment.rs"] pub mod environment;


pub fn run_ars_string(ars_string: String, key_list: &Vec<Box<dyn key_base::Key>>) {
	let tree = tree_creator::create_ars_tree(ars_string, key_list).unwrap().tree; //TODO: return warnings and errors
	return run_ars_tree(tree, key_list);
}

pub fn run_ars_tree(tree: Vec<tree_creator::TreeNode>, key_list: &Vec<Box<dyn key_base::Key>>) {
	todo!();
}