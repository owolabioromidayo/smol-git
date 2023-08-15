use std::io::Error;
use std::io::ErrorKind;
use std::process;
use std::io;
use std::path::Path;
use std::fs; 
use std::fs::File; 
use std::io::Write;
use std::ffi::OsString;
use std::path::PathBuf;


use clap::{arg, Command};

// mod index;
// use index::{GitIndex, GitIndexEntry};

mod objects;
use objects::{hash_object, cat_file};

fn cli() -> Command {
    Command::new("sgit")
        .about("Git, but in rust and crappy")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Initialize new repository in current directory")
        )
        .subcommand(
            Command::new("cat-file")
                .about("Spit the output of a blob")
                .arg(arg!(-p <BLOB_SHA> "The blob name"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("hash-object")
                .about("hash an object")
                .arg(arg!(-w <FILE>))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("ls-tree")
                .about("Inspect a tree object")
                .arg(arg!(--name_only <TREE_SHA>))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("write-tree")
                .about("Store tree object as a blob")
        )
        .subcommand(
            Command::new("commit")
                .about("Commit Tree object")
                .arg(arg!( [TREE_SHA]))
                .arg(arg!(-p <PARENT_SHA>))
                .arg(arg!(-m [MESSAGE]))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("Add files to staging area")
                .arg(arg!(<PATH> ... "Stuff to add").value_parser(clap::value_parser!(PathBuf)))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove files from staging area")
                .arg(arg!(<PATH> ... "Stuff to add").value_parser(clap::value_parser!(PathBuf)))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("status")
                .about("View current index table status.")
        )
        .subcommand(
            Command::new("clone")
                .arg(arg!( [GITHUB_URL]))
                .arg(arg!(<DIR>))
                .arg_required_else_help(true),
        )

}

fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE>)]
}

fn main() -> io::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            //check if git repository already if not, create .git folder

            //there has to be some better way of doing this right
            fs::create_dir("sgit")?;
            fs::create_dir("sgit/refs")?;
            fs::create_dir("sgit/objects")?;


            let mut f = match File::create("sgit/HEAD") {
                Ok(file) => file,
                Err(err) => {
                    return Err(Error::new(ErrorKind::Other, format!("Error creating file: {}", err)));
                }
            };

            match f.write_all("ref: refs/heads/master\n".as_bytes()) {
                Ok(_) => {}
                Err(err) => {
                    return Err(Error::new(ErrorKind::Other, format!("Error creating file: {}", err)));
                }
            }

            println!("GitHub repo initialized");
            //check all files were created? 
        }
        Some(("cat-file", sub_matches)) => {
            let blob_sha = sub_matches.get_one::<String>("BLOB_SHA").map(|s| s.as_str()).unwrap();

            let res = cat_file(&blob_sha);

            if res.is_ok(){
                println!("{}",res.unwrap());
            } else {
                println!("Hashing failed with error: {} ", res.unwrap_err());
            }

        }
        Some(("hash-object", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("FILE").map(|s| s.as_str()).unwrap();
            
            let res = hash_object(&file_path);

            if res.is_ok(){
                println!("Hash of object with path {} is {}.", file_path, res.unwrap());
            } else {
                println!("Hashing failed with error: {} ", res.unwrap_err());
            }
            

        }
        Some(("ls-tree", sub_matches)) => {
            // get an actual tree object from directory
            //sort tree
            //print in alphabetical order
 
            //at least i know what a git tree is now
        }
        Some(("write-tree", sub_matches)) => {
            //get the tree object and store the blob

            
            // let mut hasher = Sha1::new();
            // hasher.update(&buffer);
            // let hash_result = hasher.finalize();

            // //TODO : actual BLOB format
            // let mut file = File::create(format!("sgit/objects/{:x}", hash_result)).unwrap();
            // file.write_all(&buffer).unwrap();

            // println!("{:x}", hash_result);
        }
        Some(("add", sub_matches)) => {
            let paths = sub_matches
                            .get_many::<PathBuf>("PATH")
                            .into_iter()
                            .flatten()
                            .collect::<Vec<_>>();
            println!("Adding {paths:?}");

            //get these files, create their file objs and know their hashes

            //check if file is in index table
            //update this in the index table, which is basically a list of file entries


        }
        Some(("rm", sub_matches)) => {
            let paths = sub_matches
                            .get_many::<PathBuf>("PATH")
                            .into_iter()
                            .flatten()
                            .collect::<Vec<_>>();
            println!("Removing {paths:?}");

            //get these files, compute their hashes
            //check if they are in the index, if they are unstage, if not panic or warn
        }
        Some(("status", sub_matches)) => {
            //print out the current state of the index table. that is all
        }
        Some (("commit", sub_matches)) => {

            let commiter_name = "Owolabi Oromidayo";
            let commiter_enail = "owolabioromidayo16@gmail.com";

            //get parent and tree sha if any
            let tree_sha = sub_matches.get_one::<String>("TREE_SHA").map(|s| s.as_str()).unwrap();
            let parent_sha = sub_matches.get_one::<String>("PARENT_SHA").map(|s| s.as_str()).unwrap();
            let message = sub_matches.get_one::<String>("MESSAGE").map(|s| s.as_str()).unwrap();


            //get the pgp signature (optional)

            //create a commit object and print its sha
        }

        Some(("clone", sub_matches)) => {
            //use gits smart http transfer protocol for cloning
            // https://www.git-scm.com/docs/http-protocol

            //create new dir (into curr dir or provided)

            //check contents and read commit object attributes
        }
        _ => {
            println!("Unknown argument passed.");
        }
    }


    process::exit(0)

}