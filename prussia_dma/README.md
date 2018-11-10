# `prussia_dma` - Direct Memory Access management routines.

This crate contains routines to manage DMA transfers in a safe way.

# Aliasing

It is *very easy* to introduce memory aliasing. For example:

```rust
use prussia_dma as dma;
fn bad_idea(ipu_from: dma::IpuFrom, ipu_to: dma::IpuTo) -> (dma::IpuFrom, dma::IpuTo) {
    static mut BUFFER: Aligned<A16, [u32; 4]> = Aligned([0; 4]);

    let dma_from = dma::Transfer::from_mem(ipu_to, unsafe { &mut BUFFER }); // Safe.
    let dma_to = dma::Transfer::to_mem(ipu_from, unsafe { &mut BUFFER }); // UNSAFE; introduces memory aliasing.

    let (ipu_to, _) = dma_from.wait();
    let (ipu_from, _) = dma_to.wait();

    (ipu_from, ipu_to)
}
```

While this problem is fixed easily enough:
```rust
use prussia_dma as dma;
fn better(ipu_from: dma::IpuFrom, ipu_to: dma::IpuTo) -> (dma::IpuFrom, dma::IpuTo) {
    static mut BUFFER: Aligned<A16, [u32; 4]> = Aligned([0; 4]);

    let dma_from = dma::Transfer::from_mem(ipu_to, unsafe { &mut BUFFER }); // Safe.
    let (ipu_to, _) = dma_from.wait();
    
    let dma_to = dma::Transfer::to_mem(ipu_from, unsafe { &mut BUFFER }); // UNSAFE; introduces memory aliasing.
    let (ipu_from, _) = dma_to.wait();

    (ipu_from, ipu_to)
}
```
I don't yet have anything like `cortex_m::singleton!()`, so consider using a `RefCell`, or one of the `spin` crate `Mutex`es.

## TODO

This seems reasonably complete at the moment, aside from a `singleton!` equivalent, which might better belong in `prussia_rt`..

