use std::{fs::{self, File}, io::Error};
use crate::index::{GitIndex, GitIndexEntry};


//impl add remove and status functions here
//also impl file handling
// these will be the ones that interface direct with cli

pub fn add(){
    //add files to the staging area
    //create new file objs and hashes, insert into index file
    //if the file is present from before, delete the old obj and reset
}

pub fn rm( ){
    //rm files from the staging area
    //del objs also
}

//git index does not contain untracked files. but we need to do that
//walk through files, check if they are staged, print seperately from those not staged
pub fn status(index: GitIndex) -> Result<(), Error> {
    //load head file as an index

    let mut reader = File::open("sgit/HEAD")?;
    let head_index = GitIndex::deserialize(&mut reader)?;

    //compare hashes and filenames in head index. different hashes for filename means changes
    //hashmap key should be fname then

    // the index does not contain the head. diff is easier then

    //laod working dir as index and compare with staging/index
    
    println!("Changes to be committed:");
    // for entry in index.entries {
    //     println!("    {} {}", entry.status, entry.filename);
    // }

    println!("Changes not staged for commit:");

    println!("Untracked files:");

    Ok(())
}

pub fn commit(){ 
    //dont understand this process yet

        // let commiter_name = "Owolabi Oromidayo";
        // let commiter_enail = "owolabioromidayo16@gmail.com";

        // //get parent and tree sha if any
        // let tree_sha = sub_matches.get_one::<String>("TREE_SHA").map(|s| s.as_str()).unwrap();
        // let parent_sha = sub_matches.get_one::<String>("PARENT_SHA").map(|s| s.as_str()).unwrap();
        // let message = sub_matches.get_one::<String>("MESSAGE").map(|s| s.as_str()).unwrap();
// 

        //get the pgp signature (optional)

        //create a commit object and print its sha
}

pub fn ls_files(){

}


//write tests for this. only way to gain some sanity


