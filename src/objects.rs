use sha1::{Sha1, Digest};
use std::{io::{Error, Write, Read}, fs::{self, File}};

struct TreeEntry { 
    mode: [u8; 6],
    path: String,
    sha1: [u8; 20]
}

struct Tree { 
    entries : Vec <TreeEntry>
}

pub fn hash_object(fpath: &str) -> Result<String, Error>{

        let buffer = fs::read(format!("{}", fpath))?;

        //generate sha
        let mut hasher = Sha1::new();
        hasher.update(&buffer);
        let hash_result = hasher.finalize();
        let hash_hex = hash_result.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        let dir_path = format!("sgit/objects/{}", &hash_hex[..2]);
        if !(fs::metadata(&dir_path).is_ok() && fs::metadata(&dir_path).unwrap().is_dir() ) {
            fs::create_dir(&dir_path)?;
        }

        //TODO : actual BLOB format
        let mut file = File::create(format!("sgit/objects/{}/{}", &hash_hex[..2], &hash_hex[2..]))?;
        file.write_all(&buffer)?;

        return Ok(format!("{:x}", hash_result));
}

pub fn cat_file(blob_sha : &str) -> Result<String, Error>{
    fs::create_dir(format!("sgit/objects/{}", &blob_sha[..2]))?;
    let buffer = fs::read(format!("sgit/objects/{}/{}", &blob_sha[..2], &blob_sha[2..]))?;
    return Ok(format!("{:?}", buffer));
}



// [mode] space [path] 0x00 [sha-1]

impl TreeEntry {
    // Serialize the struct to a binary vector
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(&self.mode);
        buffer.push(b' ');
        buffer.extend(self.path.as_bytes());
        buffer.push(0x00);
        buffer.extend(&self.sha1);

        buffer
    }

    // Deserialize a binary vector back to a struct
    fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < 29 {
            return None;
        }

        let mut mode = [0u8; 6];
        mode.copy_from_slice(&data[..6]);

        // let mode_value = u8::from_str_radix(std::str::from_utf8(mode).unwrap(), 8).ok()?;

        let path_end = data.iter().rposition(|&byte| byte == 0x00).unwrap_or(0);
        let path = String::from_utf8_lossy(&data[7..path_end]).to_string();

        let sha1_start = path_end + 1;
        let sha1_end = sha1_start + 20;
        let mut sha1 = [0u8; 20];
        sha1.copy_from_slice(&data[sha1_start..sha1_end]);

        Some(TreeEntry {
            mode,
            path,
            sha1,
        })
    }
}


impl Tree {
    fn serialize(&self, writer: &mut impl Write) -> std::io::Result<()> {
        for entry in &self.entries {
            writer.write_all(&entry.mode)?;
            writer.write_all( &[b' '])?;
            writer.write_all(entry.path.as_bytes())?;
            writer.write_all(&[0x00])?;
            writer.write_all(&entry.sha1)?;
        }
        Ok(())
    }

    // Deserialize binary data to create a Tree struct
    fn deserialize(reader: &mut impl Read) -> std::io::Result<Self> {
        let mut entries = Vec::new();
        loop {
            let mut mode = [0u8; 6];
            reader.read_exact(&mut mode)?;
          
            let mut buf = [0u8; 1];
            let mut path_bytes = Vec::new();
            loop {
                reader.read_exact(&mut buf)?;
                if buf[0] == 0x00 {
                    break;
                }
                path_bytes.push(buf[0]);
            }
            let path = String::from_utf8_lossy(&path_bytes).to_string();

            let mut sha1 = [0u8; 20];
            reader.read_exact(&mut sha1)?;

            entries.push(TreeEntry { mode, path, sha1 });

            // Check if we've reached the end of the reader
            if reader.read_exact(&mut [0u8; 1]).is_err() {
                break;
            }
        }

        Ok(Tree { entries })
    }
}


pub fn ls_tree(fpath: &str) -> Result<String, Error>{

    //make recursive an option?

    //get the tree object
    let buffer = fs::read(format!("{}", fpath))?;
    let tree  = TreeEntry::deserialize(&buffer);

    let mut ret = String::from("");

    for entry in &tree {
        //check the entry type
        let first_bytes = &entry.mode[..2];

        let mode_str  = match first_bytes { 
            [0, 4] => "tree",    
            [1, 0] => "blob",    
            [1, 2] => "blob",    
            [1, 6] => "commit",    
            _ => "" //should panic or propagate error out here
        };
        let new_str = match mode_str{
            "tree" => ls_tree(&entry.path)? , //get path recursively
            _ => format!("{:?} {} {:?} {}", entry.mode, mode_str, entry.sha1, &entry.path)
        };

        ret = ret + &new_str;
        ret.push_str("\n");
    }

    Ok(ret)

    
}


pub fn write_tree(){

}
