struct SnapshotArray {
    snap_id: i32,
    items: Vec<Vec<(usize, i32)>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        let length = length as usize;
        let mut items = Vec::with_capacity(length);
        items.resize(length, vec![(0, 0)]);
        SnapshotArray {
            snap_id: 0,
            items,
        }
    }
    
    fn set(&mut self, index: i32, val: i32) {
        let index = index as usize;
        self.items[index] = val;
        self.items[index].push(0);
    }
    
    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        self.snap_id - 1
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let index = index as usize;
        let n = self.items[index].len();
        if n < snap_id {
            self.items[n - 1][index]
        } else {
            self.items[snap_id][index]
        }
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */

#[cfg(test)]
mod tests {
    use super::SnapshotArray;

    #[test]
    fn test() {
        let obj = SnapshotArray::new(6);
        obj.set(0, 1);
        let id1: i32 = obj.snap();
        obj.set(0, 2);
        obj.set(1, 4);
        let id2: i32 = obj.snap();
        assert_eq!(obj.get(0, id1), 1);
        assert_eq!(obj.get(0, id2), 2);
        assert_eq!(obj.get(1, id2), 4);
    }

}
