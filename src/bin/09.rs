use std::collections::VecDeque;

advent_of_code::solution!(9);

#[derive(Debug)]
struct DiskBlock {
    id: usize,
}

#[derive(Debug)]
struct DiskFile {
    id: usize,
    starting_index: usize,
    used: usize,
    free: usize,
    moved: bool,
}

impl DiskFile {
    fn get_total(&self) -> usize {
        self.used + self.free
    }

    fn can_fit(&self, other: &Self) -> bool {
        self.free >= other.used
    }

    fn shrink(&mut self, other: &mut Self) {
        other.free = self.free - other.used;
        other.starting_index = self.starting_index + self.used;
        self.free = 0;
    }

    fn add_free_space(&mut self, free: usize) {
        self.free += free;
    }

    fn get_hash(&self) -> usize {
        let mut acc = 0;
        for i in 0..self.used {
            acc += self.id * (self.starting_index + i);
        }
        acc
    }
}

#[derive(Debug)]
struct Disk {
    blocks: Vec<Option<DiskBlock>>,
    files: VecDeque<DiskFile>,
}

impl Disk {
    fn from_input(input: &str) -> Self {
        let mut files = VecDeque::new();
        let mut file_index = 0;
        let mut block_index = 0;
        // I want to break a string into chars, map those to u8 and then iterate over them, while there is a next, then I want to parse the next two into a DiskFile
        let mut chars = input
            .trim()
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as usize);

        while let Some(used) = chars.next() {
            let free = chars.next();
            let free = match free {
                Some(f) => f,
                None => 0,
            };
            files.push_back(DiskFile {
                id: file_index,
                starting_index: block_index,
                used,
                free,
                moved: false,
            });
            file_index += 1;
            block_index += used + free;
        }

        let total_blocks = files.iter().map(|f| f.get_total()).sum();
        let mut blocks = Vec::with_capacity(total_blocks);
        blocks.resize_with(total_blocks, || None);

        let mut index = 0;
        for (file_index, file) in files.iter().enumerate() {
            for _ in 0..file.used {
                blocks.insert(index, Some(DiskBlock { id: file_index }));
                index += 1;
            }
            index += file.free;
        }

        Disk { blocks, files }
    }

    fn defragment_blocks(&mut self) {
        // This should go through the blocks, and if it finds a used block, move to the next
        // if if finds a free block, it should get a block from the end and move it to the free block
        // then it should move to the next block
        let mut index = 0;

        // Pop all of the blocks from the end
        // then find the next unused block and insert the popped block
        while index < self.blocks.len() {
            let last_block = self.blocks.pop();
            match last_block {
                Some(Some(block)) => {
                    while self.blocks[index].is_some() {
                        index += 1;
                        if index >= self.blocks.len() {
                            break;
                        }
                    }
                    if index >= self.blocks.len() {
                        self.blocks.push(Some(block));
                        break;
                    } else {
                        self.blocks[index] = Some(block);
                        index += 1;
                    }
                }
                _ => continue,
            }
        }
    }

    fn defragment_files(&mut self) {
        let file_list = &mut self.files;

        loop {
            // Find the index of the unmoved file with the highest id
            let highest_id = file_list
                .iter()
                .filter(|f| !f.moved)
                .map(|file| file.id)
                .max();
            if highest_id.is_none() {
                break;
            }
            let highest_id = highest_id.unwrap();
            let back_index = file_list.iter().position(|f| f.id == highest_id).unwrap();

            if back_index == 0 {
                break;
            }
            // Set the current file to moved
            file_list[back_index].moved = true;
            let mut index = 0;
            // Find the first available space that can fit the file
            // It should be to the left of the file
            while index < back_index {
                if file_list[index].can_fit(&file_list[back_index]) {
                    let mut moved_file = file_list.remove(back_index).unwrap();
                    // Add free space to the previous file
                    file_list[back_index - 1].add_free_space(moved_file.get_total());

                    file_list[index].shrink(&mut moved_file);
                    moved_file.moved = true;
                    file_list.insert(index, moved_file);
                    break;
                }
                index += 1;
            }
        }
    }
    fn calc_checksum_blocks(&self) -> usize {
        // The checksum is calculated by multiplying the index of the block with id of the block
        self.blocks
            .iter()
            .enumerate()
            .map(|(index, b)| match b {
                Some(block) => block.id * index,
                None => 0,
            })
            .sum()
    }

    fn calc_checksum_files(&self) -> usize {
        // The checksum is calculated by multiplying the index of the file with id of the file
        self.files.iter().map(|f| f.get_hash()).sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk = Disk::from_input(input);
    disk.defragment_blocks();
    Some(disk.calc_checksum_blocks())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk = Disk::from_input(input);
    disk.defragment_files();
    Some(disk.calc_checksum_files())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
