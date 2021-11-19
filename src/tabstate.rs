pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub fn new(titles: Vec<String>) -> TabsState {
        let mut indexed_titles: Vec<String> = Vec::new();
        for (title, idx) in titles.iter().zip(1..titles.len() + 1) {
            indexed_titles.push(format!("{}.{}", idx, title));
        }
        TabsState {
            titles: indexed_titles,
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn jump(&mut self, tab: usize) {
        if tab >= self.titles.len() {
            self.index = self.titles.len() - 1;
            return;
        }
        while self.index != tab - 1 {
            self.next();
        }
    }
}
