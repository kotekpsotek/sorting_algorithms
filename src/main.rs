/** Determine what each sort algorithm should include */
trait SortShape {
    fn sort(list: Vec<u32>) -> Vec<u32>;
}

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



fn main() {
    let arr = vec![255, 34, 12, 1, 10];
    let selection_sort = SelectionSort::sort(arr);
    println!("{selection_sort:?}")
}
