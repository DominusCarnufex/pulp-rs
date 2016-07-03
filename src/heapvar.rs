extern crate alloc;

use ::std;

#[repr(C)]
pub struct HeapVar  {
    pointer : *mut u8,
    size    : usize
}

impl HeapVar    {
    pub fn new(size : usize) -> Result<Self, String>    {
        if size == 0    {
            return Ok(HeapVar { pointer : 0 as *mut u8, size : 0 });
        }

        let heapvar = HeapVar {
            pointer : unsafe { alloc::heap::allocate(
                size,
                std::mem::align_of::<u8>()
            ) }, 
            size : size
        };

        if heapvar.pointer == 0 as *mut u8  {
            return Err(
                String::from("Allocation de mémoire pour HeapVar échouée.")
            );
        }

        heapvar.clear();
        Ok(heapvar)
    }

    pub fn from<T>(seed : T) -> Result<Self, String>    {
        let size = std::mem::size_of::<T>();

        let heapvar = match HeapVar::new(size)  {
            Ok(a)  => a,
            Err(e) => return Err(e)
        };

        unsafe { std::ptr::write(heapvar.pointer as *mut T, seed); }
        Ok(heapvar)
    }

    pub fn from_string(string : String) -> Result<Self, String> {
        let bytes = string.into_bytes();
        let size  = bytes.len() + 1;

        let heapvar = match HeapVar::new(size)  {
            Ok(a)  => a,
            Err(e) => return Err(e)
        };

        unsafe {
            std::ptr::copy(bytes.as_ptr(), heapvar.pointer, heapvar.size);
        }

        Ok(heapvar)
    }

    pub fn clear(&self) {
        unsafe { std::ptr::write_bytes(self.pointer, 0, self.size) };
    }

    pub fn pointer(&self) -> *mut u8 {
        self.pointer
    }

    pub fn vector<T>(&self) -> Result<Vec<T>, String>   {
        if self.size % std::mem::size_of::<T>() != 0  {
            return Err(String::from("HeapVar size uncompatible \
                with requested vector elements’ size."));
        }

        let size = self.size / std::mem::size_of::<T>();
        let mut vector : Vec<T> = Vec::with_capacity(size);

        unsafe {
            vector.set_len(size);
            std::ptr::copy(
                self.pointer,
                vector.as_mut_ptr() as *mut u8,
                self.size
            );
        }

        Ok(vector)
    }
} // Fin de impl.
