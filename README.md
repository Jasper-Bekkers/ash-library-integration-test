# ash-library-integration-test

This is a test bed for doing library integrations with ash.

Right now both if you're using or building a utility library for vulkan and you choose to rely on ash you need to make sure that both the application using the library and the actual library are on more or less the same version of ash.

It also means that all downstream helper libraries (such as the popular `vk-mem-rs`) need to be updated as soon as possible when `ash` bumps it's version.

This repository (and the associated ones in https://github.com/Jasper-Bekkers/vk-mem-rs and https://github.com/Jasper-Bekkers/ash) change the ash API in such a way that one only relies on raw vulkan handles to share between projects and a such should be a lot less prone to these kinds of issues.

That way a utility library such as `vk-mem-rs` could stay on whichever version of `ash` it prefers.

The changes on the `ash` side are relatively minor. I've just added two functions for creating devices and instances from raw handles, while still being able to use & rely on the `ash` function loader.

On the `vk-mem-rs` the core changes are also relatively small (just use those functions), but I've also gone ahead and removed all `ash` related data types from it's public interface. This means that calling code will need to be updated to use the data types it's generated in the `vk_mem_rs::ffi` module instead of the `ash` types. However, since most types are pretty basic one can easily `std::mem::transmute` between them (similar to what `vk_mem_rs` used to do internally).
