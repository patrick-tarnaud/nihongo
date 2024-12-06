mod jmdict;
use jmdict::read_jmdict;

fn main() {
    let jmdict = read_jmdict("./resources/jmdict.xml");
    assert!(jmdict.is_ok());
    let jmdict = jmdict.unwrap();
    println!("{:#?}", jmdict.entries.len());
}    
