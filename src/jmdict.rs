use serde::{Deserialize, Serialize};

const JMDICT: &str = "resources/jmdict.xml";

// <!ELEMENT JMdict (entry*)>
#[derive(Debug, Deserialize, Serialize)]
pub struct JMDict { 
    #[serde(rename = "entry")]
    entries: Vec<Entry>,
} 

// <!ELEMENT entry (ent_seq, k_ele*, r_ele+, sense+)>
#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub ent_seq: String,
    #[serde(default)]
    pub k_ele: Vec<KEle>,
    pub r_ele: Vec<REle>,
    pub sense: Vec<Sense>,
}

// <!ELEMENT k_ele (keb, ke_inf*, ke_pri*)>
#[derive(Debug, Deserialize, Serialize)]
pub struct KEle {
    keb: String,
    #[serde(default)]
    ke_inf: Vec<String>,
    #[serde(default)]
    ke_pri: Vec<String>,
}

// <!ELEMENT r_ele (reb, re_nokanji?, re_restr*, re_inf*, re_pri*)>
#[derive(Debug, Deserialize, Serialize)]
pub struct REle {
    reb: String,
    #[serde(default)]
    re_nokanji: String,
    #[serde(default)]
    re_restr: Vec<String>,
    #[serde(default)]
    re_inf: Vec<String>,
    #[serde(default)]
    re_pri: Vec<String>,
}

// <!ELEMENT sense (stagk*, stagr*, pos*, xref*, ant*, field*, misc*, s_inf*, lsource*, dial*, gloss*, example*)>
#[derive(Debug, Deserialize, Serialize)]
pub struct Sense {
    #[serde(default)]
    stagk: Vec<String>,
    #[serde(default)]
    stagr: Vec<String>,
    #[serde(default)]
    pos: Vec<String>,
    #[serde(default)]
    xref: Vec<String>,
    #[serde(default)]
    ant: Vec<String>,
    #[serde(default)]
    field: Vec<String>,
    #[serde(default)]
    misc: Vec<String>,
    #[serde(default)]
    s_inf: Vec<String>,
    #[serde(default)]
    lsource: Vec<LSource>,
    #[serde(default)]
    dial: Vec<String>,
    #[serde(default)]
    gloss: Vec<Gloss>,
}

// <!ELEMENT lsource (#PCDATA)>
// <!ATTLIST lsource xml:lang CDATA "eng">
// <!ATTLIST lsource ls_type CDATA #IMPLIED>
// <!ATTLIST lsource ls_wasei CDATA #IMPLIED>
#[derive(Debug, Deserialize, Serialize)]
pub struct LSource {
    #[serde(rename = "$value", default)]
    lsource: String,
    // #[serde(rename = "$attr:lang", default)]
    #[serde(default)]   
    lang: String,
    // #[serde(rename = "$attr:ls_type", default)]
    #[serde(default)]
    ls_type: String,
    // #[serde(rename = "$attr:ls_wasei", default)]
    #[serde(default)]
    ls_wasei: char,  
}


// <!ELEMENT gloss (#PCDATA | pri)*>
// <!ATTLIST gloss xml:lang CDATA "eng">
// <!ATTLIST gloss g_gend CDATA #IMPLIED>
// <!ATTLIST gloss g_type CDATA #IMPLIED>
// <!ELEMENT pri (#PCDATA)>
#[derive(Debug, Deserialize, Serialize)]
pub struct Gloss {
    #[serde(rename = "$value", default)]
    gloss: String,
    #[serde(default)]
    lang: String,
    #[serde(default)]
    g_gend: String,    
    #[serde(default)]
    g_type: String,
}

#[derive(Debug)]
pub enum JMDictError {
    IoError(std::io::Error),
    XmlError(serde_xml_rs::Error),
}


impl From<std::io::Error> for JMDictError {
    fn from(value: std::io::Error) -> Self {
        JMDictError::IoError(value)
    }
}

impl From<serde_xml_rs::Error> for JMDictError {
    fn from(value: serde_xml_rs::Error) -> Self {
        JMDictError::XmlError(value)
    }
}

pub fn read_jmdict(jmdict: &str) -> Result<JMDict, JMDictError > { 
    let mut content = String::new();  
    let content = std::fs::read_to_string(jmdict)?;
    let jmdict: JMDict = serde_xml_rs::from_str(&content)?;
    Ok(jmdict)
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn test_read_jmdict() {
        let jmdict = read_jmdict("./tests/resources/jmdict.xml");
        println!("{:?}", env::current_dir());
        match jmdict {
            Ok(jmdict) => {
                // println!("{:?}", jmdict);
                // let s = to_string(&jmdict.entry[0]);
                let s = &jmdict.entries[0];
                println!("{:#?}", s);
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

