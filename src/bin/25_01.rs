fn transform_subject(current: usize, subject: usize) -> usize {
    (current * subject) % 20201227
}

fn find_loop(n: usize) -> usize {
    let subject = 7;
    let mut current = 1;
    let mut i = 0;

    // this can technically loop forever, so we probably want a way to stop that
    // and potentially return an Option<usize> instead
    while current != n {
        current = transform_subject(current, subject);
        i += 1;
    }

    i
}

/*
9232416
14144084
*/

fn main() {
    let card_public = 9232416;
    let door_public = 14144084;

    // it's not efficient to do this twice since the calculations are the same
    // so we should optimize that if it's important.
    let card_loop = find_loop(card_public);
    let door_loop = find_loop(door_public);

    // card: 8927518 door: 13240670
    println!("card: {} door: {}", card_loop, door_loop);

    let mut card_encryption = 1;
    for i in 0..card_loop {
        card_encryption = transform_subject(card_encryption, door_public);
    }

    let mut door_encryption = 1;
    for i in 0..door_loop {
        door_encryption = transform_subject(door_encryption, card_public);
    }

    // 1478097
    println!("card_encryption: {}", card_encryption);
    println!("door_encryption: {}", door_encryption);
}
