
pub struct ShellSequence {
    i: usize,
    len: usize
}

impl ShellSequence {
    pub fn new(len: usize) -> Self {
        return Self {i: 0, len: len};
    }

    pub fn next(&mut self) -> Option<usize> {
        self.i += 1;
        let result =  self.len / 2_usize.pow(self.i as u32);
        if result < 1 {
            return None;
        }
        return Some(result);
    }
}

pub struct ShellSort;
impl ShellSort {
    pub fn sort(array: &Vec<isize>) -> Vec<isize> {
        let mut result = array.clone();
        let mut sequence = ShellSequence::new(result.len());

        loop {
            match sequence.next() {
                Some(gap) => {
                    for i in gap..result.len() {
                        let aux = result[i];
                        let mut j = i;
                        while j >= gap && result[j - gap] > aux {
                            result.swap(j - gap, j);
                            j -= gap;
                        }
                        result[j] = aux;
                    }
                },
                None => break
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn shell_next() {
        let mut shell = ShellSequence::new(6);
        assert_eq!(shell.next(), Some(3));
        assert_eq!(shell.next(), Some(1));
        assert_eq!(shell.next(), None);
    }

    #[test]
    fn shell_sort() {
        let array_unordered = [55, 8, 15, 97, 66, 42].to_vec();
        let array_ordered = [8, 15, 42, 55, 66, 97].to_vec();

        let result = ShellSort::sort(&array_unordered);
        
        assert_eq!(result, array_ordered);
    }


}