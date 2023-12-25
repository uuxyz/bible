const BIBLE_TEXT: &str = include_str!("bible.txt");
const VERSES: [[usize; 29]; 41] = [
    [
        31, 25, 24, 26, 32, 22, 24, 22, 29, 32, 32, 20, 18, 24, 21, 16, 27, 33, 38, 18, 34, 24, 20, 67,
        34, 35, 46, 22, 35,
    ],
    [
        43, 55, 32, 20, 31, 29, 43, 36, 30, 23, 23, 57, 38, 34, 34, 28, 34, 31, 22, 33, 26, 22, 25, 22,
        31, 23, 30, 25, 32,
    ],
    [
        35, 29, 10, 51, 22, 31, 27, 36, 16, 27, 25, 26, 36, 31, 33, 18, 40, 37, 21, 43, 46, 38, 18, 35,
        23, 35, 35, 38, 29,
    ],
    [
        31, 43, 38, 17, 16, 17, 35, 19, 30, 38, 36, 24, 20, 47, 8, 59, 57, 33, 34, 16, 30, 37, 27, 24,
        33, 44, 23, 55, 46,
    ],
    [
        34, 54, 34, 51, 49, 31, 27, 89, 26, 23, 36, 35, 16, 33, 45, 41, 50, 13, 32, 22, 29, 35, 41, 30,
        25, 18, 65, 23, 31,
    ],
    [
        40, 16, 54, 42, 56, 29, 34, 13, 46, 37, 29, 49, 33, 25, 26, 20, 29, 22, 32, 32, 18, 29, 23, 22,
        20, 22, 21, 20, 23,
    ],
    [
        30, 25, 22, 19, 19, 26, 68, 29, 20, 30, 52, 29, 12, 18, 24, 17, 24, 15, 27, 26, 35, 27, 43, 23,
        24, 33, 15, 63, 10,
    ],
    [
        18, 28, 51, 9, 45, 34, 16, 33, 36, 23, 31, 24, 31, 40, 25, 35, 57, 18, 40, 15, 25, 20, 20, 31,
        13, 31, 30, 48, 25,
    ],
    [
        22, 23, 18, 22, 28, 36, 21, 22, 12, 21, 17, 22, 27, 27, 15, 25, 23, 52, 35, 23, 58, 30, 24, 42,
        15, 23, 29, 22, 44,
    ],
    [
        25, 12, 25, 11, 31, 13, 27, 32, 39, 12, 25, 23, 29, 18, 13, 19, 27, 31, 39, 33, 37, 23, 29, 33,
        43, 26, 22, 51, 39,
    ],
    [
        25, 53, 46, 28, 34, 18, 38, 51, 66, 28, 29, 43, 33, 34, 31, 34, 34, 24, 46, 21, 43, 29, 53, 18,
        25, 27, 44, 27, 33,
    ],
    [
        20, 29, 37, 36, 21, 21, 25, 29, 38, 20, 41, 37, 37, 21, 26, 20, 37, 20, 30, 54, 55, 24, 43, 26,
        81, 40, 40, 44, 14,
    ],
    [
        47, 40, 14, 17, 29, 43, 27, 17, 19, 8, 30, 19, 32, 31, 31, 32, 34, 21, 30, 17, 18, 17, 22, 14,
        42, 22, 18, 31, 19,
    ],
    [
        23, 16, 22, 15, 19, 14, 19, 34, 11, 37, 20, 12, 21, 27, 28, 23, 9, 27, 36, 27, 21, 33, 25, 33,
        27, 23, 11, 70, 13,
    ],
    [
        24, 17, 22, 28, 36, 15, 44, 11, 20, 32, 23, 19, 19, 73, 18, 38, 39, 36, 47, 31, 22, 23, 15, 17,
        14, 14, 10, 17, 32,
    ],
    [
        3, 22, 13, 26, 21, 27, 30, 21, 22, 35, 22, 20, 25, 28, 22, 35, 22, 16, 21, 29, 29, 34, 30, 17,
        25, 6, 14, 23, 28,
    ],
    [
        25, 31, 40, 22, 33, 37, 16, 33, 24, 41, 30, 24, 34, 17, 6, 12, 8, 8, 12, 10, 17, 9, 20, 18,
        7, 8, 6, 7, 5,
    ],
    [
        11, 15, 50, 14, 9, 13, 31, 6, 10, 22, 12, 14, 9, 11, 12, 24, 11, 22, 22, 28, 12, 40, 22, 13,
        17, 13, 11, 5, 26,
    ],
    [
        17, 11, 9, 14, 20, 23, 19, 9, 6, 7, 23, 13, 11, 11, 17, 12, 8, 12, 11, 10, 13, 20, 7, 35,
        36, 5, 24, 20, 28,
    ],
    [
        23, 10, 12, 20, 72, 13, 19, 16, 8, 18, 12, 13, 17, 7, 18, 52, 17, 16, 15, 5, 23, 11, 13, 12,
        9, 9, 5, 8, 28,
    ],
    [
        22, 35, 45, 48, 43, 13, 31, 7, 10, 10, 9, 8, 18, 19, 2, 29, 176, 7, 8, 9, 4, 8, 5, 6, 5, 6,
        8, 8, 3,
    ],
    [
        18, 3, 3, 21, 26, 9, 8, 24, 13, 10, 7, 12, 15, 21, 10, 20, 14, 9, 6, 33, 22, 35, 27, 23,
        35, 27, 36, 18, 32,
    ],
    [
        31, 28, 25, 35, 33, 33, 28, 24, 29, 30, 31, 29, 35, 34, 28, 28, 27, 28, 27, 33, 31, 18, 26, 22,
        16, 20, 12, 29, 17,
    ],
    [
        18, 20, 10, 14, 17, 17, 11, 16, 16, 13, 13, 14, 31, 22, 26, 6, 30, 13, 25, 22, 21, 34, 16, 6,
        22, 32, 9, 14, 14,
    ],
    [
        7, 25, 6, 17, 25, 18, 23, 12, 21, 13, 29, 24, 33, 9, 20, 24, 17, 10, 22, 38, 22, 8, 31, 29,
        25, 28, 28, 25, 13,
    ],
    [
        15, 22, 26, 11, 23, 15, 12, 17, 13, 12, 21, 14, 21, 22, 11, 12, 19, 12, 25, 24, 19, 37, 25, 31,
        31, 30, 34, 22, 26,
    ],
    [
        25, 23, 17, 27, 22, 21, 21, 27, 23, 15, 18, 14, 30, 40, 10, 38, 24, 22, 17, 32, 24, 40, 44, 26,
        22, 19, 32, 21, 28,
    ],
    [
        18, 16, 18, 22, 13, 30, 5, 28, 7, 47, 39, 46, 64, 34, 22, 22, 66, 22, 22, 28, 10, 27, 17, 17,
        14, 27, 18, 11, 22,
    ],
    [
        25, 28, 23, 23, 8, 63, 24, 32, 14, 49, 32, 31, 49, 27, 17, 21, 36, 26, 21, 26, 18, 32, 33, 31,
        15, 38, 28, 23, 29,
    ],
    [
        49, 26, 20, 27, 31, 25, 24, 23, 35, 21, 49, 30, 37, 31, 28, 28, 27, 27, 21, 45, 13, 11, 23, 5,
        19, 15, 11, 16, 14,
    ],
    [
        17, 15, 12, 14, 16, 9, 20, 32, 21, 15, 16, 15, 13, 27, 14, 17, 14, 15, 21, 17, 10, 10, 11, 16,
        13, 12, 13, 15, 16,
    ],
    [
        20, 15, 13, 19, 17, 20, 19, 18, 15, 20, 15, 23, 21, 13, 10, 14, 11, 15, 14, 23, 17, 12, 17, 14,
        9, 21, 14, 17, 18,
    ],
    [
        6, 25, 23, 17, 25, 48, 34, 29, 34, 38, 42, 30, 50, 58, 36, 39, 28, 27, 35, 30, 34, 46, 46, 39,
        51, 46, 75, 66, 20,
    ],
    [
        45, 28, 35, 41, 43, 56, 37, 38, 50, 52, 33, 44, 37, 72, 47, 20, 80, 52, 38, 44, 39, 49, 50, 56,
        62, 42, 54, 59, 35,
    ],
    [
        35, 32, 31, 37, 43, 48, 47, 38, 71, 56, 53, 51, 25, 36, 54, 47, 71, 53, 59, 41, 42, 57, 50, 38,
        31, 27, 33, 26, 40,
    ],
    [
        42, 31, 25, 26, 47, 26, 37, 42, 15, 60, 40, 43, 48, 30, 25, 52, 28, 41, 40, 34, 28, 41, 38, 40,
        30, 35, 27, 27, 32,
    ],
    [
        44, 31, 32, 29, 31, 25, 21, 23, 25, 39, 33, 21, 36, 21, 14, 23, 33, 27, 31, 16, 23, 21, 13, 20,
        40, 13, 27, 33, 34,
    ],
    [
        31, 13, 40, 58, 24, 24, 17, 18, 18, 21, 18, 16, 24, 15, 18, 33, 21, 14, 24, 21, 29, 31, 26, 18,
        23, 22, 21, 32, 33,
    ],
    [
        24, 30, 30, 21, 23, 29, 23, 25, 18, 10, 20, 13, 18, 28, 12, 17, 18, 20, 15, 16, 16, 25, 21, 18,
        26, 17, 22, 16, 15,
    ],
    [
        15, 25, 14, 18, 19, 16, 14, 20, 28, 13, 28, 39, 40, 29, 25, 27, 26, 18, 17, 20, 25, 25, 22, 19,
        14, 21, 22, 18, 10,
    ],
    [
        29, 24, 21, 21, 13, 14, 25, 20, 29, 22, 11, 14, 17, 17, 13, 21, 11, 19, 17, 18, 20, 8, 21, 18,
        24, 21, 15, 27, 21,
    ],
];
struct Chapter {
    name: &'static str,
    pos: (usize, usize),
}

const CHAPTERS: [Chapter; 66] = [
    Chapter { name: "Genesis", pos: (0, 49) },
    Chapter { name: "Exodus", pos: (50, 89) },
    Chapter { name: "Leviticus", pos: (90, 116) },
    Chapter { name: "Numbers", pos: (117, 152) },
    Chapter { name: "Deuteronomy", pos: (153, 186) },
    Chapter { name: "Joshua", pos: (187, 210) },
    Chapter { name: "Judges", pos: (211, 231) },
    Chapter { name: "Ruth", pos: (232, 235) },
    Chapter { name: "1 Samuel", pos: (236, 266) },
    Chapter { name: "2 Samuel", pos: (267, 290) },
    Chapter { name: "1 Kings", pos: (291, 312) },
    Chapter { name: "2 Kings", pos: (313, 337) },
    Chapter { name: "1 Chronicles", pos: (338, 366) },
    Chapter { name: "2 Chronicles", pos: (367, 402) },
    Chapter { name: "Ezra", pos: (403, 412) },
    Chapter { name: "Nehemiah", pos: (413, 425) },
    Chapter { name: "Esther", pos: (426, 435) },
    Chapter { name: "Job", pos: (436, 477) },
    Chapter { name: "Psalms", pos: (478, 627) },
    Chapter { name: "Proverbs", pos: (628, 658) },
    Chapter { name: "Ecclesiastes", pos: (659, 670) },
    Chapter { name: "Song of Solomon", pos: (671, 678) },
    Chapter { name: "Isaiah", pos: (679, 744) },
    Chapter { name: "Jeremiah", pos: (745, 796) },
    Chapter { name: "Lamentations", pos: (797, 801) },
    Chapter { name: "Ezekiel", pos: (802, 849) },
    Chapter { name: "Daniel", pos: (850, 861) },
    Chapter { name: "Hosea", pos: (862, 875) },
    Chapter { name: "Joel", pos: (876, 878) },
    Chapter { name: "Amos", pos: (879, 887) },
    Chapter { name: "Obadiah", pos: (888, 888) },
    Chapter { name: "Jonah", pos: (889, 892) },
    Chapter { name: "Micah", pos: (893, 899) },
    Chapter { name: "Nahum", pos: (900, 902) },
    Chapter { name: "Habakkuk", pos: (903, 905) },
    Chapter { name: "Zephaniah", pos: (906, 908) },
    Chapter { name: "Haggai", pos: (909, 910) },
    Chapter { name: "Zechariah", pos: (911, 924) },
    Chapter { name: "Malachi", pos: (925, 928) },
    Chapter { name: "Matthew", pos: (929, 956) },
    Chapter { name: "Mark", pos: (957, 972) },
    Chapter { name: "Luke", pos: (973, 996) },
    Chapter { name: "John", pos: (997, 1017) },
    Chapter { name: "Acts", pos: (1018, 1045) },
    Chapter { name: "Romans", pos: (1046, 1061) },
    Chapter { name: "1 Corinthians", pos: (1062, 1077) },
    Chapter { name: "2 Corinthians", pos: (1078, 1090) },
    Chapter { name: "Galatians", pos: (1091, 1096) },
    Chapter { name: "Ephesians", pos: (1097, 1102) },
    Chapter { name: "Philippians", pos: (1103, 1106) },
    Chapter { name: "Colossians", pos: (1107, 1110) },
    Chapter { name: "1 Thessalonians", pos: (1111, 1115) },
    Chapter { name: "2 Thessalonians", pos: (1116, 1118) },
    Chapter { name: "1 Timothy", pos: (1119, 1124) },
    Chapter { name: "2 Timothy", pos: (1125, 1128) },
    Chapter { name: "Titus", pos: (1129, 1131) },
    Chapter { name: "Philemon", pos: (1132, 1132) },
    Chapter { name: "Hebrews", pos: (1133, 1145) },
    Chapter { name: "James", pos: (1146, 1150) },
    Chapter { name: "1 Peter", pos: (1151, 1155) },
    Chapter { name: "2 Peter", pos: (1156, 1158) },
    Chapter { name: "1 John", pos: (1159, 1163) },
    Chapter { name: "2 John", pos: (1164, 1164) },
    Chapter { name: "3 John", pos: (1165, 1165) },
    Chapter { name: "Jude", pos: (1166, 1166) },
    Chapter { name: "Revelation", pos: (1167, 1188) },
];

fn find_bible(nth_verse: usize) -> Result<(&'static str, usize, usize), &'static str> {
    let mut cnt: usize = 0;
    for chapter in &CHAPTERS {
        for i in chapter.pos.0..=chapter.pos.1 {
            let verse = VERSES[i / 29][i % 29];
            cnt += verse;
            if cnt > nth_verse {
                let chapter_n = i - chapter.pos.0 + 1;
                let verse_n = verse + nth_verse - cnt + 1;
                return Ok((chapter.name, chapter_n, verse_n));
            }
        }
    }
    Err("Verse not found")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("ERROR");
        return;
    }
    let mode_str = &args[1];
    let mut have_verse = false;
    let mut have_book_name = false;
    let mut have_chapter_num = false;
    let mut have_verse_num = false;
    match mode_str.parse::<i32>() {
        Ok(number) => {
            have_verse = (number & 1) != 0;
            have_book_name = (number & 2) != 0;
            have_chapter_num = (number & 4) != 0;
            have_verse_num = (number & 8) != 0;
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    let number_str = &args[2];
    match number_str.parse::<i32>() {
        Ok(number) => {
            let bible_index = number.rem_euclid(31102) as usize;
            match find_bible(bible_index) {
                Ok((book, chapter, verse)) => {
                    if have_book_name {
                        print!("{}", book);
                        if have_chapter_num || have_verse_num {
                            print!(" ");
                        }
                    }
                    if have_chapter_num {
                        print!("{}", chapter);
                    }
                    if have_chapter_num && have_verse_num {
                        print!(":");
                    }
                    if have_verse_num {
                        print!("{}", verse);
                    }
                    if have_verse {
                        if let Some(line) = BIBLE_TEXT.lines().nth(bible_index) {
                            if have_book_name || have_chapter_num || have_verse_num {
                                println!("");
                                println!("{}", line);
                            } else {
                                print!("{}", line);
                            }
                        } else {
                            println!("Line {} not found", number);
                        }
                    }
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
