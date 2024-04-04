use std::fmt::UpperHex;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use regex::Regex;

use serde::{Deserialize, Serialize};

use std::fs::write;

#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    be: Vec<String>,
    af: Vec<String>,
    en: String,
    no: Vec<String>,

    ta: Vec<String>,
    ty: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct ChangeSet {
    fr: String,
    to: String,
    or: String,
    os: String,
    so: Vec<String>,
    no: Vec<String>,
    ch: Vec<Rule>,
}
#[derive(Debug, Serialize, Deserialize)]
struct FamilySet {
    fa: String,
    cs: Vec<ChangeSet>,
}

fn main() {
    let mut s = read_file();
    println!("{s}");

    let plusminus = Regex::new(r"(?<plusminus>[+-]) ").unwrap();
    let s = plusminus.replace_all(s.as_str(), "$1").to_string();
    let dashes = Regex::new(r"— ").unwrap();
    let s = dashes.replace_all(s.as_str(), "").to_string();

    let headings =
        Regex::new(r"([0-9]*\.)*[0-9] (?<fromto>(?<from>.*) to (?<to>[^\r\n]*))\r").unwrap();

    let change_sets = Vec::<ChangeSet>::new();

    let s = headings.replace_all(s.as_str(), "°$fromto").to_string();
    println!("{s}");

    let notes = Regex::new(r"(\r\n\r\n)(?<note>[^→\r\n]+)\r\n").unwrap();

    let re = Regex::new(
        r"(?<before>.*) → (?<after>[^/\r\n]*) ?\/? ?(?<env>[^/\r\n;]*)?;?(?<notes>[^\r\n]*)?",
    )
    .unwrap();

    let change_sets_regex = Regex::new(
        r"(°)(?<from>.*) to (?<to>.*)\n\r\n(?<or_contr>[^,]*),(?<src>[^/\r\n]*)(?<changes>[^°]*)",
    )
    .unwrap();

    let parens = Regex::new(r"(°)(?<from>.*) to (?<to>.*)(?<changes>[^°]*)").unwrap();

    let mut sources: Vec<String> = Vec::new();

    let cs: Vec<_> = change_sets_regex
        .captures_iter(s.as_str())
        .map(|c| {
            let fr = c.name("from").unwrap().as_str().to_string();
            let to = c.name("to").unwrap().as_str().to_string();
            let or = c.name("or_contr").unwrap().as_str().to_string();
            let os = c.name("src").unwrap().as_str().to_string();

            let changes2 = c.name("changes").unwrap().as_str().to_string();
            println!("{changes2}");
            let changes2 = notes
                .replace_all(changes2.as_str(), "$1 →  /;$note")
                .to_string();

            let mut output: Vec<&str> = Vec::new();

            let mut so = Vec::new();

            let no = Vec::new();

            let ch: Vec<_> = re
                .captures_iter(changes2.as_str())
                .map(|c| {
                    let be: Vec<String> = c
                        .name("before")
                        .unwrap()
                        .as_str()
                        .to_string()
                        .split_terminator(" ")
                        .map(|c| replace_classes(c))
                        .collect();
                    let mut af: Vec<String> = c
                        .name("after")
                        .unwrap()
                        .as_str()
                        .to_string()
                        .split_terminator(" ")
                        .map(|c| replace_classes(c))
                        .collect();
                    let en = replace_classes(c.name("env").unwrap().as_str());
                    
                    let note = c.name("notes").unwrap().as_str().to_string();

                    let mut no: Vec<String> = Vec::new();
                    if note != "".to_string() {
                        no.push(note);
                    } else {
                    }

                    while af.len() > be.len() {
                        no.push(af.pop().unwrap());
                    }

                    
                    let ta = vec!["generated".to_string(), "unchecked".to_string()];
                    let ty = Vec::new();

                    Rule {
                        be,
                        af,
                        en,
                        no,
                        
                        ta,
                        ty,
                    }
                })
                .collect();

            if !sources.contains(&os) {
                sources.push(os.clone());
                let mut s = (sources.len() - 1).to_string();
                s.insert_str(0, "s");
                so.push(s);
            } else {
                so.push(
                    sources
                        .iter()
                        .position(|r| r == &os)
                        .unwrap()
                        .to_string(),
                );
            }

            ChangeSet {
                fr,
                to,
                or,
                os,
                so,
                no,
                ch,
            }
        })
        .collect();

    let family_set = FamilySet {
        fa: "TODO".to_string(),
        cs,
    };
    /*
    let change_sets_output = String::new();

    for set in change_sets {
        let mut rules = String::new();
        for rule in set.changes {
            let output = json!({
            "before": format!("{}", rule.before[0])
            });
            rules.push_str(serde_json::to_string(&output).unwrap().as_str());
        }
        let output = json!({
        "family": "TODO",
        "changesets": format!("{}", rules)
        });
    } */
    let final_string = serde_json::to_string_pretty(&family_set).unwrap();
    println!("{}", final_string);

    println!("{:#?}", &sources);
    let final_sources = serde_json::to_string_pretty(&sources).unwrap();

    std::fs::write::<&str, &[u8]>(
        r"C:/Users\ensel\Documents\nils\Programming\indexdiachronica\INDEX\sources.txt",
        final_sources.as_ref(),
    )
    .unwrap();

    finish(final_string).expect("couldn't save file");
    //dbg!(change_sets);
}

fn read_file() -> String {
    let file =
        File::open(r"C:/Users\ensel\Documents\nils\Programming\indexdiachronica\INDEX\input.txt")
            .unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents
    // return contents;
    // , std::IO::Error
}

fn finish(f: String) -> std::io::Result<()> {
    std::fs::write::<&str, &[u8]>(
        r"C:/Users\ensel\Documents\nils\Programming\indexdiachronica\INDEX\ouput.txt",
        f.as_ref(),
    )?;

    Ok(())
}

fn replace_classes(s: &str) -> String {
    s.replace("C[", "[+consonant ")
        .replace("V[", "[+vowel ")
        .replace("E[", "[+vowel +front ")
        .replace("F[", "[+fricative ")
        .replace("J[", "[+approximant ")
        .replace("Ḱ[", "[+palatovelar ")
        .replace("K[", "[+velar ")
        .replace("L[", "[+liquid ")
        .replace("N[", "[+nasal ")
        .replace("O[", "[-sonorant ")
        .replace("P[", "[+labial,bilabial ")
        .replace("Q[", "[+uvular ")
        //.replace("Q[", "[+click ")
        .replace("R[", "[+sonorant ")
        .replace("S[", "[+plosive ")
        .replace("T[", "[+plosive -voice ")
        .replace("U[", "σ")
        .replace("W[", "[+semivowel ")
        .replace("Z[", "[+continuant ")
        .replace("”", "ˈ")
        .replace("C", "[+consonant]")
        .replace("V", "[+vowel]")
        .replace("A", "[+affricate]")
        .replace("B", "[+vowel +back]")
        .replace("D", "[+plosive +voice]")
        .replace("E", "[+vowel +front]")
        .replace("F", "[+fricative]")
        .replace("H", "[+laryngeal]")
        .replace("J", "[+approximant]")
        .replace("Ḱ", "[+palatovelar]")
        .replace("K", "[+velar]")
        .replace("L", "[+liquid]")
        .replace("M", "[+diphtong]")
        .replace("N", "[+nasal]")
        .replace("O", "[-sonorant]")
        .replace("P", "[+labial,bilabial]")
        .replace("Q", "[+uvular]")
        //.replace("Q", "[+click]")
        .replace("R", "[+sonorant]")
        .replace("S", "[+plosive]")
        .replace("T", "[+plosive -voice]")
        .replace("U", "σ")
        .replace("W", "[+semivowel]")
        .replace("Z", "[+continuant]")
}
