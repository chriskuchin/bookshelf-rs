use std::collections::HashMap;

pub const MIME_TYPES: [(&'static str, &'static str); 6] = [
    ("azw3", "application/x-mobi8-ebook"),
    ("epub", "application/epub+zip"),
    ("fb2", "application/fb2+zip"),
    ("mobi", "application/x-mobipocket-ebook"),
    ("pdf", "application/pdf"),
    ("txt", "text/plain"),
];

pub fn ext_to_mime(ext: &String) -> Option<String> {
    for (extension, mime) in MIME_TYPES {
        if ext.ends_with(extension) {
            return Some(mime.to_string());
        }
    }
    None
}

pub fn list_mimes() -> HashMap<String, String> {
    let mut mimes = HashMap::new();
    for (extension, mime) in MIME_TYPES {
        mimes.insert(format!(".{}", extension).to_string(), mime.to_string());
    }

    mimes
}

pub fn mime_to_ext(mime: &str) -> String {
    for (extension, mime) in MIME_TYPES {
        if mime == extension {
            return String::from(extension);
        }
    }

    String::from(mime)
}
