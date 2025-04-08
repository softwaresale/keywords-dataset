use crate::err::{AppError, AppResult};
use lopdf::{Destination, Document, Encoding, Object, ObjectId, Outline};
use std::path::Path;
use indexmap::IndexMap;
use lopdf::content::Operation;

pub fn extract_keywords_from_pdf_file<PathT: AsRef<Path>>(path: PathT) -> AppResult<()> {

    let document = Document::load(path)?;
    let root_catalog = document.catalog()?;
    println!("root: {:?}", root_catalog);

    let mut dests = IndexMap::new();
    let outlines = match document.get_outlines(None, None, &mut dests) {
        Ok(Some(outlines)) => {
            println!("found {} outlines", outlines.len());
            outlines
        },
        Ok(None) => {
            println!("no error, but no outline");
            return Ok(());
        }
        Err(err) => {
            eprintln!("error while getting outline: {}", err);
            return Err(err.into());
        }
    };

    let abstract_dest = find_section(outlines, "Abstract")?;
    println!("abstract dest: {:?}", abstract_dest);
    
    let page = abstract_dest.page()?;
    println!("abstract page: {:?}", page);
    println!("abstract page type: {}", page.enum_variant());

    let abstract_page_ref = page.as_reference()?;
    println!("abstract page ref: {:?}", abstract_page_ref);

    Ok(())
}

fn find_section(outlines: Vec<Outline>, section_title: &str) -> AppResult<Destination> {
    let mut traversal = Vec::<Outline>::from(outlines);
    while let Some(top) = traversal.pop() {
        match top {
            Outline::Destination(dest) => {
                let title_str = decode_outline_title(&dest)?;
                if &title_str == section_title {
                    return Ok(dest);
                }
            }
            Outline::SubOutlines(_) => {}
        }
    }

    Err(AppError::Other("no destination found".to_string()))
}

fn decode_outline_title(outline: &Destination) -> AppResult<String> {
    let encoding = Encoding::SimpleEncoding(b"UniGB-UCS2-H");
    let title = outline.title()?;
    Document::decode_text(&encoding, title.as_str()?)
        .map_err(|err| err.into())
}
