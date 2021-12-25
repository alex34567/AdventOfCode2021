struct ProgramRun {
    z_div: bool,
    x_off: i64,
    z_off: i64,
}

static PROGRAM: [ProgramRun; 14] = [
    ProgramRun {
        z_div: false,
        x_off: 11,
        z_off: 14,
    },
    ProgramRun {
        z_div: false,
        x_off: 13,
        z_off: 8,
    },
    ProgramRun {
        z_div: false,
        x_off: 11,
        z_off: 4,
    },
    ProgramRun {
        z_div: false,
        x_off: 10,
        z_off: 10,
    },
    ProgramRun {
        z_div: true,
        x_off: -3,
        z_off: 14,
    },
    ProgramRun {
        z_div: true,
        x_off: -4,
        z_off: 10,
    },
    ProgramRun {
        z_div: false,
        x_off: 12,
        z_off: 4,
    },
    ProgramRun {
        z_div: true,
        x_off: -8,
        z_off: 14,
    },
    ProgramRun {
        z_div: true,
        x_off: -3,
        z_off: 1,
    },
    ProgramRun {
        z_div: true,
        x_off: -12,
        z_off: 6,
    },
    ProgramRun {
        z_div: false,
        x_off: 14,
        z_off: 0,
    },
    ProgramRun {
        z_div: true,
        x_off: -6,
        z_off: 9,
    },
    ProgramRun {
        z_div: false,
        x_off: 11,
        z_off: 13,
    },
    ProgramRun {
        z_div: true,
        x_off: -12,
        z_off: 12,
    },
];

pub fn day24() {
    let input: [i64; 14] = [1, 1, 1, 1, 8, 1, 5, 1, 6, 3, 7, 1, 1, 2];
    let mut z = 0;
    for (i, w) in input.iter().enumerate() {
        println!("Step: {}", i);
        let mut x = z % 26;
        if PROGRAM[i].z_div {
            println!("Div step!");
            z /= 26;
        }
        x += PROGRAM[i].x_off;
        println!("Interference value {}", x);
        if x != *w {
            z *= 26;
            z += *w + PROGRAM[i].z_off;
        } else {
            println!("Interference");
        }
        println!("forced z: {}", PROGRAM[i].z_off);
        println!("z: {}", z);
        println!("next x: {}", z % 26);

        println!();
    }
}
