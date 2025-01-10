use std::{
    alloc::{alloc, handle_alloc_error, realloc, Layout},
    ptr,
};

pub struct Vector<T> {
    ptr: ptr::NonNull<T>,
    capacity: usize,
    size: usize,
}

unsafe impl<T: Send> Send for Vector<T> {}
unsafe impl<T: Sync> Sync for Vector<T> {}

impl<T: Copy + Eq> Vector<T> {
    pub fn new(items: &[T]) -> Self {
        let mut vec: Vector<T> = Vector {
            ptr: ptr::NonNull::dangling(),
            capacity: 0,
            size: 0,
        };

        let len = items.len();
        let new_capacity = 2_usize.pow((len as f64).log2().ceil() as u32);
        vec.resize(new_capacity);

        for &item in items {
            vec.push(item)
        }

        vec
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn at(&self, index: usize) -> Result<T, &'static str> {
        if index >= self.size {
            return Err("Index larger than size");
        }
        let item: T = unsafe { ptr::read(self.ptr.as_ptr().add(index)) };
        Ok(item)
    }

    pub fn push(&mut self, item: T) -> () {
        if self.size == self.capacity {
            self.resize(2 * self.capacity)
        }

        unsafe { ptr::write(self.ptr.as_ptr().add(self.size), item) };
        self.size += 1;
    }

    pub fn prepend(&mut self, item: T) -> () {
        if self.size == self.capacity {
            self.resize(2 * self.capacity)
        }

        unsafe {
            ptr::copy(self.ptr.as_ptr(), self.ptr.as_ptr().add(1), self.size);
            ptr::write(self.ptr.as_ptr(), item);
        };

        self.size += 1;
    }

    pub fn insert(&mut self, index: usize, item: T) -> Result<(), &'static str> {
        if index > self.size {
            return Err("Index is larger than vector size");
        }

        if self.size == self.capacity {
            self.resize(2 * self.capacity)
        }

        unsafe {
            ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.size - index,
            );
            ptr::write(self.ptr.as_ptr().add(index), item);
        };

        self.size += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.size == 0 {
            return Err("Cannot pop from empty vector");
        }
        self.size -= 1;
        let value = unsafe { ptr::read(self.ptr.as_ptr().add(self.size)) };

        if self.capacity > 1 && self.size <= self.capacity / 2 {
            self.resize(self.capacity / 2)
        }

        Ok(value)
    }

    pub fn delete(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.size {
            return Err("Index is larger than vector size");
        }

        unsafe {
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.size - index - 1,
            );
        };

        self.size -= 1;
        if self.capacity > 1 && self.size <= self.capacity / 2 {
            self.resize(self.capacity / 2)
        }

        Ok(())
    }

    pub fn remove(&mut self, item: T) -> () {
        while let Ok(i) = self.find(item) {
            self.delete(i).unwrap()
        }
    }

    pub fn find(&self, item: T) -> Result<usize, &'static str> {
        for i in 0..self.size {
            if unsafe { ptr::read(self.ptr.as_ptr().add(i)) } == item {
                return Ok(i);
            }
        }

        Err("Could not find item")
    }

    fn resize(&mut self, capacity: usize) -> () {
        let new_layout = Layout::array::<T>(capacity).unwrap(); // NOTE: crashes it out of memory

        let new_ptr = if self.capacity == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap(); // NOTE: crashes it out of memory
            let ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { realloc(ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match ptr::NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => handle_alloc_error(new_layout), // NOTE: Gracefully allocation error
        };
        self.capacity = capacity;
    }
}
