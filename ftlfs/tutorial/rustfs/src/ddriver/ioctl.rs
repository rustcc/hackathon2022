#![allow(non_snake_case)]

const _IOC_NRBITS: u32 = 8;
const _IOC_TYPEBITS: u32 = 8;
const _IOC_SIZEBITS: u32 = 14;
const _IOC_DIRBITS: u32 = 2;

const _IOC_NRMASK: u32 = (1 << _IOC_NRBITS) - 1;
const _IOC_TYPEMASK: u32 = (1 << _IOC_TYPEBITS) - 1;
const _IOC_SIZEMASK: u32 = (1 << _IOC_SIZEBITS) - 1;
const _IOC_DIRMASK: u32 = (1 << _IOC_DIRBITS) - 1;

const _IOC_NRSHIFT: u32 = 0;
const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;

const _IOC_NONE: u32 = 0;
const _IOC_WRITE: u32 = 1;
const _IOC_READ: u32 = 2;

const fn _IOC(dir: u32, typ: u32, nr: u32, size: u32) -> u32 {
    dir << _IOC_DIRSHIFT | typ << _IOC_TYPESHIFT | nr << _IOC_NRSHIFT | (size) << _IOC_SIZESHIFT
}

const fn _IOC_TYPECHECK<T>() -> u32 {
    std::mem::size_of::<T>() as u32
}

pub const fn _IO(typ: u32, nr: u32) -> u32 {
    _IOC(_IOC_NONE, typ, nr, 0)
}

pub const fn _IOR<T>(typ: u32, nr: u32) -> u32 {
    _IOC(_IOC_READ, typ, nr, _IOC_TYPECHECK::<T>())
}
