use std::fs::File;
use std::io::{self, Write, Read};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::collections::HashMap;

//should we make a simpler index? for status purposes?


pub struct GitIndexEntry {
    ctime_s: u32,
    ctime_n: u32,
    mtime_n: u32,
    mtime_s: u32,
    dev: u32,
    ino: u32,
    mode: u32,
    uid: u32,
    gid: u32,
    fsize: u32,
    // sha1: [u8; 20],
    flags: u16,
    name: String,
}

pub struct GitIndex{
    entries : HashMap<String, GitIndexEntry>
}

impl GitIndexEntry {
    pub fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u32::<BigEndian>(self.ctime_s)?;
        writer.write_u32::<BigEndian>(self.ctime_n)?;
        writer.write_u32::<BigEndian>(self.mtime_s)?;
        writer.write_u32::<BigEndian>(self.mtime_n)?;
        writer.write_u32::<BigEndian>(self.dev)?;
        writer.write_u32::<BigEndian>(self.ino)?;
        writer.write_u32::<BigEndian>(self.mode)?;
        writer.write_u32::<BigEndian>(self.uid)?;
        writer.write_u32::<BigEndian>(self.gid)?;
        writer.write_u32::<BigEndian>(self.fsize)?;
        // writer.write_all(&self.sha1)?;
        writer.write_u16::<BigEndian>(self.flags)?;
        writer.write_u16::<BigEndian>(self.name.len() as u16)?;
        writer.write_all(self.name.as_bytes())?;
        Ok(())
    }

    pub fn deserialize(reader: &mut impl Read) -> io::Result<Self> {
        let ctime_s = reader.read_u32::<BigEndian>()?;
        let ctime_n = reader.read_u32::<BigEndian>()?;
        let mtime_s = reader.read_u32::<BigEndian>()?;
        let mtime_n = reader.read_u32::<BigEndian>()?;
        let dev = reader.read_u32::<BigEndian>()?;
        let ino = reader.read_u32::<BigEndian>()?;
        let mode = reader.read_u32::<BigEndian>()?;
        let uid = reader.read_u32::<BigEndian>()?;
        let gid = reader.read_u32::<BigEndian>()?;
        let fsize = reader.read_u32::<BigEndian>()?;
        let mut sha1 = [0u8; 20];
        // reader.read_exact(&mut sha1)?;
        let flags = reader.read_u16::<BigEndian>()?;
        let name_len = reader.read_u16::<BigEndian>()?;
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
            // sha1,
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

    pub fn serialize_entries(&self, writer: &mut impl Write) -> io::Result<()> {
        for (name, entry) in &self.entries {
            let name_len = name.len() as u16;
            writer.write_u16::<BigEndian>(name_len)?;
            writer.write_all(name.as_bytes())?;
            entry.serialize(writer)?;
        }
        Ok(())
    }

    pub fn deserialize_entries(reader: &mut impl Read) -> io::Result<Self> {
        let mut entries = GitIndex::new();
        while let Ok(name_len) = reader.read_u16::<BigEndian>() {
            let mut name_bytes = vec![0u8; name_len as usize];
            reader.read_exact(&mut name_bytes)?;
            let name = String::from_utf8_lossy(&name_bytes).to_string();
            let entry = GitIndexEntry::deserialize(reader)?;
            entries.add_entry(name, entry);
        }
        Ok(entries)
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