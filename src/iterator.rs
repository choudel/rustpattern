struct BrowserHistory {
    urls: [&'static str; 3],
}
impl BrowserHistory {
    pub fn new() -> Self {
        Self {
            urls: ["yesterday", "today", "this moment"],
        }
    }
    pub fn iter(&self) -> BrowserIterator {
        BrowserIterator {
            index: 0,
            browserHistory: self,
        }
    }
}
pub struct BrowserIterator<'a> {
    index: usize,
    browserHistory: &'a BrowserHistory,
}
impl Iterator for BrowserIterator<'_> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.browserHistory.urls.len() {
            let urls = Some(self.browserHistory.urls[self.index]);
            self.index += 1;
            return urls;
        }
        None
    }
}

fn main() {
    let urls = BrowserHistory::new();
    let mut iterator = urls.iter();
    println!("1 elemnt:{:?}", iterator.next());
}
