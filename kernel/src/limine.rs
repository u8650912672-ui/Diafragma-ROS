//telling limine to give me what i want or die a cruel death
use core::ptr::{self, addr_of_mut};

pub const COMMON0: u64 = 0xc7b1dd30df4c8b88; //could prolly be generated at random :sob:
pub const COMMON1: u64 = 0x0a82e883a194f07b;

#[used]
#[link_section = ".limine_requests_start"]
pub static REQ_START: [u64; 4] = [
    0xf6b8f4b39de7d1ae,
    0xfab91a6940fcb9cf,
    0x785c6ed015d3e316,
    0x181e920a7852b9d9,
];

#[used]
#[link_section = ".limine_requests"]
pub static BASE_REV: [u64; 3] = [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc, 3];

#[used]
#[link_section = ".limine_requests_end"]
pub static REQ_END: [u64; 2] = [0xadc0e0531bb10d03, 0x9572709f31764c62];
//sorry if i still cant write semi clean code im trying my best :c
#[repr(C)]
pub struct Fb {
    pub addr: *mut u8, //im sorry for this dogshit code okay :c
    pub w: u64,
    pub h: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub mm: u8,
    pub rms: u8,
    pub rsh: u8,
    pub gms: u8,
    pub gsh: u8,
    pub bms: u8,
    pub bsh: u8,
    pub _u: [u8; 7],
    pub edid_sz: u64,
    pub edid: *mut u8,
    pub mode_count: u64,
    pub modes: *mut *mut u8, //what must be done has to be done im sorry :sob:
}

#[repr(C)]
pub struct FbResp {
    pub rev: u64,
    pub count: u64,
    pub fbs: *mut *mut Fb,
}

#[repr(C)]
pub struct FbReq {
    pub id: [u64; 4],
    pub rev: u64,
    pub resp: *mut FbResp,
}

unsafe impl Sync for FbReq {}

#[used]
#[link_section = ".limine_requests"]
pub static mut FB_REQ: FbReq = FbReq {
    id: [COMMON0, COMMON1, 0x9d5827dcd881dd75, 0xa3148604f6fab11b], //where the fuck did i get the 2 last ones well its limine request id's :3
    rev: 0, //shiiii car not even idle :c
    resp: ptr::null_mut(),
};

pub fn fb() -> Option<&'static mut Fb> {
    unsafe {
        let resp = (*addr_of_mut!(FB_REQ)).resp as *mut FbResp;
        let r = resp.as_ref()?; // if the bootloader (limine) dont give me a fb uhhhhh FUCK
        if r.count == 0 {
            return None;
        }
        Some(&mut **r.fbs)
    }
}
