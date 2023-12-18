use anyhow::Result;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Name};
use ureq;

fn main() {
    let page = make_request().expect("");
    let _ = scrape(page.as_str());
}
fn make_request() -> Result<String> {
    let url = "https://books.toscrape.com/";
    let agent = ureq::AgentBuilder::new().build();
    let resp = agent.get(url).call()?;
    let page = resp.into_string()?;
    Ok(page)
}
fn scrape(page: &str) {
    let doc = Document::from(page);
    let mut main_div = Vec::new();
    let _get_divs = doc
        .find(Attr("class", "col-sm-8 col-md-9"))
        .map(|class| {
            main_div.push(class);
        })
        .collect::<Vec<()>>();

    let mut section: Vec<Node<'_>> = Vec::new();
    let _get_section = main_div
        .iter()
        .map(|div| {
            div.find(Name("section"))
                .map(|sect| {
                    section.push(sect);
                })
                .collect::<Vec<()>>()
        })
        .collect::<Vec<Vec<()>>>();

    let mut h3: Vec<Node<'_>> = Vec::new();
    let _get_h3 = section
        .iter()
        .map(|li| {
            li.find(Name("h3"))
                .map(|h3s| {
                    h3.push(h3s);
                })
                .collect::<Vec<()>>()
        })
        .collect::<Vec<Vec<()>>>();

    let mut title = Vec::new();
    let _get_a = h3
        .iter()
        .map(|h3| {
            h3.find(Name("a"))
                .map(|a| match a.attr("title") {
                    Some(val) => title.push(val),
                    None => (),
                })
                .collect::<Vec<()>>()
        })
        .collect::<Vec<Vec<()>>>();
    dbg!(&title);
}
