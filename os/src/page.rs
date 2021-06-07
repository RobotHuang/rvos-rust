const PAGE_SIZE: u32 = 4096;
const PAGE_ORDER: u32 = 12;

const PAGE_TAKEN: u8 = 1 << 0;
const PAGE_LAST: u8 = 1 << 1;

static mut _alloc_start: u32 = 0;
static mut _alloc_end: u32 = 0;
static mut _num_pages: u32 = 0;

extern "C" {
    static MEMORY_START: u32;
    static MEMORY_END: u32;
    static HEAP_START: u32;
    static HEAP_SIZE: u32;
    static TEXT_START: u32;
    static TEXT_END: u32;
    static DATA_START: u32;
    static DATA_END: u32;
    static RODATA_START: u32;
    static RODATA_END: u32;
    static BSS_START: u32;
    static BSS_END: u32;
    fn printf(s: *const u8, ...);
}

#[derive(Copy, Clone)]
struct Page {
    flags: u8,
}

fn _clear(page: *mut Page) {
    unsafe {
        page.write_volatile(Page { flags: 0 });
    }
}

fn _is_free(page: *mut Page) -> bool {
    unsafe {
        let page_struct = page.read_volatile();
        if page_struct.flags & PAGE_TAKEN > 0 {
            return false;
        } else {
            return true;
        }
    }
}

fn _set_flag(page: *mut Page, flags: u8) {
    unsafe {
        let page_struct = page.read_volatile();
        let write_flags = page_struct.flags | flags;
        page.write_volatile(Page { flags: write_flags });
    }
}

fn _is_last(page: *mut Page) -> bool {
    unsafe {
        let page_struct = page.read_volatile();
        if page_struct.flags & PAGE_LAST > 0 {
            return true;
        } else {
            return false;
        }
    }
}

/*
 * align the address to the border of page(4K)
 */
fn _align_page(address: u32) -> u32 {
    let order: u32 = (1 << PAGE_ORDER) - 1;
    return (address + order) & (!order);
}

pub fn page_init() {
    unsafe {
        printf(
            b"MEMORY:   0x%x -> 0x%x\n\0" as *const u8,
            MEMORY_START,
            MEMORY_END,
        );
        _num_pages = (HEAP_SIZE / PAGE_SIZE) - 8;
        printf(
            b"HEAP_START = %x, HEAP_SIZE = %x, num of pages = %d\n\0" as *const u8,
            HEAP_START,
            HEAP_SIZE,
            _num_pages,
        );
        let page = HEAP_START as *mut Page;
        for i in 0.._num_pages {
            _clear(page.offset(i as isize));
        }
        _alloc_start = _align_page(HEAP_START + 8 * PAGE_SIZE);
        _alloc_end = _alloc_start + (_num_pages * PAGE_SIZE);

        printf(
            b"TEXT:   0x%x -> 0x%x\n\0" as *const u8,
            TEXT_START,
            TEXT_END,
        );
        printf(
            b"RODATA: 0x%x -> 0x%x\n\0" as *const u8,
            RODATA_START,
            RODATA_END,
        );
        printf(
            b"DATA:   0x%x -> 0x%x\n\0" as *const u8,
            DATA_START,
            DATA_END,
        );
        printf(b"BSS:    0x%x -> 0x%x\n\0" as *const u8, BSS_START, BSS_END);
        printf(
            b"HEAP:   0x%x -> 0x%x\n\0" as *const u8,
            _alloc_start,
            _alloc_end,
        );
    };
}

fn page_alloc(npages: i32) -> Option<*const u8>{
    unsafe {
        let mut found = false;
        let mut page_i = HEAP_START as *mut Page;
        for i in 0.._num_pages as i32 - npages {
            if _is_free(page_i) {
                found = true;
                let mut page_j = page_i;
                for j in i..i+npages {
                    if !_is_free(page_j) {
                        found = false;
                        break;
                    }
                    page_j = page_j.offset((j- i + 1) as isize);
                }

                if found {
                    let mut page_k = page_i;
                    for k in i..i + npages {
                        _set_flag(page_k, PAGE_TAKEN);
                        page_k = page_k.offset((k-i+1) as isize);
                    }
                    page_k = page_k.offset(-1 as isize);
                    _set_flag(page_k, PAGE_LAST);
                    return Some((_alloc_start + i as u32 * PAGE_SIZE) as *const u8)
                }
            }
            page_i = page_i.offset((i+1) as isize);
        }
    }
    None
}

// fn page_free(p: Option<*const u8>) {}

pub fn page_test() {
    let p = page_alloc(2);
	unsafe {
        if let Some(v) = p {
            printf(b"p = 0x%x\n\0" as *const u8, v);
        }
    }

    let p2 = page_alloc(7);
    unsafe {
        if let Some(v) = p2 {
            printf(b"p2 = 0x%x\n\0" as *const u8, v);
        }
    }
}