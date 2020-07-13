use crate::schema::posts;
use chrono::prelude::*;
use encoding_rs::SHIFT_JIS;
use pwhash::unix_crypt;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub name: String,
    pub tripcode: String,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub user_uuid: Option<String>,
}

#[derive(Identifiable, Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub tripcode: String,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub user_uuid: Option<String>,
}

impl Post {
    fn create_tripcode(password: &str) -> String {
        let (recoded, ..) = SHIFT_JIS.encode(password);
        let password = recoded.to_vec();

        let mut salt = password.clone();
        salt.push('H' as u8);
        salt.push('.' as u8);
        salt.push('.' as u8);

        let salt: String = salt
            .into_iter()
            .skip(1)
            .take(2)
            .map(|x| match x as char {
                ':' => 'A',
                ';' => 'B',
                '<' => 'C',
                '=' => 'D',
                '>' => 'E',
                '?' => 'F',
                '@' => 'G',
                '[' => 'a',
                '\\' => 'b',
                ']' => 'c',
                '^' => 'd',
                '_' => 'e',
                '`' => 'f',
                '.'..='z' => x as char,
                _ => '.',
            })
            .collect();

        #[allow(deprecated)]
        let hash = unix_crypt::hash_with(&salt, password).unwrap();

        let len = hash.len();
        hash.chars().skip(len - 10).collect()
    }

    fn process_name(name: &str, tripcode: &str) -> (String, String) {
        let hash_index = name.chars().position(|ch| ch == '#');
        match hash_index {
            Some(index) => {
                let password: String = name.chars().skip(index + 1).collect();
                let tripcode = Post::create_tripcode(&password);
                let name: String = name.chars().take(index).collect();
                (name, tripcode)
            }
            None => {
                let name = name.to_string();
                let tripcode = if tripcode.len() > 0 {
                    Post::create_tripcode(&tripcode)
                } else {
                    "".to_string()
                };
                (name, tripcode)
            }
        }
    }

    pub fn new(
        name: &str,
        tripcode: &str,
        message: &str,
        user_uuid: Option<&str>,
    ) -> Result<NewPost, String> {
        let (name, tripcode) = Post::process_name(name, tripcode);

        if message.len() >= 8000 {
            return Err(String::from("Message is too long"));
        }

        Ok(NewPost {
            name,
            tripcode,
            message: message.to_string(),
            created_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
            user_uuid: user_uuid.map(String::from),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Post;

    #[test]
    fn create_tripcode() {
        for &(password, expected) in [
            ("oQVCdI0.", "ViOlet.A4U"),
            ("*Tp0tp8[", "LLLLLLLLL."),
            ("KQc/IH~a", "x/IrohaN9I"),
            ("p,A|\\mJS", "DesuIR/aK."),
            ("\'IR;(6\\*", "6xAkarinDE"),
            ("A`drY1*E", "Cat//Qft12"),
            ("&4$:#*8#", "cOde//1bGM"),
        ]
        .iter()
        {
            let tripcode = Post::create_tripcode(password);
            assert_eq!(expected, tripcode);
        }
    }

    #[test]
    fn create_tripcode_with_empty_password() {
        let tripcode = Post::create_tripcode("");
        assert_eq!("8NBuQ4l6uQ", tripcode);
    }

    #[test]
    fn create_tripcode_with_short_password() {
        let tripcode = Post::create_tripcode("mdUF");
        assert_eq!("WatacYfoiE", tripcode);
    }

    #[test]
    fn create_tripcode_with_long_password() {
        let tripcode = Post::create_tripcode("Contagious");
        assert_eq!("JemGwef5bo", tripcode);
    }

    #[test]
    fn create_tripcode_with_kanji_password() {
        for &(password, expected) in [
            ("\'黽剱鷽ム", "WaTASHIjBE"),
            ("//膵囘\\叩0・", "mNkAsylumY"),
            ("果ｪｫq^ｨ機0・", "dAsylumUuU"),
            ("//腥奸`院0・", "XvAsylumD6"),
            ("2ﾜT怦J轤", "XtAsylum5s"),
        ]
        .iter()
        {
            let tripcode = Post::create_tripcode(password);
            assert_eq!(expected, tripcode);
        }
    }

    #[test]
    fn process_name_without_password() {
        let (name, tripcode) = Post::process_name("username", "");

        assert_eq!("username", name);
        assert_eq!("", tripcode);
    }

    #[test]
    fn process_name_with_empty_password() {
        let (name, tripcode) = Post::process_name("username#", "");

        assert_eq!("username", name);
        assert_eq!("8NBuQ4l6uQ", tripcode);
    }

    #[test]
    fn process_name_with_password() {
        let (name, tripcode) = Post::process_name("username#*Tp0tp8[", "");

        assert_eq!("username", name);
        assert_eq!("LLLLLLLLL.", tripcode);
    }

    #[test]
    fn process_name_with_password_only() {
        let (name, tripcode) = Post::process_name("", "*Tp0tp8[");

        assert_eq!("", name);
        assert_eq!("LLLLLLLLL.", tripcode);
    }

    #[test]
    fn process_name_with_separate_password() {
        let (name, tripcode) = Post::process_name("username", "*Tp0tp8[");

        assert_eq!("username", name);
        assert_eq!("LLLLLLLLL.", tripcode);
    }
}
