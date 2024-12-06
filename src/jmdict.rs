use serde::{Deserialize, Serialize};

const JMDICT: &str = "resources/jmdict.xml";

// <!ELEMENT JMdict (entry*)>
#[derive(Debug, Deserialize, Serialize)]
pub struct JMDict { 
    #[serde(rename = "entry")]
    pub entries: Vec<Entry>,
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
    pub keb: String,
    #[serde(default)]
    pub ke_inf: Vec<String>,
    #[serde(default)]
    pub ke_pri: Vec<String>,
}

// <!ELEMENT r_ele (reb, re_nokanji?, re_restr*, re_inf*, re_pri*)>
#[derive(Debug, Deserialize, Serialize)]
pub struct REle {
    pub reb: String,
    #[serde(default)]
    pub re_nokanji: String,
    #[serde(default)]
    pub re_restr: Vec<String>,
    #[serde(default)]
    pub re_inf: Vec<String>,
    #[serde(default)]
    pub re_pri: Vec<String>,
}

// <!ELEMENT sense (stagk*, stagr*, pos*, xref*, ant*, field*, misc*, s_inf*, lsource*, dial*, gloss*, example*)>
#[derive(Debug, Deserialize, Serialize)]
pub struct Sense {
    #[serde(default)]
    pub stagk: Vec<String>,
    #[serde(default)]
    pub stagr: Vec<String>,
    #[serde(default)]
    pub pos: Vec<String>,
    #[serde(default)]
    pub xref: Vec<String>,
    #[serde(default)]
    pub ant: Vec<String>,
    #[serde(default)]
    pub field: Vec<String>,
    #[serde(default)]
    pub misc: Vec<String>,
    #[serde(default)]
    pub s_inf: Vec<String>,
    #[serde(default)]
    pub lsource: Vec<LSource>,
    #[serde(default)]
    pub dial: Vec<String>,
    #[serde(default)]
    pub gloss: Vec<Gloss>,
}

// <!ELEMENT lsource (#PCDATA)>
// <!ATTLIST lsource xml:lang CDATA "eng">
// <!ATTLIST lsource ls_type CDATA #IMPLIED>
// <!ATTLIST lsource ls_wasei CDATA #IMPLIED>
#[derive(Debug, Deserialize, Serialize)]
pub struct LSource {
    #[serde(rename = "$value", default)]
    pub lsource: String,
    #[serde(default)]   
    pub lang: String,
    #[serde(default)]
    pub ls_type: String,
    #[serde(default)]
    pub ls_wasei: char,  
}


// <!ELEMENT gloss (#PCDATA | pri)*>
// <!ATTLIST gloss xml:lang CDATA "eng">
// <!ATTLIST gloss g_gend CDATA #IMPLIED>
// <!ATTLIST gloss g_type CDATA #IMPLIED>
// <!ELEMENT pri (#PCDATA)>
#[derive(Debug, Deserialize, Serialize)]
pub struct Gloss {
    #[serde(rename = "$value", default)]
    pub gloss: String,
    #[serde(default)]
    pub lang: String,
    #[serde(default)]
    pub g_gend: String,    
    #[serde(default)]
    pub g_type: String,
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
    let content = std::fs::read_to_string(jmdict)?;
    let jmdict: JMDict = serde_xml_rs::from_str(&content)?;
    Ok(jmdict)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn clear_string(s: &str) -> String {
        s.replace(" ", "").replace("\n", "").replace("\t", "")
    }

    const ENTRY1_CONTENT: &'static str = r#"
    Entry {
        ent_seq: "1260110",
        k_ele: [
            KEle {
                keb: "見本市",
                ke_inf: [],
                ke_pri: [
                    "news1",
                    "nf17",
                ],
            },
        ],
        r_ele: [
            REle {
                reb: "みほんいち",
                re_nokanji: "",
                re_restr: [],
                re_inf: [],
                re_pri: [
                    "news1",
                    "nf17",
                ],
            },
        ],
        sense: [
            Sense {
                stagk: [],
                stagr: [],
                pos: [
                    "noun (common) (futsuumeishi)",
                ],
                xref: [],
                ant: [],
                field: [],
                misc: [],
                s_inf: [],
                lsource: [
                    LSource {
                        lsource: "test",
                        lang: "rus",
                        ls_type: "",
                        ls_wasei: '\0',
                    },
                ],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "trade fair",
                        lang: "",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
            Sense {
                stagk: [],
                stagr: [],
                pos: [],
                xref: [],
                ant: [],
                field: [],
                misc: [],
                s_inf: [],
                lsource: [],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "handelsbeurs",
                        lang: "dut",
                        g_gend: "",
                        g_type: "",
                    },
                    Gloss {
                        gloss: "vakbeurs",
                        lang: "dut",
                        g_gend: "",
                        g_type: "",
                    },
                    Gloss {
                        gloss: "{gew.} foor",
                        lang: "dut",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
            Sense {
                stagk: [],
                stagr: [],
                pos: [],
                xref: [],
                ant: [],
                field: [],
                misc: [],
                s_inf: [],
                lsource: [],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "Messe",
                        lang: "ger",
                        g_gend: "",
                        g_type: "",
                    },
                    Gloss {
                        gloss: "Mustermesse",
                        lang: "ger",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
            Sense {
                stagk: [],
                stagr: [],
                pos: [],
                xref: [],
                ant: [],
                field: [],
                misc: [],
                s_inf: [],
                lsource: [],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "termékbemutató",
                        lang: "hun",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
            Sense {
                stagk: [],
                stagr: [],
                pos: [],
                xref: [],
                ant: [],
                field: [],
                misc: [],
                s_inf: [],
                lsource: [],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "торговля [на ярмарке] по образцам; выставка образцов (дляпродажи); выставка-продажа (товаров, изделий)",
                        lang: "rus",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
            Sense {
                stagk: [],
                stagr: [],
                pos: [],
                xref: [],
                ant: [],
                field: [],
                misc: [],
                s_inf: [],
                lsource: [],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "feria de muestras",
                        lang: "spa",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
            Sense {
                stagk: [],
                stagr: [],
                pos: [],
                xref: [],
                ant: [],
                field: [],
                misc: [],
                s_inf: [],
                lsource: [],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "fackmässa",
                        lang: "swe",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
        ],
    }"#;

    const ENTRY2_CONTENT: &str = r#"
    Entry {
        ent_seq: "1053260",
        k_ele: [],
        r_ele: [
            REle {
                reb: "コンビナートキャンペーン",
                re_nokanji: "",
                re_restr: [],
                re_inf: [],
                re_pri: [],
            },
            REle {
                reb: "コンビナート・キャンペーン",
                re_nokanji: "",
                re_restr: [],
                re_inf: [],
                re_pri: [],
            },
        ],
        sense: [
            Sense {
                stagk: [],
                stagr: [],
                pos: [
                    "noun (common) (futsuumeishi)",
                ],
                xref: [],
                ant: [],
                field: [],
                misc: [
                    "obsolete term",
                ],
                s_inf: [],
                lsource: [
                    LSource {
                        lsource: "kombinat",
                        lang: "rus",
                        ls_type: "",
                        ls_wasei: '\0',
                    },
                    LSource {
                        lsource: "campaign",
                        lang: "eng",
                        ls_type: "part",
                        ls_wasei: 'y',
                    },
                ],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "coordinated advertising campaign for various different products (sharing brand name, slogans, etc.)",
                        lang: "",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
            Sense {
                stagk: [],
                stagr: [],
                pos: [],
                xref: [],
                ant: [],
                field: [],
                misc: [],
                s_inf: [],
                lsource: [],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "industrieübergreifende Werbekampagne",
                        lang: "ger",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
        ],
    }"#;

    const ENTRY3_CONTENT: &str = r#"
    Entry {
        ent_seq: "1053261",
        k_ele: [],
        r_ele: [
            REle {
                reb: "コンビナートキャンペーン",
                re_nokanji: "",
                re_restr: [],
                re_inf: [],
                re_pri: [],
            },
            REle {
                reb: "コンビナート・キャンペーン",
                re_nokanji: "",
                re_restr: [],
                re_inf: [],
                re_pri: [],
            },
        ],
        sense: [
            Sense {
                stagk: [],
                stagr: [],
                pos: [
                    "noun (common) (futsuumeishi)",
                ],
                xref: [],
                ant: [],
                field: [],
                misc: [
                    "obsolete term",
                ],
                s_inf: [],
                lsource: [
                    LSource {
                        lsource: "kombinat",
                        lang: "rus",
                        ls_type: "",
                        ls_wasei: '\0',
                    },
                    LSource {
                        lsource: "campaign",
                        lang: "eng",
                        ls_type: "part",
                        ls_wasei: 'y',
                    },
                ],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "coordinated advertising campaign for various different products (sharing brand name, slogans, etc.)",
                        lang: "",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
            Sense {
                stagk: [],
                stagr: [],
                pos: [],
                xref: [],
                ant: [],
                field: [],
                misc: [],
                s_inf: [],
                lsource: [],
                dial: [],
                gloss: [
                    Gloss {
                        gloss: "industrieübergreifende Werbekampagne",
                        lang: "ger",
                        g_gend: "",
                        g_type: "",
                    },
                ],
            },
        ],
    }"#;

    #[test]
    fn test_read_jmdict()  {
        let r_jmdict = read_jmdict("./tests/resources/jmdict.xml");
        assert!(r_jmdict.is_ok(),"Error reading jmdict.");
        let jmdict = r_jmdict.unwrap();
        assert!(jmdict.entries.len() == 3, "There should be 3 entries in jmdict.");
        let entry1 = &jmdict.entries[0];
        let entry2 = &jmdict.entries[1];
        let entry3 = &jmdict.entries[2];

        assert_eq!(clear_string(ENTRY1_CONTENT), clear_string(&format!("{:#?}", entry1)));
        assert_eq!(clear_string(ENTRY2_CONTENT), clear_string(&format!("{:#?}", entry2)));
        assert_eq!(clear_string(ENTRY3_CONTENT), clear_string(&format!("{:#?}", entry3)));
    }
}

