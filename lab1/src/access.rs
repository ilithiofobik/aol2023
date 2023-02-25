use std::collections::VecDeque;

pub trait Accessible {
    fn new() -> Self;
    fn access(&mut self, search: usize) -> usize;
}

#[derive(Clone)]
pub struct SimpleList {
    list: Vec<usize>
}

impl Accessible for SimpleList {
    fn new() -> Self {
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

impl Accessible for MtfList {
    fn new() -> Self {
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

impl Accessible for TransList {
    fn new() -> Self {
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

impl Accessible for CountList {
    fn new() -> Self {
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
    SimpleList(SimpleList),
    MtfList(MtfList),
    TransList(TransList),
    CountList(CountList)
}

impl Accessible for AccessList {
    fn new() -> Self {
        AccessList::SimpleList(SimpleList::new())
    }

    fn access(&mut self, search: usize) -> usize {
        match self {
            AccessList::SimpleList(list)=> list.access(search),
            AccessList::MtfList(list)      => list.access(search),
            AccessList::TransList(list)  => list.access(search),
            AccessList::CountList(list)  => list.access(search)
        }
    }
}