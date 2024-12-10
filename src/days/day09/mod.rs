// https://adventofcode.com/2024/day/9

pub fn solution_part1(input: &str) -> usize {
    let disk_map = DiskMap::from_input(input);
    disk_map.checksum_after_compacting_individual_blocks()
}

pub fn solution_part2(input: &str) -> usize {
    let disk_map = DiskMap::from_input(input);
    disk_map.checksum_after_compacting_files()
}

struct DiskMap {
    structure: Vec<Option<usize>>,
}

impl DiskMap {
    fn from_input(input: &str) -> Self {
        let mut is_in_files = true;
        let mut id = 0;
        let mut structure = vec![];

        for n in input.chars().filter_map(|c| c.to_digit(10)) {
            if is_in_files {
                for _ in 0..n {
                    structure.push(Some(id));
                }

                id += 1;
            } else {
                for _ in 0..n {
                    structure.push(None);
                }
            }

            is_in_files = !is_in_files;
        }

        Self { structure }
    }

    fn checksum_after_compacting_individual_blocks(&self) -> usize {
        let mut i_from_end = self.structure.len();

        self.structure
            .iter()
            .enumerate()
            .map_while(|(i, f)| match f {
                Some(id) => {
                    if i == i_from_end {
                        None
                    } else {
                        Some(id * i)
                    }
                }
                None => loop {
                    if i == i_from_end {
                        return None;
                    }

                    if let Some(id) = self.structure[i_from_end - 1] {
                        i_from_end -= 1;
                        return Some(id * i);
                    }

                    i_from_end -= 1;
                },
            })
            .sum()
    }

    fn checksum_after_compacting_files(&self) -> usize {
        let (mut files, mut free_spaces) = group_in_files_and_free_spaces(&self.structure);

        for file in files.iter_mut().rev() {
            for free_space in free_spaces.iter_mut() {
                if file.space.from < free_space.from {
                    break;
                }

                if free_space.fits(&file) {
                    file.move_into(free_space);
                    break;
                }
            }
        }

        files
            .into_iter()
            .map(|file| {
                (file.space.from..file.space.to)
                    .map(|i| i * file.id)
                    .sum::<usize>()
            })
            .sum()
    }
}

struct Space {
    from: usize,
    to: usize,
}

struct File {
    id: usize,
    space: Space,
}

impl Space {
    fn fits(&self, file: &File) -> bool {
        self.to - self.from >= file.space.to - file.space.from
    }
}

impl File {
    fn move_into(&mut self, space: &mut Space) {
        let file_length = self.space.to - self.space.from;
        self.space.from = space.from;
        self.space.to = space.from + file_length;
        space.from += file_length;
    }
}

fn group_in_files_and_free_spaces(structure: &[Option<usize>]) -> (Vec<File>, Vec<Space>) {
    let mut files: Vec<File> = vec![];
    let mut free_spaces: Vec<Space> = vec![];

    let mut start = 0;
    let mut current_file_id = None;
    for (i, block) in structure.iter().enumerate() {
        match (current_file_id, block) {
            (None, Some(block_file_id)) => {
                if start != i {
                    free_spaces.push(Space { from: start, to: i })
                }
                start = i;
                current_file_id = Some(block_file_id);
            }
            (Some(id), Some(block_file_id)) if id != block_file_id => {
                files.push(File {
                    id: *id,
                    space: Space { from: start, to: i },
                });
                start = i;
                current_file_id = Some(block_file_id);
            }
            (Some(_), Some(_)) => {}
            (Some(id), None) => {
                files.push(File {
                    id: *id,
                    space: Space { from: start, to: i },
                });
                start = i;
                current_file_id = None;
            }
            (None, None) => {}
        }
    }

    if let Some(id) = current_file_id {
        files.push(File {
            id: *id,
            space: Space {
                from: start,
                to: structure.len(),
            },
        });
    }

    (files, free_spaces)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 1928);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_part1(INPUT), 6378826667552);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 2858);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution_part2(INPUT), 6413328569890);
    }
}
