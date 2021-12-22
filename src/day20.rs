static INPUT: &str = include_str!("input/Day20.txt");

pub fn day20() {
    let mut sections = INPUT.trim().split("\n\n");
    let enhance_str = sections.next().unwrap();
    let image_str = sections.next().unwrap();
    assert!(sections.next().is_none());

    let mut enhance = Vec::new();
    for c in enhance_str.chars() {
        if c == '.' {
            enhance.push(false);
        } else if c == '#' {
            enhance.push(true);
        }
    }

    let mut image = Vec::new();
    for row_str in image_str.split('\n') {
        let mut row = Vec::new();
        for c in row_str.chars() {
            if c == '.' {
                row.push(false);
            } else if c == '#' {
                row.push(true);
            }
        }
        image.push(row)
    }

    let mut void = false;
    for i in 0..50 {
        let mut new_image = vec![vec![false; image[0].len() + 4]; image.len() + 4];
        for (i, row) in new_image.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                let mut index = 0;
                for y_off in i..i + 3 {
                    for x_off in j..j + 3 {
                        index <<= 1;
                        let offset = y_off.checked_sub(2).zip(x_off.checked_sub(2));
                        index |= offset
                            .and_then(|(y_off, x_off)| {
                                image
                                    .get(y_off)
                                    .and_then(|row| row.get(x_off))
                                    .map(|n| *n as usize)
                            })
                            .unwrap_or(void as usize);
                    }
                }
                *pixel = enhance[index]
            }
        }
        if void {
            void = *enhance.last().unwrap()
        } else {
            void = *enhance.first().unwrap()
        }
        image = new_image;

        if i == 1 {
            println!(
                "Part1: {}",
                image.iter().flatten().map(|x| *x as u32).sum::<u32>()
            )
        }
    }

    println!(
        "Part2: {}",
        image.iter().flatten().map(|x| *x as u32).sum::<u32>()
    )
}
