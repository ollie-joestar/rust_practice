#[allow(dead_code)]
fn box_cmp(box_a: [u32; 2], box_b: [u32; 2]) -> bool {
    box_a[0] >= box_b[0] && box_a[1] >= box_b[1]
}

#[allow(dead_code)]
fn  is_sorted(boxes: &mut [[u32; 2]]) -> bool {
    let mut i = 0;
    while i < boxes.len() - 1 {
        if !box_cmp(boxes[i], boxes[i + 1]) {
            return false;
        }
        i += 1;
    }
    true
}

#[allow(dead_code)]
fn sort_boxes(boxes: &mut [[u32; 2]]) {
    if boxes.is_empty() {
        return;
    }
    let  (mut i, mut j) = (0, 0);
    while i < boxes.len() {
        while i + j < boxes.len() {
            if !box_cmp(boxes[i], boxes[i + j]) {
                boxes.swap(i, i + j);
            }
           j += 1; 
        }
        j = 0;
        i += 1;
    }
    if  !is_sorted(boxes) {
        panic!("Cant fit the unfittable");
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn  test_sort_small() {
        let mut boxes = [[4, 3], [5, 7]];
        crate::sort_boxes(&mut boxes);
        assert_eq!(boxes, [[5, 7], [4, 3]]);
    }
    #[test]
    fn  test_sort_normal() {
        let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
        crate::sort_boxes(&mut boxes);
        assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
    }
    #[test]
    fn  test_sort_imposter() {
        let mut boxes = [[0, 0], [0, 0], [0, 0], [0, 0], [1, 0]];
        crate::sort_boxes(&mut boxes);
        assert_eq!(boxes, [[1, 0], [0, 0], [0, 0], [0, 0], [0, 0]]);
    }
    #[test]
    #[should_panic]
    fn  test_sort_panic() {
        let mut boxes = [[0, 0], [0, 0], [0, 0], [0, 1], [1, 0]];
        crate::sort_boxes(&mut boxes);
    }
    #[test]
    fn  test_sort_empty() {
        let mut boxes = [];
        crate::sort_boxes(&mut boxes);
        assert_eq!(boxes.is_empty(), true);
    }
    #[test]
    fn  test_sort_sorted() {
        let mut boxes = [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]];
        crate::sort_boxes(&mut boxes);
        assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
    }
}
