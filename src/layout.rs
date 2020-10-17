#![allow(unused_variables, dead_code, unused_imports)]

#[derive(Copy, Clone)]
pub enum Layout<T: Eq + Copy> {
    Fixed {
        id: T,
        width: usize,
    },
    Auto {
        id: T,
        ratio: Option<usize>,
        min_width: usize,
        max_width: usize,
    },
}

impl<T: Eq + Copy> Layout<T> {
    fn get_id(&self) -> T {
        match *self {
            Layout::Fixed { id, .. } => id,
            Layout::Auto { id, .. } => id,
        }
    }

    fn is_fixed(&self) -> bool {
        return matches!(*self, Layout::Fixed { .. });
    }

    fn is_auto(&self) -> bool {
        return matches!(*self, Layout::Auto { .. });
    }

    fn width(&self) -> Option<usize> {
        match *self {
            Layout::Fixed { width, .. } => Some(width),
            _ => None,
        }
    }

    fn ratio(&self) -> Option<usize> {
        match *self {
            Layout::Auto { ratio, .. } => ratio,
            _ => None,
        }
    }
}

pub struct Size<T: Eq + Copy> {
    pub id: T,
    pub width: usize,
    pub pos: usize,
}

impl<T: Eq + Copy> Size<T> {
    pub fn new(id: T, width: usize) -> Self {
        Size { id, width, pos: 0 }
    }
}

pub fn size<T: Eq + Copy>(width: usize, layout: &Vec<Layout<T>>) -> Vec<Size<T>> {
    use std::cmp;

    // Check for unique ids
    let mut id_set = Vec::new();
    for l in layout.iter() {
        if id_set.contains(&l.get_id()) {
            panic!("Layout id's have to be unique!");
        }

        id_set.push(l.get_id());
    }

    let fixed: Vec<Layout<T>> = layout.iter().filter(|e| e.is_fixed()).copied().collect();
    let mut autos_vec: Vec<Layout<T>> = layout.iter().filter(|e| e.is_auto()).copied().collect();
    let fixed_width: usize = fixed.iter().map(|e| e.width().unwrap_or(0)).sum();

    let mut rem_width;
    let mut got_new = true;

    let mut fixed_autos: Vec<Layout<T>> = Vec::new();
    let mut size_vec: Vec<Size<T>> = Vec::new();

    println!("Start loop of doom. Size {}", width);
    while got_new && autos_vec.len() > 0 {
        let fixed_auto_width: usize = size_vec.iter().map(|e| e.width).sum();

        got_new = false;

        rem_width = width - fixed_width - fixed_auto_width;
        // let auto_share = rem_width / autos_vec.len();

        let tot_ratio: usize = autos_vec.iter().map(|e| e.ratio().unwrap_or(100)).sum();

        // let rat_val = (auto_share * 100) / (tot_ratio);

        // println!("Ratio: rem {} tot {} rat {}", rem_width, tot_ratio, rat_val);

        for l in autos_vec.iter() {
            if let Layout::Auto {
                id,
                ratio,
                min_width,
                max_width,
            } = *l
            {
                // let my_share = (auto_share * ratio.unwrap_or(100)) / 100;

                let my_rat = tot_ratio / ratio.unwrap_or(100);
                let my_share = rem_width / my_rat;

                println!("Myshare: {} rat: {}", my_share, my_rat);
                if my_share <= min_width {
                    println!("Share too small {}", my_share);
                    fixed_autos.push(*l);
                    got_new = true;
                    size_vec.push(Size::new(id, min_width));
                } else if my_share >= max_width {
                    println!("Share too big {}", my_share);
                    fixed_autos.push(*l);
                    got_new = true;
                    size_vec.push(Size::new(id, max_width));
                };
            }
        }

        autos_vec = autos_vec
            .into_iter()
            .filter(|e| !fixed_autos.iter().any(|l| l.get_id() == e.get_id()))
            .collect();
    }

    // let mut pos = 0;
    let fixed_auto_width: usize = size_vec.iter().map(|e| e.width).sum();
    let tot_ratio: usize = autos_vec.iter().map(|e| e.ratio().unwrap_or(100)).sum();
    let rem_width = if autos_vec.len() > 0 {
        width - fixed_width - fixed_auto_width
    } else {
        1
    };

    for l in layout {
        if size_vec.iter().any(|e| e.id == l.get_id()) {
            continue;
        }

        let ss = match *l {
            Layout::Fixed { id, width } => {
                println!("Add fixed");
                Size::new(id, width)
            }
            Layout::Auto { id, ratio, .. } => {
                let my_rat = tot_ratio / ratio.unwrap_or(100);
                let my_share = rem_width / my_rat;

                // println!("Myshare: {} rat: {}", my_share, my_rat);
                println!("Add auto w: {}", my_share);
                Size::new(id, my_share)
            }
        };

        size_vec.push(ss);
    }

    println!("Lay {} siz {}", layout.len(), size_vec.len());

    // Sort in layout order
    for (i, x) in layout.iter().enumerate() {
        if x.get_id() != size_vec[i].id {
            let six = size_vec
                .iter()
                .position(|e| e.id == x.get_id())
                .expect("Failed to get position if id");
            size_vec.swap(i, six);
        }
    }

    // Calc pos
    let mut pos = 0;
    for s in size_vec.iter_mut() {
        s.pos = pos;
        println!("Pos: {}", pos);
        pos += s.width;
    }

    return size_vec;
}

//#[cfg(test)]
mod tests {
    use crate::layout::size;
    use crate::layout::Layout;

    #[test]
    #[should_panic(expected = "Layout id's have to be unique!")]
    fn panic_if_id_not_unique() {
        let test_vec = vec![
            Layout::Fixed { id: 1, width: 10 },
            Layout::Auto {
                id: 1,
                ratio: None,
                min_width: 5,
                max_width: 20,
            },
        ];

        let size_vec = size(30, &test_vec);

        assert!(size_vec[0].width == 10);
        assert!(size_vec[1].width == 10);

        assert!(size_vec[0].pos == 0);
        assert!(size_vec[1].pos == 10);
    }

    #[test]
    fn simple_fixed() {
        let test_vec = vec![
            Layout::Fixed { id: 1, width: 10 },
            Layout::Fixed { id: 2, width: 10 },
        ];

        let size_vec = size(30, &test_vec);

        assert!(size_vec[0].width == 10);
        assert!(size_vec[1].width == 10);

        assert!(size_vec[0].pos == 0);
        assert!(size_vec[1].pos == 10);
    }

    #[test]
    fn simple_one_auto() {
        let test_vec = vec![
            Layout::Fixed { id: 1, width: 10 },
            Layout::Auto {
                id: 2,
                ratio: None,
                min_width: 5,
                max_width: 20,
            },
            Layout::Fixed { id: 3, width: 10 },
        ];

        let size_vec = size(30, &test_vec);

        assert!(size_vec[0].width == 10);
        assert!(size_vec[1].width == 10);
        assert!(size_vec[2].width == 10);

        assert!(size_vec[0].pos == 0);
        assert!(size_vec[1].pos == 10);
        assert!(size_vec[2].pos == 20);
    }

    #[test]
    fn simple_two_auto() {
        let test_vec = vec![
            Layout::Fixed { id: 1, width: 10 },
            Layout::Auto {
                id: 2,
                ratio: None,
                min_width: 5,
                max_width: 20,
            },
            Layout::Auto {
                id: 3,
                ratio: None,
                min_width: 5,
                max_width: 20,
            },
            Layout::Fixed { id: 4, width: 10 },
        ];

        let size_vec = size(30, &test_vec);

        assert!(size_vec[0].width == 10);
        assert!(size_vec[1].width == 5);
        assert!(size_vec[2].width == 5);
        assert!(size_vec[3].width == 10);

        assert!(size_vec[0].pos == 0);
        assert!(size_vec[1].pos == 10);
        assert!(size_vec[2].pos == 15);
        assert!(size_vec[3].pos == 20);
    }

    #[test]
    fn complex_three_auto() {
        let test_vec = vec![
            Layout::Fixed { id: 1, width: 10 },
            Layout::Auto {
                id: 2,
                ratio: None,
                min_width: 5,
                max_width: 5,
            },
            Layout::Auto {
                id: 3,
                ratio: None,
                min_width: 5,
                max_width: 20,
            },
            Layout::Auto {
                id: 5,
                ratio: None,
                min_width: 5,
                max_width: 20,
            },
            Layout::Fixed { id: 6, width: 10 },
        ];

        let size_vec = size(45, &test_vec);

        assert!(size_vec[0].width == 10);
        assert!(size_vec[1].width == 5);
        assert!(size_vec[2].width == 10);
        assert!(size_vec[3].width == 10);
        assert!(size_vec[4].width == 10);

        assert!(size_vec[0].pos == 0);
        assert!(size_vec[1].pos == 10);
        assert!(size_vec[2].pos == 15);
        assert!(size_vec[3].pos == 25);
        assert!(size_vec[4].pos == 35);
    }

    #[test]
    fn complex_three_auto_too_wide() {
        let test_vec = vec![
            Layout::Auto {
                id: 1,
                ratio: None,
                min_width: 5,
                max_width: 10,
            },
            Layout::Auto {
                id: 2,
                ratio: None,
                min_width: 5,
                max_width: 10,
            },
            Layout::Auto {
                id: 3,
                ratio: None,
                min_width: 5,
                max_width: 10,
            },
        ];

        let size_vec = size(40, &test_vec);

        assert!(size_vec[0].width == 10);
        assert!(size_vec[1].width == 10);
        assert!(size_vec[2].width == 10);
    }

    #[test]
    fn complex_three_auto_too_small() {
        let test_vec = vec![
            Layout::Auto {
                id: 1,
                ratio: None,
                min_width: 10,
                max_width: 10,
            },
            Layout::Auto {
                id: 2,
                ratio: None,
                min_width: 10,
                max_width: 10,
            },
            Layout::Auto {
                id: 3,
                ratio: None,
                min_width: 10,
                max_width: 10,
            },
        ];

        let size_vec = size(20, &test_vec);

        assert!(size_vec[0].width == 10);
        assert!(size_vec[1].width == 10);
        assert!(size_vec[2].width == 10);
    }

    #[test]
    fn ratio_two_simple_default() {
        let test_vec = vec![
            Layout::Auto {
                id: 1,
                ratio: None,
                min_width: 10,
                max_width: 100,
            },
            Layout::Auto {
                id: 2,
                ratio: None,
                min_width: 10,
                max_width: 100,
            },
        ];

        let size_vec = size(100, &test_vec);

        assert!(size_vec[0].width == 50);
        assert!(size_vec[1].width == 50);
    }

    #[test]
    fn ratio_two_simple() {
        let test_vec = vec![
            Layout::Auto {
                id: 1,
                ratio: Some(30),
                min_width: 10,
                max_width: 100,
            },
            Layout::Auto {
                id: 2,
                ratio: Some(30),
                min_width: 10,
                max_width: 100,
            },
        ];

        let size_vec = size(100, &test_vec);

        assert!(size_vec[0].width == 50);
        assert!(size_vec[1].width == 50);
    }

    #[test]
    fn ratio_three_simple() {
        let test_vec = vec![
            Layout::Auto {
                id: 1,
                ratio: Some(30),
                min_width: 10,
                max_width: 100,
            },
            Layout::Auto {
                id: 2,
                ratio: Some(30),
                min_width: 10,
                max_width: 100,
            },
            Layout::Auto {
                id: 3,
                ratio: Some(30),
                min_width: 10,
                max_width: 100,
            },
        ];

        let size_vec = size(90, &test_vec);

        assert!(size_vec[0].width == 30);
        assert!(size_vec[1].width == 30);
        assert!(size_vec[2].width == 30);
    }

    #[test]
    fn ratio_three_one_big_two_small() {
        let test_vec = vec![
            Layout::Auto {
                id: 1,
                ratio: None,
                min_width: 10,
                max_width: 100,
            },
            Layout::Auto {
                id: 2,
                ratio: Some(50),
                min_width: 10,
                max_width: 100,
            },
            Layout::Auto {
                id: 3,
                ratio: Some(50),
                min_width: 10,
                max_width: 100,
            },
        ];

        let size_vec = size(100, &test_vec);

        assert!(size_vec[0].width == 50);
        assert!(size_vec[1].width == 25);
        assert!(size_vec[2].width == 25);
    }

    #[test]
    fn ratio_three_one_big_two_small_tiny_values() {
        let test_vec = vec![
            Layout::Auto {
                id: 1,
                ratio: None,
                min_width: 1,
                max_width: 100,
            },
            Layout::Auto {
                id: 2,
                ratio: Some(50),
                min_width: 1,
                max_width: 100,
            },
            Layout::Auto {
                id: 3,
                ratio: Some(50),
                min_width: 1,
                max_width: 100,
            },
        ];

        let size_vec = size(10, &test_vec);

        assert!(size_vec[0].width == 5);
        assert!(size_vec[1].width == 2);
        assert!(size_vec[2].width == 2);
    }

    #[test]
    fn ratio_many_many() {
        let test_vec = vec![
            Layout::Fixed { id: 0, width: 10 },
            Layout::Auto {
                id: 1,
                ratio: None,
                min_width: 1,
                max_width: 100,
            },
            Layout::Fixed { id: 7, width: 10 },
            Layout::Auto {
                id: 2,
                ratio: None,
                min_width: 1,
                max_width: 100,
            },
            Layout::Auto {
                id: 3,
                ratio: None,
                min_width: 1,
                max_width: 100,
            },
            Layout::Auto {
                id: 4,
                ratio: None,
                min_width: 1,
                max_width: 100,
            },
            Layout::Auto {
                id: 5,
                ratio: None,
                min_width: 1,
                max_width: 100,
            },
            Layout::Auto {
                id: 6,
                ratio: None,
                min_width: 1,
                max_width: 100,
            },
        ];

        let size_vec = size(80, &test_vec);

        for s in size_vec.iter() {
            assert!(s.width == 10);
        }
    }
}
