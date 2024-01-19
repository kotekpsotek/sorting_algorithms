/** Determine what each sort algorithm should include */
trait SortShape {
    fn sort(list: Vec<u32>) -> Vec<u32>;
}

/** Selection sort is a simple and efficient sorting algorithm that works by repeatedly selecting the smallest (or largest) element from the unsorted portion of the list and moving it to the sorted portion of the list. */
struct SelectionSort;
impl SortShape for SelectionSort {
    fn sort(mut list: Vec<u32>) -> Vec<u32> {
        for k in 0..list.len() {
            
            // Find the smallest (untouched value)
            let mut m_i = k;
            for i in k+1..list.len() {
                if list[m_i] > list[i] {
                    m_i = i;
                }
            }

            let vk = list[k];
            list[k] = list[m_i];
            list[m_i] = vk;
        }

        list
    }
}

/** Bubble Sort is the simplest sorting algorithm that works by repeatedly swapping the adjacent elements if they are in the wrong order. This algorithm is not suitable for large data sets as its average and worst-case time complexity is quite high.  */
struct BubbleSort;
impl SortShape for BubbleSort {
    fn sort(mut list: Vec<u32>) -> Vec<u32> {
        for i in 0..list.len() {
            let mut new_position = i;
            
            for j in new_position+1..list.len() {
                if list[i] > list[j] {
                    new_position = j;

                    let jv = list[j];
                    list[j] =  list[i];
                    list[i] = jv;
                }
            }

            // Check list is already ordered then break (Algorithm can live without but will be less performant)
            if new_position == i {
                break;
            }
        }
        
        list
    }
}

struct InsertionSort;
impl SortShape for InsertionSort {
    fn sort(mut list: Vec<u32>) -> Vec<u32> {
        for i in 1..list.len() {
            // Move one greater key one position ahead
            let mut j = i;
            while j > 0 && list[j] < list[j - 1] {
                list.swap(j, j - 1);
                j -= 1;
            }
        }

        list
    }
}

fn main() {
    let arr = vec![255, 34, 12, 1, 10];
    let selection_sort = SelectionSort::sort(arr);
    println!("Selection Sort: {selection_sort:?}");
}

#[cfg(test)]
mod test {
    use crate::{BubbleSort, SortShape, InsertionSort};


    #[test]
    fn bubble_sort() {
        let arr = vec![255, 34, 12, 1, 10];
        let bubble_sort = BubbleSort::sort(arr);
        println!("{bubble_sort:?}")
    }

    #[test]
    fn insertion_sort() {
        let arr = vec![12, 11, 13, 5, 6];
        let insertion_sort = InsertionSort::sort(arr);
        println!("Insertion Sort {insertion_sort:?}")
    }
}
