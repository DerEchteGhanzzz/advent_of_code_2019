pub fn solve_a(input: Vec<String>) -> u32 {
    let images = to_images(input[0].chars().collect(), 25, 6);
    let mut min_zeroes = std::u32::MAX;
    let mut ans = 0;
    for image in images {
        let count = image.concat().iter().fold((0, 0, 0), |acc@(zs, os, ts), ch| { 
            match ch {
                '0' => (zs+1, os, ts),
                '1' => (zs, os+1, ts),
                '2' => (zs, os, ts+1),
                _ => acc,
            }
        });
        if count.0 < min_zeroes {
            min_zeroes = count.0;
            ans = count.1 * count.2;
        }
    }
    ans
}

pub fn solve_b(input: Vec<String>) -> String {
    let mut images = to_images(input[0].chars().map(|ch| {
        match ch {
            '0' => '.',
            '1' => '#',
            _ => '=',
        }
    }).collect(), 25, 6);
    images.iter_mut().fold(&Vec::new(), |acc, img| {
        fold_images(acc, img)
    }).iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n")
}

fn fold_images<'a>(acc: &'a Image, img: &'a mut Image) -> &'a Image {
    if acc.is_empty() {
        return img;
    }
    
    for i in 0..acc[0].len() {
        for j in 0..acc.len() {
            match acc[j][i] {
                '.' => img[j][i] = '.',
                '#' => img[j][i] = '#',
                _ => (),
            }
        }
    }
    img
}

type Image = Vec<Vec<char>>;

fn to_images(input: Vec<char>, wt: usize, ht: usize) -> Vec<Image> {
    let mut images: Vec<Vec<char>> = Vec::new();
    let mut image: Vec<char> = Vec::new();
    let mut count = 0;
    for ch in input {
        image.push(ch);
        count += 1;
        if count % (wt*ht) == 0 {
            images.push(image);
            image = Vec::new();
        }
    }
    images.iter().map(|i| to_image(i, wt, ht)).collect()
}

fn to_image(input: &Vec<char>, wt: usize, ht: usize) -> Image {
    let mut image: Image = Vec::new();
    input.iter().fold((0, 0), |acc@(cur_ht, cur_wt): (usize, usize), ch| {
        if cur_ht == ht {
            return acc;
        }
        if cur_wt == 0 {
            image.push(Vec::new());
        }
        let mut last = image.pop().unwrap();
        last.push(*ch);
        image.push(last);
        (if cur_wt + 1 == wt { cur_ht + 1 } else { cur_ht } , (cur_wt+1) % wt)
    });
    image
}