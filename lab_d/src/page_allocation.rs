use fastrand::Rng;

const NUM_OF_PROCESSORS : usize = 64;
const SEQ_LENGTH        : usize = 65536;

#[derive(Clone, Copy, PartialEq)]
enum ProcessorState {
    Increase,
    Decrease,
    Hold
}

#[derive(Clone, Copy)]
struct Processor {
    count : u128,
    state : ProcessorState
}

impl Processor {
    pub fn new_empty() -> Processor {
        Processor {
            count : 0,
            state : ProcessorState::Increase
        }
    }

    pub fn new_full() -> Processor {
        Processor {
            count : 0,
            state : ProcessorState::Hold
        }
    }
}

enum Request {
    Read,
    Write
}

fn init_processors() -> [Processor; NUM_OF_PROCESSORS] {
    let mut processors = 
        [Processor::new_empty(); NUM_OF_PROCESSORS];
    processors[0] = Processor::new_full();
    processors
}

fn rand_processor(rand: &Rng) -> usize {
    rand.usize(0..NUM_OF_PROCESSORS)
}

fn rand_request(rand: &Rng, p: f64) -> Request {
    if rand.f64() < p {
        Request::Write
    } else {
        Request::Read
    }
}

pub fn page_allocation(d: u128, p: f64) -> (f64, f64) {
    let mut total_cost = 0;
    let mut curr_copies = 1;
    let mut max_copies = 1; 
    let rand = Rng::new();
    let mut processors = init_processors();

    // 1. While c < D, if a read is initiated by p, or if a write is initiated by p, and
    // Count is waiting, increase e by 1.
    // 2. Replicate a copy of the file to p.
    // 3. While c > 0, if a write is initiated by any other processor, decrease c by 1.
    // 4. If p holds the last copy of the file, wait until it is replicated by some other
    // processor.
    // 5. Delete the copy held by p.
    // 6. Repeat from step 1.

    for _ in 0..SEQ_LENGTH {
        let pid = rand_processor(&rand);
        let request = rand_request(&rand, p);
        let state = processors[pid].state;

        match (request, state) {
            (Request::Read, ProcessorState::Increase) => {
                // reading cost
                total_cost += 1;
                processors[pid].count += 1;

                if processors[pid].count == d {
                    // copying cost
                    total_cost += d;
                    
                    curr_copies += 1;
                    max_copies = max_copies.max(curr_copies);
                    processors[pid].state = ProcessorState::Decrease;
                }
            },
            (Request::Write, ProcessorState::Increase) => {
                // writing cost
                total_cost += curr_copies;

                if curr_copies == 1 {
                    processors[pid].count += 1;
                }
                
                if processors[pid].count == d {
                    // copying cost
                    total_cost += d;
                    curr_copies += 1;
                    max_copies = max_copies.max(curr_copies);
                    processors[pid].state = ProcessorState::Decrease;
                }
            },
            (Request::Write, ProcessorState::Decrease) => {
                // writing cost
                total_cost += curr_copies - 1;
                
                processors[pid].count -= 1;

                if processors[pid].count == 0 {
                    processors[pid].state = ProcessorState::Hold;
                }
            },
            (Request::Write, ProcessorState::Hold) => {
                // writing cost
                total_cost += curr_copies - 1;
            },
            _ => ()
        }

        if curr_copies > 1 {
            for processor in processors.iter_mut() {
                if processor.state == ProcessorState::Hold {
                    curr_copies -= 1;
                    processor.state = ProcessorState::Increase;
                }
            }
        }
    }

    (total_cost as f64, max_copies as f64)
}