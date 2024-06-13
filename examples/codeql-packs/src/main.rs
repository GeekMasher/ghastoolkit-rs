use anyhow::Result;
use ghastoolkit::{CodeQLPack, CodeQLPackType, CodeQLPacks};

fn main() -> Result<()> {
    // Load my Java CodeQL Pack Library
    let pack_lib = CodeQLPack::new("./java/lib");
    println!("Pack Name :: {}", pack_lib.name());
    println!("Pack Type :: {}", pack_lib.pack_type());
    println!("Pack Path :: {}", pack_lib.path().display());

    assert_eq!(pack_lib.pack_type(), CodeQLPackType::Library);
    assert_eq!(pack_lib.name(), String::from("geekmasher/codeql-java-lib"));

    // Load my Java CodeQL Pack Queries
    let pack_src = CodeQLPack::new("./java/src");
    println!("Pack Name :: {}", pack_src.name());
    println!("Pack Type :: {}", pack_src.pack_type());
    println!("Pack Path :: {}", pack_src.path().display());

    assert_eq!(pack_src.pack_type(), CodeQLPackType::Queries);
    assert_eq!(pack_lib.name(), String::from("geekmasher/codeql-java"));

    // Load all CodeQL Packs from the current directory
    let mut packs = CodeQLPacks::load("./")?;
    // Sort the packs by type
    packs.sort();

    println!("Packs :: {:?}", packs);

    assert_eq!(packs.len(), 2);

    Ok(())
}
