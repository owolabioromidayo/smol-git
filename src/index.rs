use std::io::{self, Write, Read, Error};
use std::collections::HashMap;

pub struct GitIndexEntry {
    ctime_s: u32,
    ctime_n: u32,
    mtime_n: u32,
    mtime_s: u32,
    dev: u32,
    ino: u32,
    mode: u32, //what size is this really? 2 bytes?
    uid: u32,
    gid: u32,
    fsize: u32,
    sha1: [u8; 20],
    flags: u16,
    name: String,
}

pub struct GitIndex{
    entries : HashMap<String, GitIndexEntry>
}

impl GitIndexEntry {
    pub fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.ctime_s.to_be_bytes());
        writer.write_all(&self.ctime_n.to_be_bytes());
        writer.write_all(&self.mtime_s.to_be_bytes());
        writer.write_all(&self.mtime_n.to_be_bytes());
        writer.write_all(&self.dev.to_be_bytes());
        writer.write_all(&self.ino.to_be_bytes());
        writer.write_all(&self.mode.to_be_bytes());
        writer.write_all(&self.uid.to_be_bytes());
        writer.write_all(&self.gid.to_be_bytes());
        writer.write_all(&self.fsize.to_be_bytes());
        writer.write_all(&self.sha1);
        writer.write_all(&self.flags.to_be_bytes());
        writer.write_all(&( (self.name.len() as u16).to_be_bytes()));
        writer.write_all(self.name.as_bytes())?;
        Ok(())
    }

    pub fn deserialize(reader: &mut impl Read) -> io::Result<Self> {

        // read 12 byte header (are we doing this)
        let mut header = [0u8, 12];
        reader.read_exact(&mut header)?;

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;

        let ctime_s = u32::from_be_bytes(buffer);
        reader.read_exact(&mut buffer)?;

        let ctime_n = u32::from_be_bytes(buffer);
        reader.read_exact(&mut buffer)?;

        let mtime_s = u32::from_be_bytes(buffer);
        reader.read_exact(&mut buffer)?;

        let mtime_n = u32::from_be_bytes(buffer);
        reader.read_exact(&mut buffer)?;

        let dev = u32::from_be_bytes(buffer);
        reader.read_exact(&mut buffer)?;

        let ino = u32::from_be_bytes(buffer);
        reader.read_exact(&mut buffer)?;

        let mode = u32::from_be_bytes(buffer);
        reader.read_exact(&mut buffer)?;

        let uid = u32::from_be_bytes(buffer);
        reader.read_exact(&mut buffer)?;

        let gid = u32::from_be_bytes(buffer);
        reader.read_exact(&mut buffer)?;

        let fsize = u32::from_be_bytes(buffer);

        let mut sha1 = [0u8; 20];
        reader.read_exact(&mut sha1)?;

        let mut buffer16 = [0u8; 2];
        reader.read_exact(&mut buffer16)?;

        let flags = u16::from_be_bytes(buffer16); 
        reader.read_exact(&mut buffer16)?;

        let name_len= u16::from_be_bytes(buffer16); 
        let mut name_bytes = vec![0u8; name_len as usize];
        reader.read_exact(&mut name_bytes)?;
        let name = String::from_utf8_lossy(&name_bytes).to_string();

        Ok(GitIndexEntry {
            ctime_s,
            ctime_n,
            mtime_s,
            mtime_n,
            dev,
            ino,
            mode,
            uid,
            gid,
            fsize,
            sha1,
            flags,
            name,
        })
    }
}


impl GitIndex{
    pub fn new() -> Self {
        GitIndex {
            entries: HashMap::new(),
        }
    }

    //this would mean we have to serialize and deserialize on each update

    pub fn add_entry(&mut self, name: String, entry: GitIndexEntry) {
        self.entries.insert(name, entry);
    }

    pub fn remove_entry(&mut self, name: &str) -> Option<GitIndexEntry> {
        self.entries.remove(name)
    }

    pub fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        for (name, entry) in &self.entries {
            let name_len = name.len() as u16;
            writer.write_all(&name_len.to_be_bytes());
            writer.write_all(name.as_bytes())?;
            entry.serialize(writer)?;
        }
        Ok(())
    }

    pub fn deserialize(reader: &mut impl Read) -> io::Result<Self> {
        let mut entries = GitIndex::new();
        let mut name_len_bytes = [0u8; 2];
        while let Ok(()) = reader.read_exact(&mut name_len_bytes) {

            let name_len = u16::from_be_bytes(name_len_bytes);
            let mut name_bytes = vec![0u8; name_len as usize];
            reader.read_exact(&mut name_bytes);

            let name = String::from_utf8_lossy(&name_bytes).to_string();
            let entry = GitIndexEntry::deserialize(reader)?;
            entries.add_entry(name, entry);
        }
        Ok(entries)
    }

    pub fn ls_files(&mut self) -> Result<String, Error>{
        for (name, entry) in &self.entries{ 
            let first_bytes = ((entry.mode >> 16) & 0xFFFF) as u16; //dont know if this works

            let mode_str  = match first_bytes { 
                04 => "tree",    
                10 => "blob",    
                12 => "blob",    
                16 => "commit",    
                _ => {
                    eprintln!("Unknown mode types read in index");
                    "_"
                }
            };
            println!("{} {:?} {}",& mode_str, entry.sha1, &entry.name );
        }
        Ok(String::from("No entries"))
    }

}

    // Create a GitIndexEntry instance
    // let entry = GitIndexEntry {
    //     ctime_s: 12345,
    //     ctime_n: 67890,
    //     mtime_s: 98765,
    //     mtime_n: 43210,
    //     dev: 1,
    //     ino: 123,
    //     mode: 0o644,
    //     uid: 1000,
    //     gid: 1000,
    //     fsize: 1024,
    //     sha1: [0; 20], // Placeholder for SHA-1 hash
    //     flags: 0b1000, // Regular file
    //     name: "example.txt".to_string(),
    // };

    // // Serialize the entry to a file
    // let mut file = File::create("example.index")?;
    // entry.serialize(&mut file)?;

    // // Deserialize the entry from the file
    // let mut file = File::open("example.index")?;
    // let deserialized_entry = GitIndexEntry::deserialize(&mut file)?;

    // // Print the deserialized entry
    // println!("{:?}", deserialized_entry);

    // Ok(())