use std::collections::VecDeque;

#[derive(Clone)]
pub struct SimpleList {
    list: Vec<usize>
}

impl SimpleList {
    pub fn new() -> Self {
        SimpleList {
            list: Vec::new()
        }
    }

    fn access(&mut self, search: usize) -> usize {
        match self.list.iter().position(|&x| x == search) {
            Some(index) => index + 1,
            None => { 
                self.list.push(search); 
                self.list.len() - 1 
            }
        }
    }
}

#[derive(Clone)]

pub struct MtfList {
    list: VecDeque<usize>
}

impl MtfList {
    pub fn new() -> Self {
        MtfList {
            list: VecDeque::new()
        }
    }

    fn access(&mut self, search: usize) -> usize {
        match self.list.iter().position(|&x| x == search) {
            Some(index) => {
                self.list.remove(index);
                self.list.push_front(search);
                index + 1
            },
            None => {
                self.list.push_front(search);
                self.list.len() - 1
            }
        }
    }
}

#[derive(Clone)]

pub struct TransList {
    list: Vec<usize>
}

impl TransList {
    pub fn new() -> Self {
        TransList {
            list: Vec::new()
        }
    }

    fn access(&mut self, search: usize) -> usize {
        match self.list.iter().position(|&x| x == search) {
            Some(index) => {
                if index > 0 { 
                    self.list.swap(index, index - 1);
                }
                index + 1
            },
            None => {
                self.list.push(search);
                self.list.len() - 1
            }
        }
    }
}

#[derive(Clone)]
pub struct CountList {
    list: Vec<(usize, usize)>
}

impl CountList {
    pub fn new() -> Self {
        CountList {
            list: Vec::new()
        }
    }

    fn access(&mut self, search: usize) -> usize {
        match self.list.iter().position(|&x| x.1 == search ) {
            Some(index) => {
                self.list[index].0 += 1;
                let count = self.list[index].0;
                while index > 0 && self.list[index - 1].0 < count { 
                    self.list.swap(index, index - 1);
                }
                index + 1
            },
            None => {
                self.list.push((1, search));
                self.list.len() - 1
            }
        }
    }
}

#[derive(Clone)]

pub enum AccessList {
    Simple(SimpleList),
    Mtf(MtfList),
    Trans(TransList),
    Count(CountList)
}

impl AccessList {
    pub fn access(&mut self, search: usize) -> usize {
        match self {
            AccessList::Simple(list)=> list.access(search),
            AccessList::Mtf(list)      => list.access(search),
            AccessList::Trans(list)  => list.access(search),
            AccessList::Count(list)  => list.access(search)
        }
    }
}