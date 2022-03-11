use epub::doc::EpubDoc;

fn main() {
    let doc = EpubDoc::new("test.epub");
    let mut doc = doc.unwrap();
    doc.go_next();
    doc.go_next();
    let title = doc.mdata("title");
    let data = doc.get_resource_str(&doc.get_current_id().unwrap());
    println!("{:#?}", title);
    println!("{:#?}", doc.get_current_page());
    println!("{:#?}", data);
}
