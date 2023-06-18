use fastrand::Rng;

const NUM_OF_PROCESSORS : usize = 64;
const SEQ_LENGTH        : usize = 65536;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Increasing,
    Decreasing,
    Holding,
}

#[derive(Clone, Copy)]
struct Processor {
    count    : usize,
    state    : State
}

impl Processor {
    pub fn new_empty() -> Processor {
        Processor {
            count    : 0,
            state    : State::Increasing
        }
    }

    pub fn new_full() -> Processor {
        Processor {
            count    : 0,
            state    : State::Holding
        }
    }

    pub fn has_copy(&self) -> bool {
        !matches!(self.state, State::Increasing)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Request {
    Read,
    Write
}

fn init_processors() -> [Processor; NUM_OF_PROCESSORS] {
    let mut processors = [Processor::new_empty(); NUM_OF_PROCESSORS];
    processors[0] = Processor::new_full();
    processors
}

fn rand_processor(rand: &mut Rng) -> usize {
    rand.usize(0..NUM_OF_PROCESSORS)
}

fn rand_request(rand: &mut Rng, p: f64) -> Request {
    if rand.f64() < p {
        Request::Write
    } else {
        Request::Read
    }
}

fn is_waiting(num_of_copies: usize, processors: &[Processor]) -> bool {
    (num_of_copies == 1) && processors.iter().any(|p| p.state == State::Holding)
}

pub fn page_allocation(d: usize, p: f64) -> (f64, f64) {
    let mut total_cost = 0;
    let mut curr_copies = 1;
    let mut max_copies = 1; 
    let mut rand = Rng::new();
    let mut processors = init_processors();

    for _ in 0..SEQ_LENGTH {
        let pid = rand_processor(&mut rand);
        let request = rand_request(&mut rand, p);

        match request {
            Request::Write => {
                total_cost += if processors[pid].has_copy() {
                    curr_copies - 1
                } else {
                    curr_copies
                }
            },
            Request::Read => {
                if !processors[pid].has_copy() {
                    total_cost += 1;
                }
            },
        }

        for idx in 0..NUM_OF_PROCESSORS {
            match processors[idx].state {
                // 1. While c < D, 
                // if a read is initiated by p, 
                // or if a write is initiated by p,
                // and Count is waiting, increase c by 1.
                State::Increasing => {
                    if idx == pid && (request == Request::Read || is_waiting(curr_copies, &processors)) {
                        processors[idx].count += 1;
                        if processors[idx].count == d {
                            // 2. Replicate a copy of the file to p.
                            total_cost += d;
                            curr_copies += 1;
                            max_copies = max_copies.max(curr_copies);
                            processors[idx].state = State::Decreasing;
                        }
                    }
                },
                // 3. While c > 0, 
                // if a write is initiated by any other processor, decrease c by 1.
                State::Decreasing => {
                    if idx != pid && request == Request::Write {
                        processors[idx].count -= 1;
                        if processors[idx].count == 0 {
                            processors[idx].state = State::Holding;
                        }
                    }
                },
                // 4. If p holds the last copy of the file, 
                // wait until it is replicated by some other processors[idx].
                State::Holding => {
                    if curr_copies > 1 {
                        // 5. Delete the copy held by p.
                        curr_copies -= 1;
                        processors[idx].state = State::Increasing;
                    }
                }
            }
        }
    }

    (total_cost as f64, max_copies as f64)
}