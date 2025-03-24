fn binarysearch(arr: &[i32], n: i32) -> Option<usize> {

    let mut hi = arr.len() - 1;
    let mut lo = 0;

    while lo <= hi {
        let m = ((hi-lo)/2) + lo;
        let v = arr[m];

        if v == n {
            return Some(m);
        } else if v > n {
            hi = m.saturating_sub(1);
        } else {
            lo = m + 1;
        }
    }

    None
}

fn main() {
    
    let arr = [3, 4, 5, 6, 7, 8, 11];
    let n = 4;

    let result = binarysearch(&arr, n);
    println!("Result: {:?}", result);

}
