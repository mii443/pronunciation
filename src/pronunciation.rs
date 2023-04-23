use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}};

use wana_kana::ConvertJapanese;

macro_rules! cm {
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$((String::from($k), $v),)*])
    }};
}

macro_rules! c {
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$((String::from($k), String::from($v)),)*])
    }};
}

macro_rules! svec {
    ($($k:expr),* $(,)?) => {{
        vec![$(String::from($k),)*]
    }};
}

pub struct Pronunciation {
    pub pronunciation_map: HashMap<String, Vec<String>>,
    vowels: Vec<String>,
    dictionary: HashMap<String, HashMap<String, String>>
}

impl Pronunciation {
    pub fn phoneme_to_kana(&self, phonemes: &Vec<String>) -> String {
        let mut kana = String::default();
        let mut bef: Option<String> = None;
        for (i, phoneme) in phonemes.iter().enumerate() {
            let m = bef.clone().unwrap_or(String::default());
    
            if m == String::from("") && self.vowels.contains(phonemes.get(i + 1).unwrap_or(&String::default())) {
                bef = Some(phoneme.clone());
                continue;
            }
            
            let kanas = if m == String::from("") {
                self.dictionary[phoneme][&String::default()].clone()
            } else {
                self.dictionary[&m][phoneme].clone()
            };
            bef = None;
            kana += &kanas;
    
            //println!("{}:{}:{}", m, phoneme, kanas);
        }
        //println!("{}: {}", word, kana);
        kana
    }

    pub fn get_kana(&self, word: String) -> String {
        if let Some(phonemes) = self.pronunciation_map.get(&word.to_uppercase()) {
            self.phoneme_to_kana(phonemes)
        } else {
            word.to_kana().to_katakana()
        }
    }

    pub fn new(dict_file: &str) -> Self {
        let file = File::open(dict_file).unwrap();
        let reader = BufReader::new(file);
    
        let mut pronunciation_map: HashMap<String, Vec<String>> = HashMap::new();
    
        for line in reader.lines() {
            if let Ok(line) = line {
                let t: Vec<String> = line.split_whitespace().map(str::to_string).collect();
                pronunciation_map.insert(t[0].clone(), t[1..].to_vec());
            }
        }

        let vowels = svec!["AA","AH","AE","AW","AY","ER","IY","IH","UH","UW","EH","EY","AO","OW","OY"];
    
        let dictionary: HashMap<String, HashMap<String, String>> = cm! {
            "ZH" => c! {
                "AA" => "ジャ",
                "AH" => "ジョ",
                "AE" => "ジャ",
                "AW" => "ジャ",
                "AY" => "ジャイ",
                "ER" => "ジェ",
                "IY" => "ジ",
                "IH" => "ジ",
                "UH" => "ジュ",
                "UW" => "ジュ",
                "EH" => "ジェ",
                "EY" => "ジェ",
                "AO" => "ジョ",
                "OW" => "ジョ",
                "OY" => "ジョ",
                "" => "ジュ"
            },
            "DH" => c! {
                "AA" => "ザ",
                "AH" => "ザ",
                "AE" => "ザ",
                "AW" => "ザ",
                "AY" => "ザイ",
                "ER" => "ザー",
                "IY" => "ジ",
                "IH" => "ジ",
                "UH" => "ズ",
                "UW" => "ズ",
                "EH" => "ゼ",
                "EY" => "ゼ",
                "AO" => "ゾ",
                "OW" => "ゾ",
                "OY" => "ゾ",
                "" => "ズ"
            },
            "W" => c! {
                "AA" => "ワ",
                "AH" => "ワ",
                "AE" => "ワ",
                "AW" => "ワ",
                "AY" => "ワイ",
                "ER" => "ウィ",
                "IY" => "ウィ",
                "IH" => "ウィ",
                "UH" => "ウ",
                "UW" => "ウ",
                "EH" => "ウェ",
                "EY" => "ウェ",
                "AO" => "ウォ",
                "OW" => "ウォ",
                "OY" => "ウォ",
                "" => "ウ"
            },
            "NG" => c! {
                "AA" => "ンガ",
                "AH" => "ンガ",
                "AE" => "ンガ",
                "AW" => "ンガ",
                "AY" => "ンガイ",
                "ER" => "ンギ",
                "IY" => "ンギ",
                "IH" => "ンギ",
                "UH" => "ング",
                "UW" => "ング",
                "EH" => "ンゲ",
                "EY" => "ンゲ",
                "AO" => "ンゴ",
                "OW" => "ンゴ",
                "OY" => "ンゴ",
                "" => "ング"
            },
            "Y" => c! {
                "AA" => "ア",
                "AH" => "ア",
                "AE" => "ア",
                "AW" => "ア",
                "AY" => "アイ",
                "ER" => "イ",
                "IY" => "イ",
                "IH" => "イ",
                "UH" => "ュ",
                "UW" => "ュ",
                "EH" => "エ",
                "EY" => "エ",
                "AO" => "ョ",
                "OW" => "ョ",
                "OY" => "ョ",
                "" => "イ"
            },
            "TH" => c! {
                "AA" => "サ",
                "AH" => "サ",
                "AE" => "サ",
                "AW" => "サ",
                "AY" => "サイ",
                "ER" => "シ",
                "IY" => "シ",
                "IH" => "シ",
                "UH" => "ス",
                "UW" => "ス",
                "EH" => "セ",
                "EY" => "セ",
                "AO" => "ソ",
                "OW" => "ソ",
                "OY" => "ソ",
                "" => "ス"
            },
            "G" => c! {
                "AA" => "ガ",
                "AH" => "ガ",
                "AE" => "ガ",
                "AW" => "ガ",
                "AY" => "ガイ",
                "ER" => "ギ",
                "IY" => "ギ",
                "IH" => "ギ",
                "UH" => "グ",
                "UW" => "グ",
                "EH" => "ゲ",
                "EY" => "ゲ",
                "AO" => "ゴ",
                "OW" => "ゴ",
                "OY" => "ゴ",
                "" => "グ"
            },
            "CH" => c! {
                "AA" => "チャ",
                "AH" => "チャ",
                "AE" => "チャ",
                "AW" => "チャ",
                "AY" => "チャイ",
                "ER" => "チ",
                "IY" => "チ",
                "IH" => "チ",
                "UH" => "チュ",
                "UW" => "チュ",
                "EH" => "チェ",
                "EY" => "チェ",
                "AO" => "チョ",
                "OW" => "チョ",
                "OY" => "チョ",
                "" => "チ"
            },
            "D" => c! {
                "AA" => "ダ",
                "AH" => "ダ",
                "AE" => "ダ",
                "AW" => "ダ",
                "AY" => "ダイ",
                "ER" => "ダー",
                "IY" => "ディ",
                "IH" => "ディ",
                "UH" => "ドゥ",
                "UW" => "ドゥ",
                "EH" => "デ",
                "EY" => "デ",
                "AO" => "ド",
                "OW" => "ド",
                "OY" => "ド",
                "" => "ド"
            },
            "B" => c! {
                "AA" => "バ",
                "AH" => "バ",
                "AE" => "バ",
                "AW" => "バウ",
                "AY" => "バイ",
                "ER" => "ビ",
                "IY" => "ビ",
                "IH" => "ビ",
                "UH" => "ブ",
                "UW" => "ブ",
                "EH" => "ベ",
                "EY" => "ベ",
                "AO" => "ボ",
                "OW" => "ボ",
                "OY" => "ボ",
                "" => "ブ"
            },
            "SH" => c! {
                "AA" => "シャ",
                "AH" => "ショ",
                "AE" => "シャ",
                "AW" => "シャ",
                "AY" => "シャイ",
                "ER" => "シ",
                "IY" => "シー",
                "IH" => "シ",
                "UH" => "シュ",
                "UW" => "シュ",
                "EH" => "シェ",
                "EY" => "シェ",
                "AO" => "ショ",
                "OW" => "ショ",
                "OY" => "ショ",
                "" => "シ"
            },
            "F" => c! {
                "AA" => "ファ",
                "AH" => "ファ",
                "AE" => "ファ",
                "AW" => "ファ",
                "AY" => "ファイ",
                "ER" => "フィ",
                "IY" => "フィ",
                "IH" => "フィ",
                "UH" => "フ",
                "UW" => "フ",
                "EH" => "フェ",
                "EY" => "フェ",
                "AO" => "フォ",
                "OW" => "フォ",
                "OY" => "フォ",
                "" => "フ"
            },
            "K" => c! {
                "AA" => "カ",
                "AH" => "カ",
                "AE" => "カ",
                "AW" => "カ",
                "AY" => "カイ",
                "ER" => "キ",
                "IY" => "キ",
                "IH" => "キ",
                "UH" => "ク",
                "UW" => "ク",
                "EH" => "ケ",
                "EY" => "ケ",
                "AO" => "コ",
                "OW" => "コ",
                "OY" => "コ",
                "" => "ク"
            },
            "M" => c! {
                "AA" => "マ",
                "AH" => "マ",
                "AE" => "マ",
                "AW" => "マウ",
                "AY" => "マイ",
                "ER" => "ミ",
                "IY" => "ミ",
                "IH" => "ミ",
                "UH" => "ム",
                "UW" => "ム",
                "EH" => "メ",
                "EY" => "メ",
                "AO" => "モ",
                "OW" => "モ",
                "OY" => "モ",
                "" => "ム"
            },
            "R" => c! {
                "AA" => "ラ",
                "AH" => "ラ",
                "AE" => "ラ",
                "AW" => "ラ",
                "AY" => "ライ",
                "ER" => "リ",
                "IY" => "リ",
                "IH" => "リ",
                "UH" => "ル",
                "UW" => "ル",
                "EH" => "レ",
                "EY" => "レ",
                "AO" => "ロ",
                "OW" => "ロ",
                "OY" => "ロ",
                "" => "ー"
            },
            "V" => c! {
                "AA" => "バ",
                "AH" => "バ",
                "AE" => "ヴァ",
                "AW" => "バ",
                "AY" => "バイ",
                "ER" => "ビ",
                "IY" => "ビ",
                "IH" => "ビ",
                "UH" => "ブ",
                "UW" => "ブ",
                "EH" => "ベ",
                "EY" => "ベ",
                "AO" => "ボ",
                "OW" => "ボ",
                "OY" => "ボ",
                "" => "ブ"
            },
            "Z" => c! {
                "AA" => "ザ",
                "AH" => "ザ",
                "AE" => "ザ",
                "AW" => "ザ",
                "AY" => "ザイ",
                "ER" => "ザー",
                "IY" => "ジ",
                "IH" => "ジ",
                "UH" => "ズ",
                "UW" => "ズ",
                "EH" => "ゼ",
                "EY" => "ゼ",
                "AO" => "ゾ",
                "OW" => "ゾ",
                "OY" => "ゾ",
                "" => "ズ"
            },
            "N" => c! {
                "AA" => "ナ",
                "AH" => "ナ",
                "AE" => "ナ",
                "AW" => "ナ",
                "AY" => "ナイ",
                "ER" => "ニ",
                "IY" => "ニー",
                "IH" => "ニ",
                "UH" => "ヌ",
                "UW" => "ヌ",
                "EH" => "ネ",
                "EY" => "ネ",
                "AO" => "ノ",
                "OW" => "ノ",
                "OY" => "ノ",
                "" => "ン"
            },
            "P" => c! {
                "AA" => "パ",
                "AH" => "パ",
                "AE" => "パ",
                "AW" => "パ",
                "AY" => "パイ",
                "ER" => "ピ",
                "IY" => "ピ",
                "IH" => "ピ",
                "UH" => "プ",
                "UW" => "プ",
                "EH" => "ペ",
                "EY" => "ペ",
                "AO" => "ポ",
                "OW" => "ポ",
                "OY" => "ポ",
                "" => "プ"
            },
            "JH" => c! {
                "AA" => "ジャ",
                "AH" => "ジャ",
                "AE" => "ジャ",
                "AW" => "ジャ",
                "AY" => "ジャイ",
                "ER" => "ジ",
                "IY" => "ジ",
                "IH" => "ジ",
                "UH" => "ジュ",
                "UW" => "ジュ",
                "EH" => "ジェ",
                "EY" => "ジェ",
                "AO" => "ジョ",
                "OW" => "ジョ",
                "OY" => "ジョ",
                "" => "ジ"
            },
            "L" => c! {
                "AA" => "ラ",
                "AH" => "ラ",
                "AE" => "ラ",
                "AW" => "ラ",
                "AY" => "ライ",
                "ER" => "ラー",
                "IY" => "リー",
                "IH" => "リ",
                "UH" => "ル",
                "UW" => "ル",
                "EH" => "レ",
                "EY" => "レ",
                "AO" => "ロ",
                "OW" => "ロー",
                "OY" => "ロ",
                "" => "ル"
            },
            "HH" => c! {
                "AA" => "ハ",
                "AH" => "ハ",
                "AE" => "ハ",
                "AW" => "ハウ",
                "AY" => "ハイ",
                "ER" => "ハリ",
                "IY" => "ヒ",
                "IH" => "ヒ",
                "UH" => "フ",
                "UW" => "フ",
                "EH" => "ヘ",
                "EY" => "ヘ",
                "AO" => "ホ",
                "OW" => "ホ",
                "OY" => "ホ",
                "" => "フ"
            },
            "S" => c! {
                "AA" => "タ",
                "AH" => "タ",
                "AE" => "サ",
                "AW" => "サ",
                "AY" => "サイ",
                "ER" => "サ",
                "IY" => "シ",
                "IH" => "シ",
                "UH" => "ス",
                "UW" => "ス",
                "EH" => "セ",
                "EY" => "セイ",
                "AO" => "ソ",
                "OW" => "ソ",
                "OY" => "ソ",
                "" => "ス"
            },
            "T" => c! {
                "AA" => "トッ",
                "AH" => "タ",
                "AE" => "タ",
                "AW" => "タ",
                "AY" => "タイ",
                "ER" => "タ",
                "IY" => "ティ",
                "IH" => "ティ",
                "UH" => "チュ",
                "UW" => "チュ",
                "EH" => "テ",
                "EY" => "テ",
                "AO" => "ト",
                "OW" => "ト",
                "OY" => "ト",
                "" => "ト"
            },
            "EY" => c! {
                "" => "エイ"
            },
            "AW" => c! {
                "" => "オウ"
            },
            "AA" => c! {
                "" => "ア"
            },
            "AH" => c! {
                "ER" => "アエル",
                "OW" => "アッアウ",
                "" => "ア"
            },
            "IH" => c! {
                "" => "イ"
            },
            "EH" => c! {
                "OW" => "エオ",
                "" => "エ"
            },
            "AE" => c! {
                "" => "ア"
            },
            "OW" => c! {
                "AH" => "オア",
                "IH" => "オウィ",
                "ER" => "オウェ",
                "EH" => "オエ",
                "" => "オ"
            },
            "IY" => c! {
                "AA" => "イア",
                "IY" => "イイ",
                "EY" => "イー",
                "AH" => "イア",
                "AE" => "イア",
                "AO" => "イオ",
                "ER" => "アイヤー",
                "EH" => "イエ",
                "IH" => "ー",
                "UW" => "イウ",
                "OW" => "イオ",
                "" => "イー"
            },
            "AY" => c! {
                "AH" => "アイア",
                "AA" => "アイ",
                "AW" => "アイオウ",
                "AE" => "アイェ",
                "ER" => "アイア",
                "IH" => "アイイ",
                "EH" => "アイ",
                "IY" => "ウイェ",
                "UW" => "アユ",
                "OW" => "アイオ",
                "EY" => "ウイェ",
                "" => "アイ"
            },
            "ER" => c! {
                "AA" => "ア",
                "AY" => "アライ",
                "AH" => "ア",
                "AE" => "アラ",
                "AW" => "アラウ",
                "AO" => "アロ",
                "EY" => "アレイ",
                "ER" => "アー",
                "EH" => "オレ",
                "UH" => "オロウ",
                "OW" => "アロ",
                "OY" => "アロイ",
                "UW" => "ウル",
                "IH" => "エリ",
                "IY" => "エリ",
                "" => "アー"
            },
            "AO" => c! {
                "EH" => "アオエ",
                "" => "オ"
            },
            "EY" => c! {
                "AA" => "エイアー",
                "AH" => "エイ",
                "ER" => "エアー",
                "EY" => "アー",
                "IY" => "エイ",
                "EH" => "エイ",
                "AO" => "エイオ",
                "OW" => "アオ",
                "AW" => "アヨウ",
                "" => "エイ"
            },
            "OY" => c! {
                "ER" => "オイヤー",
                "OW" => "オヨ",
                "IH" => "オイエ",
                "" => "オイ"
            },
            "UW" => c! {
                "AA" => "ウア",
                "AH" => "ウー",
                "ER" => "ウアー",
                "EY" => "ウエ",
                "IY" => "ウイ",
                "IH" => "ウエ",
                "" => "ウ"
            },
            "AA" => c! {
                "UW" => "オウ",
                "IY" => "アイ",
                "" => "アー"
            },
            "AW" => c! {
                "AH" => "アウア",
                "IY" => "アオイ",
                "UW" => "アオウ",
                "ER" => "アワー",
                "IH" => "アウィ",
                "" => "オウ"
            },
            "UH" => c! {
                "AH" => "ウー",
                "" => "ウ"
            },
            "OW" => c! {
                "AA" => "オア",
                "AO" => "オウォ",
                "AH" => "オア",
                "AE" => "オエ",
                "IY" => "オイ",
                "IH" => "オーウィ",
                "UH" => "オウ",
                "EY" => "オウエイ",
                "EH" => "オフエ",
                "" => "オー"
            },
        };

        Self {
            pronunciation_map,
            vowels,
            dictionary
        }
    }
}