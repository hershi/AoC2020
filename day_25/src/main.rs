type Input = (usize, usize);

const DIVIDER : usize = 20201227;
const PK_SUBJECT : usize = 7;

fn read_input() -> Input {
    (11562782, 18108497)
}

fn guess_loop_size(subject: usize, pk: usize) -> usize {
    let mut val = 1;
    let divider = 20201227;

    let mut i = 0;
    for i in 0.. {
        if val == pk { return i; }

        val *= subject;
        val %= DIVIDER;
    }

    panic!("Should never happen!");
}

fn part_1(input: &Input) {
    let l1 = guess_loop_size(PK_SUBJECT, input.0);
    let l2 = guess_loop_size(PK_SUBJECT, input.1);


    println!("Loop size 1: {}", l1);
    println!("Loop size 2: {}", l2);

    let mut val = 1;

    for _ in 0..l1 {
        val *= input.1;
        val %= DIVIDER;
    }

    println!("Part 1: {:#?}", val);
}

fn part_2(input: &Input) {
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part_1(&input);
    part_2(&input);
}
