pub mod sort {

    pub fn bubble_sort(arr: &mut [i32]) -> Vec<i32> {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..arr.len() {
                if arr[i - 1] > arr[i] {
                    arr.swap(i - 1, i);
                    swapped = true
                }
            }
        }
        return Vec::from(arr);
    }

    pub fn test_bubble_sort() {
        let test_arr = &mut [3, 4, 5, 31, 2, 3, 1];
        let test_result = bubble_sort(test_arr);
        println!("The bubble sort result is {:?}", test_result);
    }
}
