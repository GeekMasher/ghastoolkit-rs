use anyhow::Result;
use ghastoolkit::{CodeQLPack, CodeQLPackType, CodeQLPacks};

fn main() -> Result<()> {
    // Load my Java CodeQL Pack Library
    let pack_lib = CodeQLPack::new("./java/lib");
    println!("Pack Name :: {}", pack_lib.get_name());
    println!("Pack Type :: {}", pack_lib.get_type());
    println!("Pack Path :: {}", pack_lib.get_path().display());

    assert_eq!(pack_lib.get_type(), CodeQLPackType::Library);

    // Load my Java CodeQL Pack Queries
    let pack_src = CodeQLPack::new("./java/src");
    println!("Pack Name :: {}", pack_src.get_name());
    println!("Pack Type :: {}", pack_src.get_type());
    println!("Pack Path :: {}", pack_src.get_path().display());

    assert_eq!(pack_src.get_type(), CodeQLPackType::Queries);

    // Load all CodeQL Packs from the current directory
    let mut packs = CodeQLPacks::load("./")?;
    // Sort the packs by type
    packs.sort();

    println!("Packs :: {:?}", packs);

    assert_eq!(packs.len(), 2);

    Ok(())
}