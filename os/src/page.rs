const PAGE_SIZE: u32 = 4096;
const PAGE_ORDER: u32 = 12;

const PAGE_TAKEN: u8 = 1 << 0;
const PAGE_LAST: u8 = 1 << 1;


extern "C" {
    static HEAP_START: u32;
}

struct Page {
    flags: u8,
}

fn _clear(page: &mut Page) {
	page.flags = 0;
}

fn _is_free(page: &mut Page) -> i32 {
	if page.flags & PAGE_TAKEN > 0{
		return 0;
	} else {
		return 1;
	}
}

fn _set_flag(page: &mut Page, flags: u8) {
	page.flags |= flags;
}

fn _is_last(page: &mut Page) -> i32 {
	if page.flags & PAGE_LAST > 0 {
		return 1;
	} else {
		return 0;
	}
}

/*
 * align the address to the border of page(4K)
 */
fn _align_page(address: u32) -> u32
{
	let order: u32 = (1 << PAGE_ORDER) - 1;
	return (address + order) & (!order);
}

fn page_init() {
    
}

fn page_alloc(pages: i32) -> Option<*const u8> {

}

fn page_free(p: Option<*const u8>) {
	
}