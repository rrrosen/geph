#![allow(warnings, dead_code)]
/* automatically generated by rust-bindgen 0.56.0 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const WINDIVERT_FLAG_SNIFF: u32 = 1;
pub const WINDIVERT_FLAG_RECV_ONLY: u32 = 4;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type UINT = ::std::os::raw::c_uint;
pub type INT16 = ::std::os::raw::c_short;
pub type INT64 = ::std::os::raw::c_longlong;
pub type UINT8 = ::std::os::raw::c_uchar;
pub type UINT16 = ::std::os::raw::c_ushort;
pub type UINT32 = ::std::os::raw::c_uint;
pub type UINT64 = ::std::os::raw::c_ulonglong;
pub type ULONG_PTR = ::std::os::raw::c_ulonglong;
pub type PVOID = *mut ::std::os::raw::c_void;
pub type HANDLE = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _OVERLAPPED {
    pub Internal: ULONG_PTR,
    pub InternalHigh: ULONG_PTR,
    pub __bindgen_anon_1: _OVERLAPPED__bindgen_ty_1,
    pub hEvent: HANDLE,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _OVERLAPPED__bindgen_ty_1 {
    pub __bindgen_anon_1: _OVERLAPPED__bindgen_ty_1__bindgen_ty_1,
    pub Pointer: PVOID,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _OVERLAPPED__bindgen_ty_1__bindgen_ty_1 {
    pub Offset: DWORD,
    pub OffsetHigh: DWORD,
}
#[test]
fn bindgen_test_layout__OVERLAPPED__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_OVERLAPPED__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_OVERLAPPED__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_OVERLAPPED__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_OVERLAPPED__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_OVERLAPPED__bindgen_ty_1__bindgen_ty_1>())).Offset as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Offset)
        )
    );

    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_OVERLAPPED__bindgen_ty_1__bindgen_ty_1>())).OffsetHigh
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(OffsetHigh)
        )
    );
}
#[test]
fn bindgen_test_layout__OVERLAPPED__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_OVERLAPPED__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(_OVERLAPPED__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_OVERLAPPED__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_OVERLAPPED__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_OVERLAPPED__bindgen_ty_1>())).Pointer as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED__bindgen_ty_1),
            "::",
            stringify!(Pointer)
        )
    );
}
#[test]
fn bindgen_test_layout__OVERLAPPED() {
    assert_eq!(
        ::std::mem::size_of::<_OVERLAPPED>(),
        32usize,
        concat!("Size of: ", stringify!(_OVERLAPPED))
    );
    assert_eq!(
        ::std::mem::align_of::<_OVERLAPPED>(),
        8usize,
        concat!("Alignment of ", stringify!(_OVERLAPPED))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_OVERLAPPED>())).Internal as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED),
            "::",
            stringify!(Internal)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_OVERLAPPED>())).InternalHigh as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED),
            "::",
            stringify!(InternalHigh)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_OVERLAPPED>())).hEvent as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_OVERLAPPED),
            "::",
            stringify!(hEvent)
        )
    );
}
pub type LPOVERLAPPED = *mut _OVERLAPPED;
extern "C" {
    pub fn GetLastError() -> DWORD;
}
pub const WINDIVERT_LAYER_WINDIVERT_LAYER_NETWORK: WINDIVERT_LAYER = 0;
pub const WINDIVERT_LAYER_WINDIVERT_LAYER_NETWORK_FORWARD: WINDIVERT_LAYER = 1;
pub const WINDIVERT_LAYER_WINDIVERT_LAYER_FLOW: WINDIVERT_LAYER = 2;
pub const WINDIVERT_LAYER_WINDIVERT_LAYER_SOCKET: WINDIVERT_LAYER = 3;
pub const WINDIVERT_LAYER_WINDIVERT_LAYER_REFLECT: WINDIVERT_LAYER = 4;
pub type WINDIVERT_LAYER = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WINDIVERT_DATA_NETWORK {
    pub IfIdx: UINT32,
    pub SubIfIdx: UINT32,
}
#[test]
fn bindgen_test_layout_WINDIVERT_DATA_NETWORK() {
    assert_eq!(
        ::std::mem::size_of::<WINDIVERT_DATA_NETWORK>(),
        8usize,
        concat!("Size of: ", stringify!(WINDIVERT_DATA_NETWORK))
    );
    assert_eq!(
        ::std::mem::align_of::<WINDIVERT_DATA_NETWORK>(),
        4usize,
        concat!("Alignment of ", stringify!(WINDIVERT_DATA_NETWORK))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_NETWORK>())).IfIdx as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_NETWORK),
            "::",
            stringify!(IfIdx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_NETWORK>())).SubIfIdx as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_NETWORK),
            "::",
            stringify!(SubIfIdx)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WINDIVERT_DATA_FLOW {
    pub EndpointId: UINT64,
    pub ParentEndpointId: UINT64,
    pub ProcessId: UINT32,
    pub LocalAddr: [UINT32; 4usize],
    pub RemoteAddr: [UINT32; 4usize],
    pub LocalPort: UINT16,
    pub RemotePort: UINT16,
    pub Protocol: UINT8,
}
#[test]
fn bindgen_test_layout_WINDIVERT_DATA_FLOW() {
    assert_eq!(
        ::std::mem::size_of::<WINDIVERT_DATA_FLOW>(),
        64usize,
        concat!("Size of: ", stringify!(WINDIVERT_DATA_FLOW))
    );
    assert_eq!(
        ::std::mem::align_of::<WINDIVERT_DATA_FLOW>(),
        8usize,
        concat!("Alignment of ", stringify!(WINDIVERT_DATA_FLOW))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_FLOW>())).EndpointId as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_FLOW),
            "::",
            stringify!(EndpointId)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_DATA_FLOW>())).ParentEndpointId as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_FLOW),
            "::",
            stringify!(ParentEndpointId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_FLOW>())).ProcessId as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_FLOW),
            "::",
            stringify!(ProcessId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_FLOW>())).LocalAddr as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_FLOW),
            "::",
            stringify!(LocalAddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_FLOW>())).RemoteAddr as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_FLOW),
            "::",
            stringify!(RemoteAddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_FLOW>())).LocalPort as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_FLOW),
            "::",
            stringify!(LocalPort)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_FLOW>())).RemotePort as *const _ as usize },
        54usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_FLOW),
            "::",
            stringify!(RemotePort)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_FLOW>())).Protocol as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_FLOW),
            "::",
            stringify!(Protocol)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WINDIVERT_DATA_SOCKET {
    pub EndpointId: UINT64,
    pub ParentEndpointId: UINT64,
    pub ProcessId: UINT32,
    pub LocalAddr: [UINT32; 4usize],
    pub RemoteAddr: [UINT32; 4usize],
    pub LocalPort: UINT16,
    pub RemotePort: UINT16,
    pub Protocol: UINT8,
}
#[test]
fn bindgen_test_layout_WINDIVERT_DATA_SOCKET() {
    assert_eq!(
        ::std::mem::size_of::<WINDIVERT_DATA_SOCKET>(),
        64usize,
        concat!("Size of: ", stringify!(WINDIVERT_DATA_SOCKET))
    );
    assert_eq!(
        ::std::mem::align_of::<WINDIVERT_DATA_SOCKET>(),
        8usize,
        concat!("Alignment of ", stringify!(WINDIVERT_DATA_SOCKET))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_DATA_SOCKET>())).EndpointId as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_SOCKET),
            "::",
            stringify!(EndpointId)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_DATA_SOCKET>())).ParentEndpointId as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_SOCKET),
            "::",
            stringify!(ParentEndpointId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_SOCKET>())).ProcessId as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_SOCKET),
            "::",
            stringify!(ProcessId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_SOCKET>())).LocalAddr as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_SOCKET),
            "::",
            stringify!(LocalAddr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_DATA_SOCKET>())).RemoteAddr as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_SOCKET),
            "::",
            stringify!(RemoteAddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_SOCKET>())).LocalPort as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_SOCKET),
            "::",
            stringify!(LocalPort)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_DATA_SOCKET>())).RemotePort as *const _ as usize
        },
        54usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_SOCKET),
            "::",
            stringify!(RemotePort)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_SOCKET>())).Protocol as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_SOCKET),
            "::",
            stringify!(Protocol)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WINDIVERT_DATA_REFLECT {
    pub Timestamp: INT64,
    pub ProcessId: UINT32,
    pub Layer: WINDIVERT_LAYER,
    pub Flags: UINT64,
    pub Priority: INT16,
}
#[test]
fn bindgen_test_layout_WINDIVERT_DATA_REFLECT() {
    assert_eq!(
        ::std::mem::size_of::<WINDIVERT_DATA_REFLECT>(),
        32usize,
        concat!("Size of: ", stringify!(WINDIVERT_DATA_REFLECT))
    );
    assert_eq!(
        ::std::mem::align_of::<WINDIVERT_DATA_REFLECT>(),
        8usize,
        concat!("Alignment of ", stringify!(WINDIVERT_DATA_REFLECT))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_DATA_REFLECT>())).Timestamp as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_REFLECT),
            "::",
            stringify!(Timestamp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_DATA_REFLECT>())).ProcessId as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_REFLECT),
            "::",
            stringify!(ProcessId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_REFLECT>())).Layer as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_REFLECT),
            "::",
            stringify!(Layer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_REFLECT>())).Flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_REFLECT),
            "::",
            stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_DATA_REFLECT>())).Priority as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_DATA_REFLECT),
            "::",
            stringify!(Priority)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WINDIVERT_ADDRESS {
    pub Timestamp: INT64,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u8>,
    pub Reserved2: UINT32,
    pub __bindgen_anon_1: WINDIVERT_ADDRESS__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WINDIVERT_ADDRESS__bindgen_ty_1 {
    pub Network: WINDIVERT_DATA_NETWORK,
    pub Flow: WINDIVERT_DATA_FLOW,
    pub Socket: WINDIVERT_DATA_SOCKET,
    pub Reflect: WINDIVERT_DATA_REFLECT,
    pub Reserved3: [UINT8; 64usize],
    _bindgen_union_align: [u64; 8usize],
}
#[test]
fn bindgen_test_layout_WINDIVERT_ADDRESS__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<WINDIVERT_ADDRESS__bindgen_ty_1>(),
        64usize,
        concat!("Size of: ", stringify!(WINDIVERT_ADDRESS__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<WINDIVERT_ADDRESS__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(WINDIVERT_ADDRESS__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_ADDRESS__bindgen_ty_1>())).Network as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_ADDRESS__bindgen_ty_1),
            "::",
            stringify!(Network)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_ADDRESS__bindgen_ty_1>())).Flow as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_ADDRESS__bindgen_ty_1),
            "::",
            stringify!(Flow)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_ADDRESS__bindgen_ty_1>())).Socket as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_ADDRESS__bindgen_ty_1),
            "::",
            stringify!(Socket)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_ADDRESS__bindgen_ty_1>())).Reflect as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_ADDRESS__bindgen_ty_1),
            "::",
            stringify!(Reflect)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WINDIVERT_ADDRESS__bindgen_ty_1>())).Reserved3 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_ADDRESS__bindgen_ty_1),
            "::",
            stringify!(Reserved3)
        )
    );
}
#[test]
fn bindgen_test_layout_WINDIVERT_ADDRESS() {
    assert_eq!(
        ::std::mem::size_of::<WINDIVERT_ADDRESS>(),
        80usize,
        concat!("Size of: ", stringify!(WINDIVERT_ADDRESS))
    );
    assert_eq!(
        ::std::mem::align_of::<WINDIVERT_ADDRESS>(),
        8usize,
        concat!("Alignment of ", stringify!(WINDIVERT_ADDRESS))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_ADDRESS>())).Timestamp as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_ADDRESS),
            "::",
            stringify!(Timestamp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WINDIVERT_ADDRESS>())).Reserved2 as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(WINDIVERT_ADDRESS),
            "::",
            stringify!(Reserved2)
        )
    );
}
impl WINDIVERT_ADDRESS {
    #[inline]
    pub fn Layer(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_Layer(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn Event(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_Event(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn Sniffed(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_Sniffed(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn Outbound(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(17usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_Outbound(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(17usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn Loopback(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(18usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_Loopback(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(18usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn Impostor(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(19usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_Impostor(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(19usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IPv6(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(20usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_IPv6(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(20usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IPChecksum(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(21usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_IPChecksum(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(21usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn TCPChecksum(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(22usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_TCPChecksum(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(22usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn UDPChecksum(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(23usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_UDPChecksum(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(23usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn Reserved1(&self) -> UINT32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_Reserved1(&mut self, val: UINT32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        Layer: UINT32,
        Event: UINT32,
        Sniffed: UINT32,
        Outbound: UINT32,
        Loopback: UINT32,
        Impostor: UINT32,
        IPv6: UINT32,
        IPChecksum: UINT32,
        TCPChecksum: UINT32,
        UDPChecksum: UINT32,
        Reserved1: UINT32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let Layer: u32 = unsafe { ::std::mem::transmute(Layer) };
            Layer as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let Event: u32 = unsafe { ::std::mem::transmute(Event) };
            Event as u64
        });
        __bindgen_bitfield_unit.set(16usize, 1u8, {
            let Sniffed: u32 = unsafe { ::std::mem::transmute(Sniffed) };
            Sniffed as u64
        });
        __bindgen_bitfield_unit.set(17usize, 1u8, {
            let Outbound: u32 = unsafe { ::std::mem::transmute(Outbound) };
            Outbound as u64
        });
        __bindgen_bitfield_unit.set(18usize, 1u8, {
            let Loopback: u32 = unsafe { ::std::mem::transmute(Loopback) };
            Loopback as u64
        });
        __bindgen_bitfield_unit.set(19usize, 1u8, {
            let Impostor: u32 = unsafe { ::std::mem::transmute(Impostor) };
            Impostor as u64
        });
        __bindgen_bitfield_unit.set(20usize, 1u8, {
            let IPv6: u32 = unsafe { ::std::mem::transmute(IPv6) };
            IPv6 as u64
        });
        __bindgen_bitfield_unit.set(21usize, 1u8, {
            let IPChecksum: u32 = unsafe { ::std::mem::transmute(IPChecksum) };
            IPChecksum as u64
        });
        __bindgen_bitfield_unit.set(22usize, 1u8, {
            let TCPChecksum: u32 = unsafe { ::std::mem::transmute(TCPChecksum) };
            TCPChecksum as u64
        });
        __bindgen_bitfield_unit.set(23usize, 1u8, {
            let UDPChecksum: u32 = unsafe { ::std::mem::transmute(UDPChecksum) };
            UDPChecksum as u64
        });
        __bindgen_bitfield_unit.set(24usize, 8u8, {
            let Reserved1: u32 = unsafe { ::std::mem::transmute(Reserved1) };
            Reserved1 as u64
        });
        __bindgen_bitfield_unit
    }
}
pub const WINDIVERT_EVENT_WINDIVERT_EVENT_NETWORK_PACKET: WINDIVERT_EVENT = 0;
pub const WINDIVERT_EVENT_WINDIVERT_EVENT_FLOW_ESTABLISHED: WINDIVERT_EVENT = 1;
pub const WINDIVERT_EVENT_WINDIVERT_EVENT_FLOW_DELETED: WINDIVERT_EVENT = 2;
pub const WINDIVERT_EVENT_WINDIVERT_EVENT_SOCKET_BIND: WINDIVERT_EVENT = 3;
pub const WINDIVERT_EVENT_WINDIVERT_EVENT_SOCKET_CONNECT: WINDIVERT_EVENT = 4;
pub const WINDIVERT_EVENT_WINDIVERT_EVENT_SOCKET_LISTEN: WINDIVERT_EVENT = 5;
pub const WINDIVERT_EVENT_WINDIVERT_EVENT_SOCKET_ACCEPT: WINDIVERT_EVENT = 6;
pub const WINDIVERT_EVENT_WINDIVERT_EVENT_SOCKET_CLOSE: WINDIVERT_EVENT = 7;
pub const WINDIVERT_EVENT_WINDIVERT_EVENT_REFLECT_OPEN: WINDIVERT_EVENT = 8;
pub const WINDIVERT_EVENT_WINDIVERT_EVENT_REFLECT_CLOSE: WINDIVERT_EVENT = 9;
pub type WINDIVERT_EVENT = ::std::os::raw::c_int;
extern "C" {
    pub fn WinDivertOpen(
        filter: *const ::std::os::raw::c_char,
        layer: WINDIVERT_LAYER,
        priority: INT16,
        flags: UINT64,
    ) -> HANDLE;
}
extern "C" {
    pub fn WinDivertRecv(
        handle: HANDLE,
        pPacket: *mut ::std::os::raw::c_void,
        packetLen: UINT,
        pRecvLen: *mut UINT,
        pAddr: *mut WINDIVERT_ADDRESS,
    ) -> BOOL;
}
extern "C" {
    pub fn WinDivertRecvEx(
        handle: HANDLE,
        pPacket: *mut ::std::os::raw::c_void,
        packetLen: UINT,
        pRecvLen: *mut UINT,
        flags: UINT64,
        pAddr: *mut WINDIVERT_ADDRESS,
        pAddrLen: *mut UINT,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
}
extern "C" {
    pub fn WinDivertSend(
        handle: HANDLE,
        pPacket: *const ::std::os::raw::c_void,
        packetLen: UINT,
        pSendLen: *mut UINT,
        pAddr: *const WINDIVERT_ADDRESS,
    ) -> BOOL;
}
extern "C" {
    pub fn WinDivertSendEx(
        handle: HANDLE,
        pPacket: *const ::std::os::raw::c_void,
        packetLen: UINT,
        pSendLen: *mut UINT,
        flags: UINT64,
        pAddr: *const WINDIVERT_ADDRESS,
        addrLen: UINT,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
}
extern "C" {
    pub fn WinDivertClose(handle: HANDLE) -> BOOL;
}
