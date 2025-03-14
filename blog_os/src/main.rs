#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
use bootloader::{BootInfo, entry_point};
use blog_os::memory::translate_addr;
use x86_64::{
    VirtAddr,
    structures::paging::PageTable,
};

entry_point!(kernel_main);

// This func is called onPanic by compiler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

// static HELLO: &[u8] = b"Red rust is a mood!";

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    
    println!("Red Rust is {}!!", "eternal");
    blog_os::init();
    
    fn stack_overflow() {
        stack_overflow();
    }
    // stack_overflow();

    // page fault
    //unsafe {
    //    *(0xdeadbeef as *mut u8) = 42;
    //};
    //let ptr = 0x204396 as *mut u8;

    // read from a code page
    //unsafe { let x = *ptr; }
    //println!("read worked");

    // write to a code page
    //unsafe { *ptr = 42; }
    //println!("write worked");
    
    // Breakpoint here
    //x86_64::instructions::interrupts::int3();
    
    use blog_os::memory;
    use x86_64::{structures::paging::Translate, structures::paging::Page, registers::control::Cr3, VirtAddr};

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();
    use blog_os::memory::BootInfoFrameAllocator;
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    }; 
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    //let page22 = Page::<x86_64::structures::paging::Size4KiB>::containing_address(VirtAddr::new(0xdeadbeaf000));
//    let l4_table = unsafe { active_level_4_table(phys_mem_offset)};
//
//    for (i, entry) in l4_table.iter().enumerate() {
//        if !entry.is_unused() {
//            println!("L4 Entry {}: {:?}", i, entry);
//
//            let phys = entry.frame().unwrap().start_address();
//            let virt = phys.as_u64() + boot_info.physical_memory_offset;
//            let ptr = VirtAddr::new(virt).as_mut_ptr();
//            let l3_table: &PageTable = unsafe {&*ptr};
//
//            for (i, entry) in l3_table.iter().enumerate() {
//                if !entry.is_unused() {
//                    println!("  L3 Entry {}: {:?}", i, entry);
//                }
//            }
//        }
//    }
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

//    for &address in &addresses {
//        let virt = VirtAddr::new(address);
//        // let phys = unsafe { translate_addr(virt, phys_mem_offset) };
//        let phys = mapper.translate_addr(virt);
//        println!("{:?} -> {:?}", virt, phys);
//    }

    println!("I am speed!");
    //loop {
    //    use blog_os::print;
    //    print!("-");        // new
    //}
    blog_os::hlt_loop();
}


#[test_case]
fn trivial_assertion() {
    // serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    // serial_println!("[ok]");
}

