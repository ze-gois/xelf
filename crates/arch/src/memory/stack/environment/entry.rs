#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Entry<'a> {
    pub index: usize,
    pub pointer: *mut u8,
    pub key: &'a str,
    pub value: &'a str,
    
    // Optionally store the full environment string
    pub full_string: &'a str,
}

impl<'a> Entry<'a> {
    pub fn from_pointer(p: *mut *mut u8, i: usize) -> Self {
        let mut key = "";
        let mut value = "";
        let full_string;
        
        unsafe {
            // Get the environment string pointer
            let env_ptr = *p;
            
            // Get the length of the string
            let l = crate::memory::misc::length(env_ptr);
            
            // Create a slice from the raw pointer
            let s = core::slice::from_raw_parts(env_ptr, l);
            
            // Convert to UTF-8 string
            full_string = core::str::from_utf8(s).unwrap_or("");
            
            // Split at '=' character to get key and value
            if let Some(pos) = full_string.find('=') {
                key = &full_string[..pos];
                value = &full_string[pos+1..];
            } else {
                // If there's no '=' character, use the whole string as key
                key = full_string;
            }
        }
        
        Entry {
            index: i,
            pointer: unsafe { *p },
            key,
            value,
            full_string,
        }
    }
    
    pub fn name(&self) -> &str {
        self.key
    }
    
    pub fn value(&self) -> &str {
        self.value
    }
}
