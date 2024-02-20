enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

fn library_publications(publications:Vec<Publication>) {
    for publication in publications {
        match publication {
            
        }
    }
}