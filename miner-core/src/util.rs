use rand::RngCore;

pub fn random_hex(bytes: usize) -> String {
    let mut buffer = vec![0u8; bytes];
    rand::rng().fill_bytes(&mut buffer);
    hex::encode(buffer)
}

pub fn random_device_id() -> String {
    random_hex(16)
}

pub fn normalize_login(input: &str) -> Option<String> {
    let text = input.trim().to_ascii_lowercase();

    let login = match text.find("twitch.tv/") {
        Some(at) => {
            let mut parts = text[at + "twitch.tv/".len()..]
                .split(['/', '?', '#'])
                .filter(|part| !part.is_empty());
            match parts.next() {
                Some("popout" | "moderator") => parts.next().unwrap_or(""),
                Some(part) => part,
                None => "",
            }
        }
        None => text
            .trim_start_matches('@')
            .split('?')
            .next()
            .unwrap_or("")
            .trim_end_matches('/'),
    };

    let trimmed = login.to_owned();

    if trimmed.is_empty()
        || trimmed.len() > 25
        || !trimmed
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    {
        return None;
    }

    Some(trimmed)
}

pub fn game_slug(name: &str) -> String {
    let mut slug = String::with_capacity(name.len());
    let mut dash = false;

    for ch in name.chars() {
        // ! Twitch drops apostrophes ("Assassin's Creed" is assassins-creed)
        // and writes accented letters plainly ("Pokémon" is pokemon).
        if matches!(ch, '\'' | '\u{2019}') {
            continue;
        }

        let ch = fold_accent(ch);

        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            dash = false;
        } else if !dash && !slug.is_empty() {
            slug.push('-');
            dash = true;
        }
    }

    slug.trim_end_matches('-').to_owned()
}

fn fold_accent(ch: char) -> char {
    match ch {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => 'a',
        'ç' | 'Ç' => 'c',
        'è' | 'é' | 'ê' | 'ë' | 'È' | 'É' | 'Ê' | 'Ë' => 'e',
        'ì' | 'í' | 'î' | 'ï' | 'Ì' | 'Í' | 'Î' | 'Ï' => 'i',
        'ñ' | 'Ñ' => 'n',
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => 'o',
        'ù' | 'ú' | 'û' | 'ü' | 'Ù' | 'Ú' | 'Û' | 'Ü' => 'u',
        'ý' | 'ÿ' | 'Ý' => 'y',
        _ => ch,
    }
}
